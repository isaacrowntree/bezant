//! Client wrapper: composes the auto-generated [`bezant_api::IbRestApiClient`]
//! with saner defaults for talking to the local IBKR Client Portal Gateway.

use std::sync::Arc;
use std::time::Duration;

use reqwest::cookie::Jar;
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
    http: reqwest::Client,
    base_url: url::Url,
    gateway_root: url::Url,
    cookie_jar: Arc<Jar>,
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
        &self.inner.http
    }

    /// Base URL the client is pointed at, including the CPAPI prefix
    /// (e.g. `https://localhost:5000/v1/api`).
    #[must_use]
    pub fn base_url(&self) -> &url::Url {
        &self.inner.base_url
    }

    /// The Gateway's root URL — [`Client::base_url`] with the CPAPI prefix
    /// trimmed off (e.g. `https://localhost:5000/`). Useful when you need to
    /// hit paths the Gateway serves outside `/v1/api` (login, static assets).
    #[must_use]
    pub fn gateway_root_url(&self) -> &url::Url {
        &self.inner.gateway_root
    }

    /// Shared cookie jar backing the underlying `reqwest::Client`.
    ///
    /// Expose this when you're running bezant alongside a reverse proxy
    /// (for example bezant-server's `/sso/Login` passthrough): you can
    /// inject cookies that arrive from the proxied caller so that typed
    /// API calls made through the same `Client` see the same session.
    #[must_use]
    pub fn cookie_jar(&self) -> Arc<Jar> {
        Arc::clone(&self.inner.cookie_jar)
    }
}

/// Builder for [`Client`].
#[must_use]
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base_url: String,
    accept_invalid_certs: bool,
    timeout: Duration,
    user_agent: String,
    follow_redirects: bool,
    http1_only: bool,
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
            // Default to HTTP/1.1-only because the production CPAPI path is
            // fronted by an Akamai CDN that rejects empty POSTs without a
            // Content-Length header — something hyper can end up emitting
            // under HTTP/2. See `ClientBuilder::http1_only` for the escape
            // hatch.
            http1_only: true,
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

    /// Force HTTP/1.1 only (no ALPN upgrade to HTTP/2). Defaults to `true`
    /// because IBKR fronts the CPAPI with an Akamai CDN that rejects
    /// empty-body POSTs shipped over HTTP/2 with `411 Length Required`.
    /// Flip this to `false` if you're targeting a Gateway deployment that
    /// does not sit behind Akamai (e.g. a self-hosted instance) and you
    /// want the latency benefits of HTTP/2.
    pub fn http1_only(mut self, http1_only: bool) -> Self {
        self.http1_only = http1_only;
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
        let cookie_jar = Arc::new(Jar::default());
        let mut http_builder = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_jar))
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .timeout(self.timeout)
            .user_agent(&self.user_agent)
            .redirect(redirect_policy);
        if self.http1_only {
            http_builder = http_builder.http1_only();
        }
        let http = http_builder.build().map_err(Error::Http)?;

        if self.accept_invalid_certs {
            warn!(
                "bezant: accepting invalid TLS certs (Gateway default self-signed cert). \
                 Set ClientBuilder::accept_invalid_certs(false) once you install a trusted cert."
            );
        }

        let api = bezant_api::IbRestApiClient::with_client(&self.base_url, http.clone())
            .map_err(|e| Error::InvalidBaseUrl(e.to_string()))?;
        let base_url: url::Url = self
            .base_url
            .parse()
            .map_err(|e: url::ParseError| Error::InvalidBaseUrl(e.to_string()))?;
        let gateway_root = derive_gateway_root(&base_url);

        Ok(Client {
            inner: Arc::new(ClientInner {
                api,
                http,
                base_url,
                gateway_root,
                cookie_jar,
            }),
        })
    }
}

/// Strip the CPAPI prefix off `base_url` to recover the Gateway root.
///
/// Handles both the `.../v1/api` and `.../v1/api/` forms; returns the
/// origin (`scheme://host[:port]/`) if we can't identify the prefix.
fn derive_gateway_root(base_url: &url::Url) -> url::Url {
    let mut root = base_url.clone();
    // Normalise away trailing slashes so path segment editing is consistent.
    if root.path().ends_with('/') {
        let trimmed = root.path().trim_end_matches('/').to_owned();
        root.set_path(&trimmed);
    }
    if root.path().ends_with("/v1/api") {
        let new_path = root
            .path()
            .strip_suffix("/v1/api")
            .unwrap_or("")
            .to_owned();
        root.set_path(&new_path);
    }
    // Always end the root with a single '/', so callers can `.join("sso/Login")`.
    if !root.path().ends_with('/') {
        let with_slash = format!("{}/", root.path());
        root.set_path(&with_slash);
    }
    root.set_query(None);
    root.set_fragment(None);
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gateway_root_strips_v1_api() {
        let base: url::Url = "https://localhost:5000/v1/api".parse().unwrap();
        assert_eq!(
            derive_gateway_root(&base).as_str(),
            "https://localhost:5000/"
        );
    }

    #[test]
    fn gateway_root_strips_trailing_slash() {
        let base: url::Url = "https://localhost:5000/v1/api/".parse().unwrap();
        assert_eq!(
            derive_gateway_root(&base).as_str(),
            "https://localhost:5000/"
        );
    }

    #[test]
    fn gateway_root_preserves_custom_prefix() {
        // Some self-hosted deployments prefix the CPAPI path with their own
        // routing — if there's no `/v1/api` suffix we just drop trailing
        // slashes and keep whatever's there.
        let base: url::Url = "https://gw.example.com/ibkr/v1/api".parse().unwrap();
        assert_eq!(
            derive_gateway_root(&base).as_str(),
            "https://gw.example.com/ibkr/"
        );
    }
}
