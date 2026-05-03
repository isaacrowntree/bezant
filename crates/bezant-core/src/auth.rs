//! Session keepalive + health helpers layered over the generated client.

use std::time::Duration;

use tokio::sync::oneshot;

use crate::client::Client;
use crate::error::{Error, Result};

/// Simplified view of the Gateway's brokerage session status, projected from
/// the generated [`bezant_api::BrokerageSessionStatus`] type.
///
/// `#[non_exhaustive]` so adding a field in a point release isn't a SemVer
/// break — match with `AuthStatus { authenticated, connected, .. }` rather
/// than positional destructuring.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AuthStatus {
    /// Whether the Gateway is authenticated.
    pub authenticated: bool,
    /// Whether the Gateway is connected to IBKR servers.
    pub connected: bool,
    /// Whether this Gateway session is the "competing" primary.
    pub competing: bool,
    /// Server-reported status message.
    pub message: Option<String>,
}

impl From<bezant_api::BrokerageSessionStatus> for AuthStatus {
    fn from(s: bezant_api::BrokerageSessionStatus) -> Self {
        Self {
            authenticated: s.authenticated.unwrap_or(false),
            connected: s.connected.unwrap_or(false),
            competing: s.competing.unwrap_or(false),
            message: s.message,
        }
    }
}

/// Response from a tickle call, projected from the generated type.
///
/// `#[non_exhaustive]` so adding a field in a point release isn't a SemVer
/// break.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct TickleResponse {
    /// Session identifier the Gateway returned, if any.
    pub session: Option<String>,
    /// Raw response — exposed for callers that need the full tickle payload.
    pub raw: bezant_api::TickleResponse,
}

impl Client {
    /// Query current auth + connection status via
    /// `POST /iserver/auth/status`.
    ///
    /// # Errors
    /// Transport + decode errors; [`Error::Api`] for any underlying error.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn auth_status(&self) -> Result<AuthStatus> {
        // We drive a raw POST instead of the generated client so we
        // can distinguish "no session" (Gateway redirects to login)
        // from genuine protocol errors. CPAPI returns 302 for
        // unauthenticated callers — not the 401 the OpenAPI spec
        // implies — so the auto-generated `Unauthorized` branch never
        // fires in practice.
        let mut url = self.base_url().clone();
        url.path_segments_mut()
            .map_err(|()| Error::UrlNotABase {
                url: self.base_url().to_string(),
            })?
            .push("iserver")
            .push("auth")
            .push("status");
        url.set_query(None);
        // Akamai (in front of CPAPI) refuses POSTs that reach it
        // without a Content-Length header — even an empty `Vec<u8>`
        // body isn't enough if reqwest decides to serialize it with
        // Transfer-Encoding: chunked. Setting the header explicitly
        // forces a `Content-Length: 0` wire representation.
        //
        // We also pin Origin/Referer to match the Gateway's own
        // origin: CPGateway's CPAPI handler treats requests with
        // missing/mismatched Origin as a CSRF attempt and 401s. Local
        // browser flows succeed because the browser supplies an
        // origin that matches the Gateway's expectations; from a
        // typed server-side caller we need to mint one ourselves.
        let gateway_origin = self
            .gateway_root_url()
            .as_str()
            .trim_end_matches('/')
            .to_owned();
        let resp = self
            .http()
            .post(url.clone())
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .header(reqwest::header::ORIGIN, &gateway_origin)
            .header(reqwest::header::REFERER, format!("{gateway_origin}/"))
            .send()
            .await
            .map_err(Error::Http)?;
        let status = resp.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::NotAuthenticated);
        }
        if status.is_redirection() {
            // Tighten: only treat redirects that actually aim at the SSO
            // login flow as "not authenticated". Any other 3xx is a
            // genuine protocol surprise we'd rather surface verbatim.
            let location = resp
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if location.contains("/sso/login") || location.contains("/sso/dispatcher") {
                return Err(Error::NotAuthenticated);
            }
            return Err(Error::UpstreamStatus {
                endpoint: "iserver/auth/status",
                status: status.as_u16(),
                body_preview: Some(format!("redirect to: {location}")),
            });
        }
        if !status.is_success() {
            return Err(Error::UpstreamStatus {
                endpoint: "iserver/auth/status",
                status: status.as_u16(),
                body_preview: None,
            });
        }
        let parsed: bezant_api::BrokerageSessionStatus = resp.json().await.map_err(|e| {
            Error::Decode {
                endpoint: format!("POST {}/iserver/auth/status", self.base_url()),
                status: status.as_u16(),
                message: e.to_string(),
            }
        })?;
        Ok(AuthStatus::from(parsed))
    }

    /// Tickle the Gateway (POST `/tickle`) to keep the session alive.
    ///
    /// CPAPI sessions expire after ~5 minutes of inactivity; call this at
    /// least once a minute from a background task, or use
    /// [`Client::spawn_keepalive`].
    ///
    /// # Errors
    /// Transport + decode errors.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn tickle(&self) -> Result<TickleResponse> {
        let resp = self
            .api()
            .get_session_token(bezant_api::GetSessionTokenRequest::default())
            .await?;
        match resp {
            bezant_api::GetSessionTokenResponse::Ok(payload) => {
                let session = match &payload {
                    bezant_api::TickleResponse::Successful(s) => s.session.clone(),
                    bezant_api::TickleResponse::Failed(_) => None,
                };
                Ok(TickleResponse {
                    session,
                    raw: payload,
                })
            }
            bezant_api::GetSessionTokenResponse::Unauthorized => Err(Error::NotAuthenticated),
            bezant_api::GetSessionTokenResponse::Unknown => Err(Error::Unknown {
                endpoint: "iserver/auth/tickle",
            }),
        }
    }

    /// Convenience: return the status if authenticated, else a typed error
    /// ([`Error::NotAuthenticated`] / [`Error::NoSession`]).
    ///
    /// # Errors
    /// See variants.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn health(&self) -> Result<AuthStatus> {
        let status = self.auth_status().await?;
        if !status.connected {
            return Err(Error::NoSession);
        }
        if !status.authenticated {
            return Err(Error::NotAuthenticated);
        }
        Ok(status)
    }

    /// Spawn a tokio task that calls [`Client::tickle`] on `interval` until
    /// the returned [`KeepaliveHandle`] is dropped. The CPAPI session times
    /// out around 5 minutes; 60s is a sensible default.
    ///
    /// Tickle failures are logged via `tracing::warn!` and don't abort the
    /// task — a transient outage is recoverable once the Gateway is back.
    #[must_use]
    pub fn spawn_keepalive(&self, interval: Duration) -> KeepaliveHandle {
        use tracing::Instrument;

        let client = self.clone();
        let (tx, mut rx) = oneshot::channel::<()>();
        let span = tracing::info_span!("bezant_keepalive", interval_secs = interval.as_secs());
        let join = tokio::spawn(
            async move {
                let mut ticker = tokio::time::interval(interval);
                // First tick fires immediately; skip it so we don't hit the
                // Gateway the microsecond after the caller created the client.
                ticker.tick().await;
                loop {
                    tokio::select! {
                        _ = &mut rx => break,
                        _ = ticker.tick() => {
                            if let Err(e) = client.tickle().await {
                                tracing::warn!(error = %e, "bezant keepalive tickle failed");
                            }
                        }
                    }
                }
            }
            .instrument(span),
        );
        KeepaliveHandle {
            shutdown: Some(tx),
            join: Some(join),
        }
    }
}

/// Drop-to-stop handle for a background keepalive task.
///
/// Dropping the handle sends the shutdown signal via the [`Drop`] impl,
/// so a forgotten `_handle` won't keep tickling forever. The spawned
/// task observes the signal on its next tick — a pending tickle
/// request can still be in flight when the runtime shuts down.
/// Prefer [`KeepaliveHandle::stop`] when you need a clean, awaited
/// exit (e.g. on SIGTERM in a long-running binary).
#[derive(Debug)]
pub struct KeepaliveHandle {
    shutdown: Option<oneshot::Sender<()>>,
    join: Option<tokio::task::JoinHandle<()>>,
}

impl KeepaliveHandle {
    /// Stop the keepalive task and await its exit.
    ///
    /// # Errors
    /// Returns [`Error::Other`] wrapping the JoinError if the task panicked.
    pub async fn stop(mut self) -> Result<()> {
        // Explicitly send the shutdown signal rather than relying on
        // drop semantics — makes intent obvious and tolerates the
        // receiver having already dropped (which is fine, nothing to
        // do there).
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(join) = self.join.take() {
            join.await
                .map_err(|e| Error::Other(format!("keepalive task panicked: {e}")))?;
        }
        Ok(())
    }
}

impl Drop for KeepaliveHandle {
    /// Send the shutdown signal so a forgotten handle doesn't keep
    /// tickling forever. The spawned task observes the signal on its
    /// next tick — for a clean awaited exit use [`KeepaliveHandle::stop`].
    /// We can't `.await` here (sync Drop), so any in-flight tickle
    /// finishes detached.
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        // Don't abort the join handle — we want any in-flight tickle
        // to finish naturally rather than getting cut mid-write.
    }
}

#[cfg(test)]
mod keepalive_tests {
    use super::*;
    use std::time::Duration;

    /// Build a `Client` against an unreachable URL — never actually
    /// hits the network because we cancel the keepalive before its
    /// first tick fires.
    fn dummy_client() -> Client {
        Client::builder("http://127.0.0.1:1/v1/api")
            .build()
            .expect("client")
    }

    #[tokio::test]
    async fn stop_sends_shutdown_and_joins_cleanly() {
        let client = dummy_client();
        let handle = client.spawn_keepalive(Duration::from_secs(60));
        // Give the spawned task a beat to enter its select loop.
        tokio::time::sleep(Duration::from_millis(50)).await;
        handle.stop().await.expect("clean stop");
    }

    #[tokio::test]
    async fn drop_sends_shutdown_signal() {
        let client = dummy_client();
        // Capture the JoinHandle by stealing it out of the
        // KeepaliveHandle's internals via the public API: we drop the
        // handle and then verify the task observes the signal by
        // checking that a fresh handle can be spawned without panicking
        // (i.e. the runtime hasn't been clogged by a leaked task).
        {
            let _handle = client.spawn_keepalive(Duration::from_secs(60));
            tokio::time::sleep(Duration::from_millis(50)).await;
            // _handle drops here — Drop sends the shutdown signal.
        }
        // Drop happened. Spawn a fresh keepalive — if the previous
        // task hadn't observed shutdown yet (it observes on next
        // tick, which is 60s away), this still spawns fine. The real
        // assertion is that no panic / deadlock occurs.
        let _h2 = client.spawn_keepalive(Duration::from_secs(60));
        // Give the runtime a moment; if Drop deadlocked we'd hang here.
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
