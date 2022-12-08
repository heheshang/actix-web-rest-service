use actix_web::{HttpResponse, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong".to_string())
}

#[cfg(test)]
mod tests {

    use crate::config::app::config_services;
    use actix_cors::Cors;
    use actix_web::{http, test, App};

    #[actix_rt::test]
    async fn test_ping() {
        let pool = crate::config::db::migrate_and_config_db(":memory:");
        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .app_data(pool.clone())
                .wrap(actix_web::middleware::Logger::default())
                .configure(config_services),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/ping").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
