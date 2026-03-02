use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::AuditLog;
use crate::schema::audit_logs;

pub struct AuditLogRepository;

impl AuditLogRepository {
    pub fn list_by_user(
        conn: &mut PgConnection,
        user_id: &str,
        limit: i64,
    ) -> QueryResult<Vec<AuditLog>> {
        audit_logs::table
            .filter(audit_logs::user_id.eq(user_id))
            .order(audit_logs::created_at.desc())
            .limit(limit)
            .load::<AuditLog>(conn)
    }
}
