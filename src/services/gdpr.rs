use serde_json::{json, Value};

use crate::models::{Ticket, User};
use crate::repositories::{AuditLogRepository, TicketRepository, UserRepository};
use crate::services::AuditService;
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

pub struct GdprService;

impl GdprService {
    pub fn delete_user_data(pool: &DbPool, actor: &User, user_id: &str) -> ApiResult<Value> {
        if actor.role != "administrateur" && actor.id != user_id {
            return Err(ApiError::Forbidden(
                "You are not authorized to delete this user's data".to_string(),
            ));
        }

        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let target_user = UserRepository::find_by_id(&mut conn, user_id)
            .map_err(|_| ApiError::NotFound(format!("User {} not found", user_id)))?;

        AuditService::log_action(
            pool,
            actor,
            "GDPR_DELETE_REQUEST",
            "user",
            user_id,
            Some(json!({
                "user_id": target_user.id,
                "username": target_user.username,
                "reason": "Right to be forgotten (Article 17 GDPR)"
            })),
            None,
            None,
            None,
        )?;

        TicketRepository::anonymize_requester(&mut conn, user_id, "ANONYMIZED_USER").map_err(
            |e| ApiError::InternalServerError(format!("Failed to anonymize tickets: {}", e)),
        )?;

        UserRepository::delete_by_id(&mut conn, user_id)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to delete user: {}", e)))?;

        log::info!("GDPR deletion completed for user: {}", user_id);

        Ok(json!({
            "deleted": true,
            "user_id": user_id,
            "message": "User data deleted successfully. Audit logs retained for legal compliance."
        }))
    }

    pub fn export_user_data(pool: &DbPool, actor: &User, user_id: &str) -> ApiResult<Value> {
        if actor.role != "administrateur" && actor.id != user_id {
            return Err(ApiError::Forbidden(
                "You are not authorized to export this user's data".to_string(),
            ));
        }

        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let target_user = UserRepository::find_by_id(&mut conn, user_id)
            .map_err(|_| ApiError::NotFound(format!("User {} not found", user_id)))?;
        let user_tickets: Vec<Ticket> = TicketRepository::list_by_requester(&mut conn, user_id)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load tickets: {}", e)))?;
        let user_audits =
            AuditLogRepository::list_by_user(&mut conn, user_id, 1000).map_err(|e| {
                ApiError::InternalServerError(format!("Failed to load audit logs: {}", e))
            })?;

        if let Err(err) = AuditService::log_action(
            pool,
            actor,
            "GDPR_EXPORT_REQUEST",
            "user",
            user_id,
            None,
            Some(json!({"export_date": chrono::Utc::now()})),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        let export_data = json!({
            "export_date": chrono::Utc::now().to_rfc3339(),
            "user": {
                "id": target_user.id,
                "username": target_user.username,
                "role": target_user.role,
                "first_name": target_user.first_name,
                "last_name": target_user.last_name,
                "email": target_user.email,
                "service": target_user.service,
                "created_at": target_user.created_at,
            },
            "tickets_created": user_tickets,
            "audit_logs": user_audits,
            "gdpr_notice": "This export contains all personal data we hold about you. Data retention policy: Audit logs are kept for 7 years for legal compliance."
        });

        log::info!("GDPR export completed for user: {}", user_id);
        Ok(export_data)
    }
}
