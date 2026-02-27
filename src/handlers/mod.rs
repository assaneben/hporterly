pub mod auth;
pub mod health;
pub mod transfers;
pub mod ws;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/healthz").route(web::get().to(health::healthz)))
        .service(web::resource("/ws").route(web::get().to(ws::ws_index)))
        .service(
            web::scope("/api/v1")
                .service(web::resource("/auth/login").route(web::post().to(auth::login)))
                .service(
                    web::resource("/transfers")
                        .route(web::get().to(transfers::list_transfers))
                        .route(web::post().to(transfers::create_transfer)),
                ),
        );
}
