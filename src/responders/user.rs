use crate::models::User;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};

// not in use - just an example
impl Responder for User {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self: Self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}
