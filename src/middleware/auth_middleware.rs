use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

use crate::models::response::ResponseBody;
use crate::{config::db::Pool, constants, utils::token_utils};
use actix_service::{Service, Transform};
use actix_web::{
    body::EitherBody,
    dev::{ServiceRequest, ServiceResponse},
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    web::Data,
    Error, HttpResponse,
};

use futures_util::future::LocalBoxFuture;
use log::info;
pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        info!("AuthenticationMiddleware .... Transform");
        info!("AuthenticationMiddleware .... Transform");
        // ok(AuthenticationMiddleware { service })
        ready(Ok(AuthenticationMiddleware { service }))
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}
//

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    // forward_ready!(service);

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        log::info!("AuthenticationMiddleware call ...");
        info!("AuthenticationMiddleware .... call");
        let mut authenticate_pass = false;

        let headers = req.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
        }
        info!("AuthenticationMiddleware .... {}", authenticate_pass);
        if !authenticate_pass {
            if let Some(pool) = req.app_data::<Data<Pool>>() {
                info!("Connecting to database ...");
                if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
                    info!("Parsing authorization header...");
                    if let Ok(authen_header) = authen_header.to_str() {
                        if authen_header.starts_with("Bearer")
                            || authen_header.starts_with("bearer")
                        {
                            info!("Parsing token...");
                            let token = authen_header[6..authen_header.len()].to_string();
                            if let Ok(token) = crate::utils::token_utils::decode_token(token) {
                                info!("Decoding token...");
                                if token_utils::verify_token(&token, pool).is_ok() {
                                    info!("Validating token...");
                                    authenticate_pass = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if authenticate_pass {
            let res = self.service.call(req);

            Box::pin(async move {
                // r.map(|res| res.map_body(|_, body| EitherBody::Left(body)))

                let sss1 = res.await.map(ServiceResponse::map_into_left_body);
                match sss1 {
                    Ok(res) => {
                        info!("AuthenticationMiddleware .... ok");
                        Ok(res)
                    }
                    Err(err) => {
                        info!("AuthenticationMiddleware .... error");
                        Err(err)
                    }
                }
            })
        } else {
            let (request, _pl) = req.into_parts();

            let response = HttpResponse::Unauthorized()
                .json(ResponseBody::new(
                    constants::MESSAGE_INVALID_TOKEN,
                    constants::EMPTY,
                ))
                // constructed responses map to "right" body
                .map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(request, response)) })
        }
    }
}
