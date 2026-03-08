use actix_web::{get, web, HttpResponse};
use diesel::{sql_query, RunQueryDsl};
use serde_json::json;

use crate::telemetry;
use crate::DbPool;

fn ok_payload(status: &str) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": status,
        "service": "hporterly-api",
    }))
}

fn readiness_payload(pool: &DbPool) -> HttpResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(error) => {
            log::error!("Readiness failed: database connection error: {}", error);
            return HttpResponse::ServiceUnavailable().json(json!({
                "status": "error",
                "service": "hporterly-api",
            }));
        }
    };

    if let Err(error) = sql_query("SELECT 1").execute(&mut conn) {
        log::error!("Readiness failed: database ping error: {}", error);
        return HttpResponse::ServiceUnavailable().json(json!({
            "status": "error",
            "service": "hporterly-api",
        }));
    }

    ok_payload("ready")
}

#[get("/health")]
async fn health() -> HttpResponse {
    ok_payload("ok")
}

#[get("/healthz")]
async fn healthz() -> HttpResponse {
    ok_payload("ok")
}

#[get("/api/health")]
async fn api_health() -> HttpResponse {
    ok_payload("ok")
}

#[get("/ready")]
async fn ready(pool: web::Data<DbPool>) -> HttpResponse {
    readiness_payload(pool.get_ref())
}

#[get("/readyz")]
async fn readyz(pool: web::Data<DbPool>) -> HttpResponse {
    readiness_payload(pool.get_ref())
}

#[get("/api/ready")]
async fn api_ready(pool: web::Data<DbPool>) -> HttpResponse {
    readiness_payload(pool.get_ref())
}

#[get("/metrics")]
async fn metrics(pool: web::Data<DbPool>) -> HttpResponse {
    match telemetry::render_prometheus_metrics(pool.get_ref()) {
        Ok(payload) => HttpResponse::Ok()
            .content_type("text/plain; version=0.0.4; charset=utf-8")
            .body(payload),
        Err(error) => {
            log::error!("Failed to render Prometheus metrics: {}", error);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "service": "hporterly-api",
            }))
        }
    }
}

#[get("/api/metrics")]
async fn api_metrics(pool: web::Data<DbPool>) -> HttpResponse {
    match telemetry::render_prometheus_metrics(pool.get_ref()) {
        Ok(payload) => HttpResponse::Ok()
            .content_type("text/plain; version=0.0.4; charset=utf-8")
            .body(payload),
        Err(error) => {
            log::error!("Failed to render Prometheus metrics: {}", error);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "service": "hporterly-api",
            }))
        }
    }
}

#[get("/api/healthz")]
async fn api_healthz() -> HttpResponse {
    ok_payload("ok")
}

#[get("/api/readyz")]
async fn api_readyz(pool: web::Data<DbPool>) -> HttpResponse {
    readiness_payload(pool.get_ref())
}

#[get("/api/version")]
async fn api_version() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "service": "hporterly-api",
        "default_version": "v1",
        "supported_versions": ["v1"],
        "base_paths": {
            "versioned": "/api/v1",
            "legacy_compat": "/api"
        },
        "compatibility_mode": true
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health)
        .service(healthz)
        .service(api_health)
        .service(api_healthz)
        .service(ready)
        .service(readyz)
        .service(api_ready)
        .service(api_readyz)
        .service(metrics)
        .service(api_metrics)
        .service(api_version);
}
