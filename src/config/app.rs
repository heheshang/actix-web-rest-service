use actix_web::web;
use log::info;

use crate::api::{account_controller, address_book_controller, ping_controller};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes... start");
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(
                web::scope("/auth")
                    .route("/signup", web::post().to(account_controller::signup))
                    .service(
                        web::resource("/login").route(web::post().to(account_controller::login)),
                    )
                    .service(
                        web::resource("/logout").route(web::post().to(account_controller::logout)),
                    ),
            )
            .service(
                web::scope("/address-book")
                    .service(
                        web::resource("")
                            .route(web::get().to(address_book_controller::find_all))
                            .route(web::post().to(address_book_controller::insert)),
                    )
                    .service(
                        web::resource("/id/{id}")
                            .route(web::get().to(address_book_controller::find_by_id))
                            .route(web::put().to(address_book_controller::update))
                            .route(web::delete().to(address_book_controller::delete)),
                    )
                    .service(
                        web::resource("/filter")
                            .route(web::get().to(address_book_controller::filter)),
                    ),
            ),
    );
    info!("Configuring routes... done");
}
