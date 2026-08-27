use std::collections::HashSet;

use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload};
use futures_util::future::{ready, Ready};

use domain::db::db_transform::TransformId;

/// The set of transform ids the caller may read this request — owned,
/// granted directly, or granted via a workspace they belong to. Populated
/// once by `TransformAccessMiddleware` so handlers can check membership
/// locally instead of a grants lookup per resource id.
#[derive(Clone, Debug, Default)]
pub struct TransformAccessContext(HashSet<TransformId>);

impl TransformAccessContext {
    pub fn new(ids: HashSet<TransformId>) -> Self {
        Self(ids)
    }

    pub fn contains(&self, id: TransformId) -> bool {
        self.0.contains(&id)
    }
}

impl FromRequest for TransformAccessContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<TransformAccessContext>() {
            Some(ctx) => ready(Ok(ctx.clone())),
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "Transform access context missing",
            ))),
        }
    }
}
