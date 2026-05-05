//! Shared application state.

use std::sync::Arc;

use crate::events::EventsHandle;

/// State shared across all axum handlers.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    client: bezant::Client,
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
