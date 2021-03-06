use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse<'a> {
    pub status: u16,
    pub message: &'a str,
}

impl<'a> fmt::Display for ErrorResponse<'a> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl<'a> ResponseError for ErrorResponse<'a> {
    fn status_code(self: &Self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(self: &Self) -> HttpResponse {
        let error_json = json!({"error": self.message});
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .json(error_json)
    }
}
