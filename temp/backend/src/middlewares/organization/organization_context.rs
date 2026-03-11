use actix_web::{FromRequest, HttpMessage};
use futures_util::future::{ready, Ready};

use crate::domain::db::db_organization::OrganizationId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrganizationContext {
    pub id: OrganizationId,
    pub name: String,
    pub slug: String,
    pub billing_email: String,
    pub stripe_customer_id: Option<String>,
    pub status: String,
}

impl FromRequest for OrganizationContext {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.extensions().get::<OrganizationContext>().cloned() {
            Some(ctx) => ready(Ok(ctx)),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "Organization context missing",
            ))),
        }
    }
}
