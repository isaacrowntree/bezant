//! Session keepalive + health helpers layered over the generated client.

use std::time::Duration;

use tokio::sync::oneshot;

use crate::client::Client;
use crate::error::{Error, Result};

/// Simplified view of the Gateway's brokerage session status, projected from
/// the generated [`bezant_api::BrokerageSessionStatus`] type.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
            .map_err(|()| Error::other("base url cannot be a base"))?
            .push("iserver")
            .push("auth")
            .push("status");
        url.set_query(None);
        let resp = self
            .http()
            .post(url)
            .body(Vec::<u8>::new())
            .send()
            .await
            .map_err(Error::Http)?;
        let status = resp.status();
        if status.is_redirection() || status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::NotAuthenticated);
        }
        if !status.is_success() {
            return Err(Error::other(format!(
                "auth_status returned {status}"
            )));
        }
        let parsed: bezant_api::BrokerageSessionStatus =
            resp.json().await.map_err(Error::Http)?;
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
            bezant_api::GetSessionTokenResponse::Unknown => {
                Err(Error::other("unknown tickle response"))
            }
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
        let client = self.clone();
        let (tx, mut rx) = oneshot::channel::<()>();
        let join = tokio::spawn(async move {
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
        });
        KeepaliveHandle {
            shutdown: Some(tx),
            join: Some(join),
        }
    }
}

/// Drop-to-stop handle for a background keepalive task.
///
/// Dropping the handle closes the shutdown channel; the background task
/// exits on its next tick. Call [`KeepaliveHandle::stop`] instead if you
/// want to await a clean exit.
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
        drop(self.shutdown.take());
        if let Some(join) = self.join.take() {
            join.await
                .map_err(|e| Error::other(format!("keepalive task panicked: {e}")))?;
        }
        Ok(())
    }
}
