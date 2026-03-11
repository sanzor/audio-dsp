use std::{rc::Rc, sync::Arc};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    http::Method,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::middlewares::jwt::{
    jwt_context::JWTContext, jwt_provider::JwtProvider, portal_jwt_claims::TokenPurpose,
};

pub struct JwtAuthMiddlewareService<S> {
    service: Rc<S>,
    jwt: Arc<dyn JwtProvider>,
}

impl<S> JwtAuthMiddlewareService<S> {
    pub fn new(service: Rc<S>, jwt: Arc<dyn JwtProvider>) -> Self {
        Self { service, jwt }
    }
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddlewareService<S>
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
        let jwt = self.jwt.clone();
        let service = self.service.clone();

        Box::pin(async move {
            if req.method() == Method::OPTIONS {
                return service.call(req).await.map(|res| res.map_into_left_body());
            }

            let token = req
                .headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok())
                .and_then(|header| header.strip_prefix("Bearer "))
                .map(str::trim)
                .filter(|value| !value.is_empty());

            let Some(token) = token else {
                return Ok(req
                    .into_response(HttpResponse::Unauthorized().body("missing bearer token"))
                    .map_into_right_body());
            };

            let claims = match jwt.verify(token) {
                Ok(c) => c,
                Err(e) => {
                    return Ok(req
                        .into_response(
                            HttpResponse::Unauthorized()
                                .body(format!("invalid token: {e}")),
                        )
                        .map_into_right_body());
                }
            };

            if claims.purpose != TokenPurpose::Access {
                return Ok(req
                    .into_response(
                        HttpResponse::Unauthorized().body("token not valid for API access"),
                    )
                    .map_into_right_body());
            }

            req.extensions_mut().insert(JWTContext {
                user_id: claims.user_id,
                org_id: claims.org_id,
            });

            service.call(req).await.map(|res| res.map_into_left_body())
        })
    }
}
