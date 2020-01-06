use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use crate::models::User;

// not in use - just an example
impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self: Self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)
            )
        )
    }
}
