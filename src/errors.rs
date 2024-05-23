use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use serde::Serialize;

#[derive(Debug, Display, From)]
pub enum ApiError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
}

impl std::error::Error for ApiError {}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        match *self {
            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json(error_response)
            }
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
        }
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
