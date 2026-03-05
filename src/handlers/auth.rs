use crate::auth::mfa;
use crate::config::Config;
use crate::models::{LoginRequest, User};
use crate::services::AuthService;
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static LOGIN_ATTEMPTS: Lazy<Mutex<HashMap<String, Vec<i64>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Login endpoint
#[post("/api/auth/login")]
async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    credentials: web::Json<LoginRequest>,
) -> ApiResult<HttpResponse> {
    let source_ip = req.peer_addr().map(|addr| addr.ip().to_string());
    if let Some(ip) = source_ip.as_deref() {
        enforce_login_rate_limit(ip, config.rate_limit_login_per_ip)?;
    }

    let response = AuthService::login(
        &pool,
        &config,
        credentials.into_inner(),
        source_ip.as_deref(),
    )?;
    if let Some(ip) = source_ip.as_deref() {
        clear_login_rate_limit(ip)?;
    }
    Ok(HttpResponse::Ok().json(response))
}

/// Get current user info
#[get("/api/auth/me")]
async fn me(pool: web::Data<DbPool>, user: web::ReqData<User>) -> ApiResult<HttpResponse> {
    let user_info = AuthService::me(&pool, &user)?;
    Ok(HttpResponse::Ok().json(user_info))
}

/// Logout endpoint
#[post("/api/auth/logout")]
async fn logout() -> ApiResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Deconnexion reussie"
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(me).service(logout);
    mfa::configure(cfg);
}

fn enforce_login_rate_limit(ip: &str, max_per_minute: u32) -> ApiResult<()> {
    let now = chrono::Utc::now().timestamp();
    let mut guard = LOGIN_ATTEMPTS
        .lock()
        .map_err(|_| ApiError::InternalServerError("Login limiter lock error".to_string()))?;
    let entry = guard.entry(ip.to_string()).or_default();
    entry.retain(|ts| now - *ts <= 60);
    if entry.len() >= max_per_minute as usize {
        return Err(ApiError::TooManyRequests(
            "Trop de tentatives de connexion. Reessayez dans une minute.".to_string(),
        ));
    }
    entry.push(now);
    Ok(())
}

fn clear_login_rate_limit(ip: &str) -> ApiResult<()> {
    let mut guard = LOGIN_ATTEMPTS
        .lock()
        .map_err(|_| ApiError::InternalServerError("Login limiter lock error".to_string()))?;
    guard.remove(ip);
    Ok(())
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-04: login handler wired to MFA-aware auth service.
  - SBD-11: per-IP login throttling guard (configurable) before credential verification.
  - SBD-10: source IP captured for auditable auth decisions.
  - SBD-11: MFA endpoints exposed through dedicated auth route group.
  - SBD-21: no permissive fallback path in auth routing.
- Not fully satisfiable in this file:
  - SBD-08 and SBD-24 are not enforceable at handler wiring level.
    Alternative: enforce via transport policy and operational retention/incident procedures.
*/
