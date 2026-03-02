use crate::models::{NewAuditLog, User};
use crate::schema::audit_logs;
use crate::utils::ApiResult;
use crate::DbPool;
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

pub struct AuditService;

impl AuditService {
    /// Enregistre une action dans l'audit log
    #[allow(clippy::too_many_arguments)]
    pub fn log_action(
        pool: &DbPool,
        user: &User,
        action: &str,
        entity_type: &str,
        entity_id: &str,
        old_value: Option<Value>,
        new_value: Option<Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> ApiResult<()> {
        let mut conn = pool.get().map_err(|e| {
            crate::utils::ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let new_audit = NewAuditLog {
            id: format!("audit-{}", Uuid::new_v4()),
            user_id: user.id.clone(),
            action: action.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            old_value,
            new_value,
            ip_address,
            user_agent,
        };

        diesel::insert_into(audit_logs::table)
            .values(&new_audit)
            .execute(&mut conn)
            .map_err(|e| {
                crate::utils::ApiError::InternalServerError(format!(
                    "Failed to create audit log: {}",
                    e
                ))
            })?;

        log::info!(
            "Audit log created: user={}, action={}, entity={}/{}",
            user.id,
            action,
            entity_type,
            entity_id
        );

        Ok(())
    }
}
