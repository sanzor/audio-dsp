use actix_web::{
    Error, HttpMessage,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse},
};
use std::{future::Future, pin::Pin, rc::Rc};

use crate::token::token_utils::verify_token;

use super::jwt_context::JwtContext;

pub struct JwtAuthMiddlewareService<S> {
    pub service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            if req.method() == actix_web::http::Method::OPTIONS {
                return srv.call(req).await.map(|r| r.map_into_left_body());
            }

            let token = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .map(|t| t.to_owned())
                .or_else(|| req.cookie("auth_token").map(|c| c.value().to_owned()));

            let claims = match token {
                None => {
                    let res = req.into_response(actix_web::HttpResponse::Unauthorized().body("Missing auth token"));
                    return Ok(res.map_into_right_body());
                }
                Some(t) => match verify_token(&t) {
                    Ok(c) => c,
                    Err(_) => {
                        let res = req.into_response(actix_web::HttpResponse::Unauthorized().body("Invalid token"));
                        return Ok(res.map_into_right_body());
                    }
                },
            };

            let ctx = JwtContext {
                user_id: claims.user_id,
                is_admin: claims.is_admin,
            };

            req.extensions_mut().insert(ctx);
            srv.call(req).await.map(|r| r.map_into_left_body())
        })
    }
}
