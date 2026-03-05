use actix_web::{get, web, HttpResponse};
use diesel::{sql_query, RunQueryDsl};
use serde_json::json;

use crate::DbPool;

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "hporterly-api",
    }))
}

#[get("/api/health")]
async fn api_health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "hporterly-api",
    }))
}

#[get("/ready")]
async fn ready(pool: web::Data<DbPool>) -> HttpResponse {
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

    HttpResponse::Ok().json(json!({
        "status": "ready",
        "service": "hporterly-api",
    }))
}

#[get("/api/ready")]
async fn api_ready(pool: web::Data<DbPool>) -> HttpResponse {
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

    HttpResponse::Ok().json(json!({
        "status": "ready",
        "service": "hporterly-api",
    }))
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
        .service(api_health)
        .service(ready)
        .service(api_ready)
        .service(api_version);
}
