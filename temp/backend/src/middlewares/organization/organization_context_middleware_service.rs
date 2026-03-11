use std::{rc::Rc, sync::Arc};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    http::Method,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::{
    middlewares::{
        jwt::jwt_context::JWTContext, organization::organization_context::OrganizationContext,
    },
    organizations::organization_provider::OrganizationProvider,
};

pub struct OrganizationContextMiddlewareService<S> {
    service: Rc<S>,
    organization_provider: Arc<dyn OrganizationProvider>,
}

impl<S> OrganizationContextMiddlewareService<S> {
    pub fn new(service: Rc<S>, organization_provider: Arc<dyn OrganizationProvider>) -> Self {
        Self {
            service,
            organization_provider,
        }
    }
}

impl<S, B> Service<ServiceRequest> for OrganizationContextMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let organization_provider = self.organization_provider.clone();

        Box::pin(async move {
            if req.method() == Method::OPTIONS {
                return service.call(req).await.map(|res| res.map_into_left_body());
            }

            // JWTContext was already deposited by the JWT middleware upstream
            let jwt_ctx = req.extensions().get::<JWTContext>().copied();

            let Some(jwt_ctx) = jwt_ctx else {
                return Ok(req
                    .into_response(HttpResponse::Unauthorized().body("JWT context missing"))
                    .map_into_right_body());
            };

            // If the token is org-scoped, verify the org still exists and deposit its context
            if let Some(org_id) = jwt_ctx.org_id {
                match organization_provider.get_organization(org_id).await {
                    Ok(Some(result)) => {
                        let org = result.organization;
                        if org.status != "active" {
                            return Ok(req
                                .into_response(
                                    HttpResponse::Forbidden().body("organization is suspended"),
                                )
                                .map_into_right_body());
                        }
                        req.extensions_mut().insert(OrganizationContext {
                            id: org.id,
                            name: org.name,
                            slug: org.slug,
                            billing_email: org.billing_email,
                            stripe_customer_id: org.stripe_customer_id,
                            status: org.status,
                        });
                    }
                    Ok(None) => {
                        return Ok(req
                            .into_response(
                                HttpResponse::Unauthorized().body("organization not found"),
                            )
                            .map_into_right_body());
                    }
                    Err(e) => {
                        return Ok(req
                            .into_response(
                                HttpResponse::InternalServerError()
                                    .body("failed to verify organization"),
                            )
                            .map_into_right_body());
                    }
                }
            }

            service.call(req).await.map(|res| res.map_into_left_body())
        })
    }
}
