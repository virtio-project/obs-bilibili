use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse, HttpResponseBuilder};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("bili error")]
    BiliError(#[from] bili::Error),
    #[error("transport error")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(ErrorResponse::from(self))
    }
}

impl From<&ApiError> for ErrorResponse {
    fn from(e: &ApiError) -> Self {
        Self {
            err: format!("{}", e),
        }
    }
}
