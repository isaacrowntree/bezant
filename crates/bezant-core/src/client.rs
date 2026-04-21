//! Client wrapper: composes the auto-generated [`bezant_api::IbRestApiClient`]
//! with saner defaults for talking to the local IBKR Client Portal Gateway.

use std::sync::Arc;
use std::time::Duration;

use tracing::warn;

use crate::error::{Error, Result};

/// Default base URL of the Client Portal Gateway when run locally via the
/// bundled Docker image.
pub const DEFAULT_BASE_URL: &str = "https://localhost:5000/v1/api";

/// A configured client for the IBKR Client Portal Web API.
///
/// `Client` holds a [`bezant_api::IbRestApiClient`] internally. The `Arc`
/// makes it cheap to clone — share one instance across your app.
#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    api: bezant_api::IbRestApiClient,
}

impl Client {
    /// Construct a client pointed at `base_url` with Bezant's recommended
    /// defaults (accepts the Gateway's self-signed cert, 30s timeout,
    /// persistent cookie jar).
    ///
    /// # Errors
    /// Returns [`Error::InvalidBaseUrl`] if `base_url` is not a valid URL and
    /// [`Error::Http`] if reqwest fails to build its client.
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        ClientBuilder::new(base_url).build()
    }

    /// Return a builder for fine-grained configuration.
    pub fn builder(base_url: impl AsRef<str>) -> ClientBuilder {
        ClientBuilder::new(base_url)
    }

    /// Borrow the underlying generated API client for raw endpoint access.
    ///
    /// Every one of the ~155 CPAPI endpoints is available via typed methods
    /// on `bezant_api::IbRestApiClient`.
    #[must_use]
    pub fn api(&self) -> &bezant_api::IbRestApiClient {
        &self.inner.api
    }

    /// Borrow the underlying `reqwest::Client` for untyped HTTP passthrough
    /// (e.g. when you want to proxy CPAPI calls rather than decode them).
    #[must_use]
    pub fn http(&self) -> &reqwest::Client {
        &self.inner.api.client
    }

    /// Base URL the client is pointed at (e.g. `https://localhost:5000/v1/api`).
    #[must_use]
    pub fn base_url(&self) -> &url::Url {
        &self.inner.api.base_url
    }
}

/// Builder for [`Client`].
#[must_use]
pub struct ClientBuilder {
    base_url: String,
    accept_invalid_certs: bool,
    timeout: Duration,
    user_agent: String,
    follow_redirects: bool,
}

impl ClientBuilder {
    /// Start a new builder pointed at `base_url`.
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            base_url: base_url.as_ref().to_owned(),
            accept_invalid_certs: true,
            timeout: Duration::from_secs(30),
            user_agent: format!("bezant/{}", env!("CARGO_PKG_VERSION")),
            follow_redirects: true,
        }
    }

    /// Accept self-signed / invalid TLS certificates. Defaults to **true**
    /// because the Gateway ships with a self-signed cert; set to `false`
    /// once you install a trusted cert.
    pub fn accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    /// Request timeout for every HTTP call (defaults to 30s).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Override the `User-Agent` header (defaults to `bezant/<version>`).
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    /// Follow HTTP redirects automatically. Defaults to `true` (reqwest's
    /// normal 10-hop policy). Set to `false` when you're operating as a
    /// reverse proxy and want 3xx responses passed through to the caller —
    /// otherwise the browser ends up seeing the redirected body at the
    /// original URL, which breaks relative asset paths on pages like
    /// CPGateway's `/sso/Login`.
    pub fn follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    /// Finish configuration and build the [`Client`].
    ///
    /// # Errors
    /// Propagates URL parse errors and reqwest build errors.
    pub fn build(self) -> Result<Client> {
        let redirect_policy = if self.follow_redirects {
            reqwest::redirect::Policy::default()
        } else {
            reqwest::redirect::Policy::none()
        };
        // Akamai fronts the CPAPI path — an empty POST that travels as
        // `Transfer-Encoding: chunked` (reqwest's default when the body
        // is an empty stream over HTTP/2) comes back as
        // `411 Length Required`. Forcing HTTP/1.1 gives hyper a chance
        // to serialize empty bodies as `Content-Length: 0`, which the
        // CDN accepts.
        let http = reqwest::Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .timeout(self.timeout)
            .user_agent(&self.user_agent)
            .redirect(redirect_policy)
            .http1_only()
            .build()
            .map_err(Error::Http)?;

        if self.accept_invalid_certs {
            warn!(
                "bezant: accepting invalid TLS certs (Gateway default self-signed cert). \
                 Set ClientBuilder::accept_invalid_certs(false) once you install a trusted cert."
            );
        }

        let api = bezant_api::IbRestApiClient::with_client(&self.base_url, http)
            .map_err(|e| Error::InvalidBaseUrl(e.to_string()))?;

        Ok(Client {
            inner: Arc::new(ClientInner { api }),
        })
    }
}
