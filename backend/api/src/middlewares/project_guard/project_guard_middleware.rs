use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use std::{future::{ready, Ready}, sync::Arc};

use crate::memberships::memberships_provider::MembershipsProvider;
use crate::users::user_provider::UserProvider;

use super::project_guard_middleware_service::ProjectGuardMiddlewareService;

#[derive(Clone)]
pub struct ProjectGuardMiddleware {
    pub memberships: Arc<dyn MembershipsProvider>,
    pub user_provider: Arc<dyn UserProvider>,
}

impl<S, B> Transform<S, ServiceRequest> for ProjectGuardMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = ProjectGuardMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProjectGuardMiddlewareService {
            service: std::rc::Rc::new(service),
            memberships: Arc::clone(&self.memberships),
            user_provider: Arc::clone(&self.user_provider),
        }))
    }
}
