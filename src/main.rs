use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use hporterly::config;
use hporterly::handlers;
use hporterly::middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    if let Err(err) = dotenvy::dotenv() {
        log::debug!("No .env loaded: {}", err);
    }
    env_logger::init();

    // Load and validate configuration
    let config = config::Config::from_env();
    if let Err(e) = config.validate() {
        log::error!("Configuration validation failed: {}", e);
        panic!("Invalid configuration: {}", e);
    }

    log::info!("Configuration loaded and validated successfully");

    // Database connection pool
    log::info!("Creating database connection pool...");
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap_or_else(|e| {
        log::error!("Failed to create database connection pool: {}", e);
        eprintln!(
            "CRITICAL ERROR: Failed to create database connection pool: {}",
            e
        );
        std::process::exit(1);
    });

    log::info!("Database connection pool created");

    // Rate limiting configuration
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(500) // 500 requests per second
        .burst_size(1000) // Allow bursts up to 1000 requests
        .finish()
        .expect("Failed to create rate limiter configuration");

    log::info!("Rate limiting configured: 500 req/s, burst: 1000");
    log::info!(
        "Starting HPorterly server at {}:{}",
        config.host,
        config.port
    );
    let bind_host = config.host.clone();
    let bind_port = config.port;
    let app_config = config.clone();

    HttpServer::new(move || {
        // CORS configuration
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        if app_config.cors_allowed_origins.is_empty() {
            if app_config.environment.eq_ignore_ascii_case("development") {
                cors = cors.allow_any_origin();
            } else {
                log::warn!("No CORS_ALLOWED_ORIGINS configured in non-development environment");
            }
        } else {
            for origin in &app_config.cors_allowed_origins {
                cors = cors.allowed_origin(origin);
            }
        }

        App::new()
            .wrap(cors)
            .wrap(middleware::AuthMiddleware)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .configure(handlers::configure_health)
            .configure(handlers::configure_api_compat)
            .configure(handlers::configure_auth)
            .configure(handlers::configure_gdpr)
            .configure(handlers::configure_tickets)
            .configure(handlers::configure_porters)
            .configure(handlers::configure_patients)
            .configure(handlers::configure_services)
            .configure(handlers::configure_priority_rules)
            .configure(handlers::configure_referentials)
            .configure(handlers::configure_users)
            .configure(handlers::configure_notifications)
            // Serve static files LAST so API routes take precedence
            .service(actix_files::Files::new("/", "../frontend").index_file("index.html"))
    })
    .bind((bind_host.as_str(), bind_port))?
    .run()
    .await
}
