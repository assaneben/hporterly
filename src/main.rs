mod auth;
mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod state;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::{info, warn};

use crate::config::AppConfig;
use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = AppConfig::from_env().expect("failed to load configuration");
    let _configured_cors_origins = config.cors_allowed_origins.clone();
    let bind_addr = format!("{}:{}", config.app_host, config.app_port);

    let db_pool = match db::create_pool(&config.database_url) {
        Ok(pool) => {
            if let Err(err) = db::run_migrations(&pool) {
                warn!("database migrations failed or skipped: {err}");
            }
            Some(pool)
        }
        Err(err) => {
            warn!("database pool not initialized: {err}");
            None
        }
    };

    let state = web::Data::new(AppState::new(config.clone(), db_pool));
    let governor_cfg = GovernorConfigBuilder::default()
        .per_second(500)
        .burst_size(1000)
        .finish()
        .expect("valid rate-limit config");

    info!("Starting Hporterly backend at {bind_addr}");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(Governor::new(&governor_cfg))
            .configure(handlers::configure)
    })
    .bind(bind_addr)?
    .run()
    .await
}
