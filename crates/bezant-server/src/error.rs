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
#[derive(Debug)]
pub struct AppError(pub bezant::Error);

impl From<bezant::Error> for AppError {
    fn from(value: bezant::Error) -> Self {
        Self(value)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(bezant::Error::Api(value))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self.0 {
            bezant::Error::InvalidBaseUrl(_) => (StatusCode::BAD_REQUEST, "invalid_base_url"),
            bezant::Error::Http(_) => (StatusCode::BAD_GATEWAY, "upstream_http_error"),
            bezant::Error::Api(_) => (StatusCode::BAD_GATEWAY, "upstream_api_error"),
            bezant::Error::NotAuthenticated => (StatusCode::UNAUTHORIZED, "not_authenticated"),
            bezant::Error::NoSession => (StatusCode::SERVICE_UNAVAILABLE, "no_session"),
            bezant::Error::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
            // `bezant::Error` is `#[non_exhaustive]` — future-proof the match.
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        };
        let body = ErrorBody {
            code,
            message: self.0.to_string(),
        };
        (status, Json(body)).into_response()
    }
}
