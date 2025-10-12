use actix::fut::{ready, Ready};
use actix_web::{Error, FromRequest};

pub struct CsrfToken;

impl FromRequest for CsrfToken {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_http::Payload,
    ) -> Self::Future {
        let csrf_cookie = req.cookie("csrf_token").map(|c| c.value().to_string());
        let csrf_header = req
            .headers()
            .get("x-csrf-token")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        match (csrf_cookie, csrf_header) {
            (Some(cookie), Some(header)) if cookie == header => ready(Ok(CsrfToken)),
            _ => ready(Err(actix_web::error::ErrorForbidden("Invalid CSRF token"))),
        }
    }
}
