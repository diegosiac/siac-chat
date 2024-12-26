use actix_web::dev::{Service, ServiceRequest, Transform};
use actix_web::middleware::{self, Next};
use actix_web::{Error, HttpMessage};
use futures::future::{ok, Ready};
use jsonwebtoken::errors::ErrorKind;

use crate::lib::jwt::decode_jwt;

#[derive(Debug, Clone)]
pub struct AuthMiddleware {
    secret: String,
}

impl AuthMiddleware {
    pub fn new(secret: String) -> Self {
        AuthMiddleware { secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>
        + 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
            secret: self.secret.clone(),
        })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>
        + 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone();

        async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        let token_data = decode::<Claims>(
                            token,
                            &DecodingKey::from_secret(secret.as_ref()),
                            &Validation::default(),
                        );

                        match token_data {
                            Ok(data) => {
                                req.extensions_mut().insert(data.claims);
                                let res = self.service.call(req).await?;
                                return Ok(res);
                            }
                            Err(_) => {
                                return Ok(req.into_response(HttpResponse::Unauthorized().finish()))
                            }
                        }
                    }
                }
            }

            Ok(req.into_response(HttpResponse::Unauthorized().finish()))
        }
        .boxed_local()
    }
}
