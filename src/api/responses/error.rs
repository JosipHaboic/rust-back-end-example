use std::fmt;
use serde_derive::{Serialize, Deserialize};
use serde_json::{json, to_string_pretty};
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::{StatusCode};


#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse<'a> {
	pub status: u16,
	pub message: &'a str,
}

impl <'a> fmt::Display for ErrorResponse <'a> {
	fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", to_string_pretty(self).unwrap())
	}
}

impl <'a> ResponseError for ErrorResponse <'a> {
	fn render_response(self: &Self) -> HttpResponse {
		let error_json = json!({"error": self.message});
		HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(error_json)
	}
}