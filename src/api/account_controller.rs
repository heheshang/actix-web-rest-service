use actix_web::{web, HttpRequest, HttpResponse, Result};
use log::info;

use crate::{
    config::db::Pool,
    constants,
    models::{
        response::ResponseBody,
        user::{LoginDto, UserDto},
    },
    services::account_service,
};

pub async fn signup(user_dto: web::Json<UserDto>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    info!("signup ...");
    match account_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn login(user_dto: web::Json<LoginDto>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::login(user_dto.0, &pool) {
        Ok(token) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token)))
        }
        Err(err) => Ok(err.response()),
    }
}
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        account_service::logout(authen_header, &pool);
        Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGOUT_SUCCESS,
            constants::EMPTY,
        )))
    } else {
        Ok(HttpResponse::Unauthorized().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        )))
    }
}

#[cfg(test)]
mod tests {

    use actix_cors::Cors;
    use actix_web::{
        body::to_bytes,
        http::{self, header, StatusCode},
        test,
        web::Data,
        App,
    };
    use log::info;

    use crate::config;
    use actix_service::Service;

    use futures_util::FutureExt;

    #[actix_rt::test]
    async fn test_signup_ok() {
        let pool = config::db::migrate_and_config_db(":memory:");
        info!("pool: {:?}", pool);
        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .app_data(Data::new(pool))
                .wrap(actix_web::middleware::Logger::default())
                .wrap(crate::middleware::auth_middleware::Authentication)
                .wrap_fn(|req, srv| {
                    info!("Hi from start. You requested: {}", req.path());
                    srv.call(req).map(|res| {
                        info!("Hi from response {:?}", res);
                        res
                    })
                })
                .configure(crate::config::app::config_services),
        )
        .await;
        info!("app 初始化完成");
        let resp = test::TestRequest::post()
            .uri("/api/auth/signup")
            .insert_header(header::ContentType::json())
            .set_payload(
                r#"{"username":"admin","email":"admin@gmail.com","password":"123456"}"#.as_bytes(),
            )
            .send_request(&app)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn test_signup_duplicate_user() {
        let pool = config::db::migrate_and_config_db(":memory:");
        info!("pool: {:?}", pool);
        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .app_data(Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .wrap(crate::middleware::auth_middleware::Authentication)
                .wrap_fn(|req, srv| {
                    info!("Hi from start. You requested: {}", req.path());
                    srv.call(req).map(|res| {
                        info!("Hi from response {:?}", res);
                        res
                    })
                })
                .configure(crate::config::app::config_services),
        )
        .await;

        test::TestRequest::post()
            .uri("/api/auth/signup")
            .insert_header(header::ContentType::json())
            .set_payload(
                r#"{"username":"admin","email":"admin@gmail.com","password":"123456"}"#.as_bytes(),
            )
            .send_request(&app)
            .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/signup")
            .insert_header(header::ContentType::json())
            .set_payload(
                r#"{"username":"admin","email":"admin@gmail.com","password":"123456"}"#.as_bytes(),
            )
            .send_request(&app)
            .await;
        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        // let status = resp.status();
        println!(
            "resp: {:?}",
            serde_json::from_slice::<serde_json::Value>(&body_bytes).unwrap()
        );
        // assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
