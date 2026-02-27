use actix_web::{web, HttpResponse};

use crate::errors::AppError;
use crate::models::{LoginRequest, TokenResponse};
use crate::state::AppState;

pub async fn login(
    state: web::Data<AppState>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    state
        .auth
        .verify_demo_credentials(&payload.username, &payload.password)?;
    let token = state.auth.issue_token(&payload.username, "dispatcher")?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in_seconds: state.config.jwt_exp_minutes * 60,
    }))
}
