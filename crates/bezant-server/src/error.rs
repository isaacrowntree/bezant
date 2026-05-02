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
        let (status, code) = match &self.0 {
            bezant::Error::InvalidBaseUrl(_) => (StatusCode::BAD_REQUEST, "invalid_base_url"),
            bezant::Error::Http(_) => (StatusCode::BAD_GATEWAY, "upstream_http_error"),
            bezant::Error::Api(_) => (StatusCode::BAD_GATEWAY, "upstream_api_error"),
            bezant::Error::Decode(_) => (StatusCode::BAD_GATEWAY, "upstream_decode_error"),
            bezant::Error::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            bezant::Error::NotAuthenticated => (StatusCode::UNAUTHORIZED, "not_authenticated"),
            bezant::Error::NoSession => (StatusCode::SERVICE_UNAVAILABLE, "no_session"),
            bezant::Error::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
            // `bezant::Error` is `#[non_exhaustive]` — future-proof the match.
            _ => {
                tracing::error!(error = ?self.0, "unmapped bezant::Error variant");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal")
            }
        };
        let body = ErrorBody {
            code,
            message: self.0.to_string(),
        };
        (status, Json(body)).into_response()
    }
}
