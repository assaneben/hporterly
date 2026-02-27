use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::models::HealthResponse;
use crate::state::AppState;

pub async fn healthz(state: web::Data<AppState>) -> HttpResponse {
    let queue_size = state.transfers.read().await.len();
    let body = HealthResponse {
        status: "ok".to_string(),
        app: "Hporterly".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        time_utc: Utc::now(),
        db_pool_configured: state.db_pool.is_some(),
        queue_size,
    };
    HttpResponse::Ok().json(body)
}
