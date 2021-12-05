use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub enum ApiErrorResponse {
    AuthenticationFailure,
    NeedsAuthentication,
    UnknownError,
}

#[derive(Debug, Serialize)]
pub struct ApiResponseBody {
    pub success: bool,
    pub errcode: u16,
    pub message: String,
}

impl actix_web::error::ResponseError for ApiErrorResponse {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiErrorResponse::AuthenticationFailure => StatusCode::UNAUTHORIZED,
            ApiErrorResponse::NeedsAuthentication => StatusCode::UNAUTHORIZED,
            ApiErrorResponse::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiErrorResponse::AuthenticationFailure => HttpResponse::build(self.status_code())
                .json(ApiResponseBody {
                    success: false,
                    errcode: 100,
                    message: String::from("authentication failure"),
                }),
            ApiErrorResponse::NeedsAuthentication => {
                HttpResponse::build(self.status_code()).json(ApiResponseBody {
                    success: false,
                    errcode: 110,
                    message: String::from("need authentication"),
                })
            }
            ApiErrorResponse::UnknownError => {
                HttpResponse::build(self.status_code()).json(ApiResponseBody {
                    success: false,
                    errcode: 500,
                    message: String::from("unknown error"),
                })
            }
        }
    }
}

use serde_json::to_string_pretty;
use std::fmt::{Display, Formatter};

impl Display for ApiErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}
