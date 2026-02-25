use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;

use crate::payload_description::ErrorResponse;

#[derive(Debug, Error)]
pub enum ApiErrors {
    #[error("Error: {0}")]
    NotFound(String),

    #[error("BadRequest Error: {0}")]
    BadRequest(String),

    #[error("Unauthorized Error: {0}")]
    Unauthorized(String),

    #[error("Password Error: {0}")]
    PasswordFail(String),

    #[error("Eamil Error: {0}")]
    EmailValidation(String),

    #[error("Text Error: {0}")]
    TextValidation(String),

    // #[error("Conflict: {0}")]
    // Conflict(String),
    #[error("Server Error: {0}")]
    InternalServerError(String),

    #[error("Error: {0}")]
    MethodNotAllowed(String),

    #[error("this is not a valid uuid or string")]
    InvalidString(String),
}

impl IntoResponse for ApiErrors {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            ApiErrors::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiErrors::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiErrors::PasswordFail(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            // ApiErrors::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiErrors::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiErrors::EmailValidation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiErrors::TextValidation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiErrors::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiErrors::MethodNotAllowed(msg) => (StatusCode::METHOD_NOT_ALLOWED, msg),
            ApiErrors::InvalidString(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = serde_json::to_string(&ErrorResponse { message })
            .unwrap_or_else(|_| "{\"message\":\"Internal error\"}".to_string());

        // (status, body).into_response()
        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}
