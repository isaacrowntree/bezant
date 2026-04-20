//! Shared application state.

use std::sync::Arc;

/// State shared across all axum handlers.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    client: bezant::Client,
}

impl AppState {
    /// Build app state from a configured [`bezant::Client`].
    #[must_use]
    pub fn new(client: bezant::Client) -> Self {
        Self {
            inner: Arc::new(Inner { client }),
        }
    }

    /// Borrow the underlying Bezant client.
    #[must_use]
    pub fn client(&self) -> &bezant::Client {
        &self.inner.client
    }
}
