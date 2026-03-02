// GDPR Compliance Handlers
// Implements Right to be Forgotten and Data Export

use crate::models::User;
use crate::services::GdprService;
use crate::utils::ApiResult;
use crate::DbPool;
use actix_web::{delete, get, web, HttpResponse};

/// GDPR: Right to be Forgotten
#[delete("/api/users/{id}/gdpr")]
async fn delete_user_gdpr(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    user_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let payload = GdprService::delete_user_data(&pool, &user, user_id.as_str())?;
    Ok(HttpResponse::Ok().json(payload))
}

/// GDPR: Right to Data Portability
#[get("/api/users/{id}/gdpr/export")]
async fn export_user_data(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    user_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let export_data = GdprService::export_user_data(&pool, &user, user_id.as_str())?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"gdpr_export_{}.json\"", user_id.as_str()),
        ))
        .json(export_data))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_user_gdpr).service(export_user_data);
}
