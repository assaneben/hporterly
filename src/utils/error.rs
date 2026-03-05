use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    TooManyRequests(String),
    InternalServerError(String),
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::TooManyRequests(msg) => write!(f, "Too Many Requests: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            ApiError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            ApiError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            ApiError::TooManyRequests(_) => (StatusCode::TOO_MANY_REQUESTS, "TOO_MANY_REQUESTS"),
            ApiError::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            ApiError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
        };

        let message = match self {
            ApiError::InternalServerError(details) => {
                log::error!("Internal server error: {}", details);
                "Une erreur interne est survenue".to_string()
            }
            _ => self.to_string(),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message,
        })
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-11: explicit HTTP 429 support for abuse/rate-limit enforcement.
  - SBD-13: controlled and structured error responses.
  - SBD-21: explicit security failure types discourage permissive fallbacks.
- Not fully satisfiable in this file:
  - SBD-10 requires caller-side audit event emission when returning security errors.
    Alternative: add centralized error-to-audit bridge in middleware/handlers.
*/
