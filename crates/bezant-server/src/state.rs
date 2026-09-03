//! Shared application state.

use std::sync::Arc;

use crate::events::EventsHandle;

/// State shared across all axum handlers.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

/// Last thing the SSO bridge (`iserver/auth/ssodh/init`) said, observed as
/// those calls pass through the proxy.
///
/// Why this is worth keeping: the Gateway can be ALIVE AND WRONG. On
/// 2026-09-03 that endpoint returned HTTP 500 for eleven hours while
/// `/health`, `auth/status` and the process table all looked perfect —
/// meaning no login could complete, and the only symptom anyone could see was
/// 2FA "not working". A restart cleared it and the same call answered 401.
///
/// Recorded PASSIVELY rather than probed on a timer: the relogin service and
/// the watchdog already POST this endpoint through us, so watching what goes
/// by costs nothing and cannot add load to a Gateway that is already unwell.
#[derive(Debug, Clone, Copy)]
pub struct SsoBridge {
    /// HTTP status of the most recent `ssodh/init` seen.
    pub status: u16,
    /// Unix seconds when it was seen.
    pub at: u64,
    /// Consecutive 5xx. 401 is healthy-but-logged-out and resets this.
    pub consecutive_faults: u32,
}

struct Inner {
    client: bezant::Client,
    /// See [`SsoBridge`]. `None` until a bridge call has been observed.
    sso_bridge: std::sync::Mutex<Option<SsoBridge>>,
    /// Optional token guarding the `/debug/*` endpoints. When `None`,
    /// debug endpoints return 404. When `Some`, callers must present a
    /// matching token via `?token=…` query string or
    /// `X-Bezant-Debug-Token` header.
    debug_token: Option<String>,
    /// Handle to the optional events-capture connector. `None` disables
    /// the `/events/*` routes (they return 503).
    events: Option<EventsHandle>,
}

impl AppState {
    /// Build app state from a configured [`bezant::Client`].
    ///
    /// Debug endpoints are disabled by default. Use
    /// [`AppState::with_debug_token`] to enable them with token gating.
    /// Events are disabled by default. Use [`AppState::with_events`] to
    /// attach a connector handle.
    #[must_use]
    pub fn new(client: bezant::Client) -> Self {
        Self {
            inner: Arc::new(Inner {
                client,
                sso_bridge: std::sync::Mutex::new(None),
                debug_token: None,
                events: None,
            }),
        }
    }

    /// Enable the `/debug/*` endpoints, requiring the given token on
    /// every request (via `?token=…` or `X-Bezant-Debug-Token` header).
    /// Without this, all `/debug/*` routes 404.
    ///
    /// **Security:** the cookie jar holds live IBKR session cookies
    /// — anyone who can read it can resume the IBKR session and
    /// trade the account. Pick a long, random token (>=32 bytes
    /// from `/dev/urandom`) and treat it like a credential.
    #[must_use]
    pub fn with_debug_token(client: bezant::Client, token: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(Inner {
                client,
                sso_bridge: std::sync::Mutex::new(None),
                debug_token: Some(token.into()),
                events: None,
            }),
        }
    }

    /// Return a new state with the given events handle attached. The
    /// handle is what powers `/events/*` reads. Without it, those routes
    /// return 503.
    #[must_use]
    pub fn with_events(self, events: EventsHandle) -> Self {
        let inner = Inner {
            client: self.inner.client.clone(),
            // Carry the observation across, rather than resetting it: this is
            // called during startup wiring and a fault seen before the events
            // handle attaches is still a fault.
            sso_bridge: std::sync::Mutex::new(*self.inner.sso_bridge.lock().unwrap_or_else(|e| e.into_inner())),
            debug_token: self.inner.debug_token.clone(),
            events: Some(events),
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Borrow the underlying Bezant client.
    #[must_use]
    pub fn client(&self) -> &bezant::Client {
        &self.inner.client
    }

    /// Record what the SSO bridge just answered. Called from the proxy as
    /// `ssodh/init` responses pass through — see [`SsoBridge`].
    ///
    /// A poisoned lock is recovered from rather than propagated: this is
    /// observability, and it must never be the reason a proxied trading call
    /// fails.
    pub fn record_sso_bridge(&self, status: u16) {
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let mut slot = self
            .inner
            .sso_bridge
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let previous_faults = slot.map_or(0, |b: SsoBridge| b.consecutive_faults);
        *slot = Some(SsoBridge {
            status,
            at,
            consecutive_faults: if status >= 500 {
                previous_faults.saturating_add(1)
            } else {
                0
            },
        });
    }

    /// The last observed SSO bridge state, if any call has been seen.
    #[must_use]
    pub fn sso_bridge(&self) -> Option<SsoBridge> {
        *self
            .inner
            .sso_bridge
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    /// Borrow the configured debug token, if any.
    #[must_use]
    pub fn debug_token(&self) -> Option<&str> {
        self.inner.debug_token.as_deref()
    }

    /// Borrow the events handle, if attached.
    #[must_use]
    pub fn events(&self) -> Option<&EventsHandle> {
        self.inner.events.as_ref()
    }
}
