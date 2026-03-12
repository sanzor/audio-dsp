use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use std::future::{ready, Ready};

use domain::project_role::ProjectRole;

#[derive(Debug, Clone)]
pub struct RoleContext {
    pub role: Option<ProjectRole>,
    pub is_admin: bool,
}

impl RoleContext {
    pub fn can_view(&self) -> bool {
        self.is_admin || self.role.is_some()
    }

    pub fn can_edit(&self) -> bool {
        self.is_admin
            || matches!(self.role, Some(ProjectRole::Owner) | Some(ProjectRole::Editor))
    }

    pub fn is_owner(&self) -> bool {
        self.is_admin || matches!(self.role, Some(ProjectRole::Owner))
    }
}

impl FromRequest for RoleContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<RoleContext>() {
            Some(ctx) => ready(Ok(ctx.clone())),
            None => ready(Err(actix_web::error::ErrorUnauthorized("No role context"))),
        }
    }
}
