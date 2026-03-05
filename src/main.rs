use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use hporterly::config;
use hporterly::handlers;
use hporterly::hl7;
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
    let per_second_api = std::cmp::max(1, (config.rate_limit_api_per_user / 60) as u64);
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(per_second_api)
        .burst_size(config.rate_limit_api_per_user)
        .finish()
        .expect("Failed to create rate limiter configuration");

    log::info!(
        "Rate limiting configured: {} req/s, burst: {}",
        per_second_api,
        config.rate_limit_api_per_user
    );
    log::info!(
        "Starting HPorterly server at {}:{}",
        config.host,
        config.port
    );
    let bind_host = config.host.clone();
    let bind_port = config.port;
    let hl7_bind_port = config.hl7_internal_port;
    let app_config = config.clone();
    let hl7_app_config = config.clone();
    let public_pool = pool.clone();
    let hl7_pool = pool.clone();

    let hl7_governor_conf = GovernorConfigBuilder::default()
        .per_second(50)
        .burst_size(100)
        .finish()
        .unwrap_or_else(|| {
            log::error!("Failed to create HL7 rate limiter configuration");
            panic!("Failed to create HL7 rate limiter configuration");
        });

    let public_server = HttpServer::new(move || {
        // CORS configuration
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        if app_config.cors_allowed_origins.is_empty() {
            log::warn!("No CORS_ALLOWED_ORIGINS configured: cross-origin requests will be denied");
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
            .app_data(web::Data::new(public_pool.clone()))
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
    .run();

    log::info!(
        "Starting internal HL7 webhook server at 127.0.0.1:{}",
        hl7_bind_port
    );
    let hl7_server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Governor::new(&hl7_governor_conf))
            .app_data(web::Data::new(hl7_pool.clone()))
            .app_data(web::Data::new(hl7_app_config.clone()))
            .configure(hl7::handlers::configure)
    })
    .bind(("127.0.0.1", hl7_bind_port))?
    .run();

    tokio::try_join!(public_server, hl7_server)?;
    Ok(())
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-11: global/public and dedicated internal rate limiting configured.
  - SBD-20: CORS explicit-origin mode only; no wildcard origin enablement.
  - SBD-21: HL7 endpoints isolated on loopback-only server and separate port.
  - SBD-22: security-relevant boot sequence validated at startup.
- Not fully satisfiable in this file:
  - SBD-08 (TLS 1.3) is deployment/topology dependent and not fully enforced here.
    Alternative: terminate TLS 1.3 at ingress/LB for public traffic; keep HL7 server private loopback.
*/
