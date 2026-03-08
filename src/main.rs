use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{dev::Service, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use hporterly::config;
use hporterly::handlers;
use hporterly::hl7;
use hporterly::middleware;
use hporterly::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    if let Err(err) = dotenvy::dotenv() {
        log::debug!("No .env loaded: {}", err);
    }
    let config = config::Config::from_env();
    if let Err(error) = telemetry::init_logging(config.environment.as_str()) {
        eprintln!("Failed to initialize structured logging: {}", error);
        return Err(std::io::Error::other(error.to_string()));
    }

    // Load and validate configuration
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
        eprintln!("CRITICAL ERROR: Failed to create database connection pool: {}", e);
        std::process::exit(1);
    });

    log::info!("Database connection pool created");
    if !std::path::Path::new(config.frontend_static_dir.as_str()).exists() {
        log::warn!(
            "Frontend static directory '{}' does not exist yet; API will still start",
            config.frontend_static_dir
        );
    }

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
    log::info!("Starting HPorterly server at {}:{}", config.host, config.port);
    let bind_host = config.host.clone();
    let bind_port = config.port;
    let hl7_bind_port = config.hl7_internal_port;
    let app_config = config.clone();
    let hl7_app_config = config.clone();
    let public_pool = pool.clone();
    let hl7_pool = pool.clone();

    let hl7_governor_conf =
        GovernorConfigBuilder::default().per_second(50).burst_size(100).finish().unwrap_or_else(
            || {
                log::error!("Failed to create HL7 rate limiter configuration");
                panic!("Failed to create HL7 rate limiter configuration");
            },
        );

    let public_server = HttpServer::new(move || {
        // CORS configuration
        let mut cors = Cors::default().allow_any_method().allow_any_header().max_age(3600);
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
            .wrap_fn(|req, srv| {
                let method = req.method().as_str().to_string();
                let raw_path = req.path().to_string();
                let peer_ip = req
                    .peer_addr()
                    .map(|addr| addr.ip().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                let started_at = std::time::Instant::now();
                let fut = srv.call(req);

                async move {
                    let res = fut.await?;
                    let status = res.status().as_u16();
                    let route_pattern = res.request().match_pattern();
                    let path_label = telemetry::normalize_path_label(
                        route_pattern.as_deref(),
                        raw_path.as_str(),
                    );
                    let elapsed = started_at.elapsed();

                    telemetry::observe_http_request(
                        method.as_str(),
                        path_label.as_str(),
                        status,
                        elapsed,
                    );
                    tracing::info!(
                        target: "http_access",
                        method = %method,
                        path = %path_label,
                        status,
                        latency_ms = elapsed.as_millis() as u64,
                        peer_ip = %peer_ip,
                        "http_request"
                    );

                    Ok(res)
                }
            })
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
            .configure(handlers::configure_reports)
            .configure(handlers::configure_referentials)
            .configure(handlers::configure_users)
            .configure(handlers::configure_notifications)
            // Serve static files LAST so API routes take precedence
            .service(
                actix_files::Files::new("/", app_config.frontend_static_dir.clone())
                    .index_file("index.html"),
            )
    })
    .bind((bind_host.as_str(), bind_port))?
    .run();

    log::info!("Starting internal HL7 webhook server at 127.0.0.1:{}", hl7_bind_port);
    let hl7_server = HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let method = req.method().as_str().to_string();
                let raw_path = req.path().to_string();
                let started_at = std::time::Instant::now();
                let fut = srv.call(req);

                async move {
                    let res = fut.await?;
                    let status = res.status().as_u16();
                    let route_pattern = res.request().match_pattern();
                    let path_label = telemetry::normalize_path_label(
                        route_pattern.as_deref(),
                        raw_path.as_str(),
                    );
                    let elapsed = started_at.elapsed();

                    telemetry::observe_http_request(
                        method.as_str(),
                        path_label.as_str(),
                        status,
                        elapsed,
                    );
                    tracing::info!(
                        target: "hl7_access",
                        method = %method,
                        path = %path_label,
                        status,
                        latency_ms = elapsed.as_millis() as u64,
                        "http_request"
                    );

                    Ok(res)
                }
            })
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
