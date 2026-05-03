//! HTTP error type — maps `bezant::Error` into JSON responses.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// A JSON error envelope returned on failure.
#[derive(Debug, Serialize)]
pub struct ErrorBody {
    /// Machine-readable error code.
    pub code: &'static str,
    /// Human-readable message.
    pub message: String,
}

/// Wraps [`bezant::Error`] so axum handlers can bubble errors with `?`.
///
/// The inner error is deliberately private — the HTTP surface exposes
/// status + code + message, not the whole typed variant. Construct via
/// `From<bezant::Error>` (i.e. the `?` operator) rather than a literal.
#[derive(Debug)]
pub struct AppError(bezant::Error);

impl AppError {
    /// Borrow the wrapped error. Prefer matching on the HTTP response
    /// instead; this exists for logging / tracing use cases.
    #[must_use]
    pub fn inner(&self) -> &bezant::Error {
        &self.0
    }
}

impl From<bezant::Error> for AppError {
    fn from(value: bezant::Error) -> Self {
        Self(value)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(bezant::Error::from(value))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = map_status(&self.0);

        // Log every mapped failure at the boundary so production debuggability
        // doesn't depend on every handler manually emitting a span event.
        // 5xx are upstream/internal — `error!`. 4xx are caller-input — `warn!`.
        // Avoids logging the full debug repr to minimise log volume + redaction
        // surface (the typed `Display` impl is already redaction-safe).
        if status.is_server_error() {
            tracing::error!(
                code,
                status = %status,
                error = %self.0,
                "request failed (5xx)"
            );
        } else {
            tracing::warn!(
                code,
                status = %status,
                error = %self.0,
                "request failed (4xx)"
            );
        }

        let body = ErrorBody {
            code,
            message: self.0.to_string(),
        };
        (status, Json(body)).into_response()
    }
}

/// Map a `bezant::Error` variant to its HTTP status + machine code.
///
/// Branching by variant rather than reflection so the mapping is
/// auditable + the compiler enforces exhaustiveness via the
/// `#[non_exhaustive]` catch-all (`_`).
fn map_status(err: &bezant::Error) -> (StatusCode, &'static str) {
    use bezant::Error as E;
    match err {
        E::InvalidBaseUrl(_) => (StatusCode::BAD_REQUEST, "invalid_base_url"),
        E::UrlNotABase { .. } => (StatusCode::BAD_REQUEST, "url_not_a_base"),
        E::Http(e) => {
            // Branch on reqwest error shape so 504/503 are real signals
            // for upstream-overload retry loops (HAProxy / k8s probes /
            // alerting). `is_request()` covers DNS / TLS etc.
            if e.is_timeout() {
                (StatusCode::GATEWAY_TIMEOUT, "upstream_timeout")
            } else if e.is_connect() {
                (StatusCode::SERVICE_UNAVAILABLE, "upstream_unreachable")
            } else {
                (StatusCode::BAD_GATEWAY, "upstream_http_error")
            }
        }
        E::UpstreamStatus { status, .. } => {
            // 5xx and 429 propagate as 5xx; 4xx as the same 4xx the upstream
            // returned (so callers can distinguish "you sent garbage" from
            // "upstream is broken"). Other status codes default to 502.
            let s = StatusCode::from_u16(*status).unwrap_or(StatusCode::BAD_GATEWAY);
            if s.is_server_error() || s.as_u16() == 429 {
                (s, "upstream_status")
            } else if s.is_client_error() {
                (s, "upstream_client_error")
            } else {
                (StatusCode::BAD_GATEWAY, "upstream_status")
            }
        }
        E::Unknown { .. } => (StatusCode::BAD_GATEWAY, "upstream_unknown_variant"),
        E::Decode { .. } => (StatusCode::BAD_GATEWAY, "upstream_decode_error"),
        E::Api(_) => (StatusCode::BAD_GATEWAY, "upstream_api_error"),
        E::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
        E::MissingQuery { .. } => (StatusCode::BAD_REQUEST, "missing_query_param"),
        E::Header { .. } => (StatusCode::BAD_REQUEST, "invalid_header_value"),
        E::SymbolNotFound { .. } => (StatusCode::NOT_FOUND, "symbol_not_found"),
        E::BadConid { .. } => (StatusCode::BAD_GATEWAY, "upstream_bad_conid"),
        E::WsHandshake { .. } => (StatusCode::BAD_GATEWAY, "ws_handshake_failed"),
        E::WsTransport { .. } => (StatusCode::BAD_GATEWAY, "ws_transport_failed"),
        E::WsProtocol(_) => (StatusCode::BAD_GATEWAY, "ws_protocol_error"),
        E::ResponseBuild(_) => (StatusCode::INTERNAL_SERVER_ERROR, "response_build"),
        E::NotAuthenticated => (StatusCode::UNAUTHORIZED, "not_authenticated"),
        E::NoSession => (StatusCode::SERVICE_UNAVAILABLE, "no_session"),
        E::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        // `bezant::Error` is `#[non_exhaustive]` — future-proof the match.
        _ => {
            tracing::error!(error = ?err, "unmapped bezant::Error variant");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal")
        }
    }
}
