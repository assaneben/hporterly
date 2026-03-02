use crate::schema::{notification_preferences, notifications};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════
// Notification - Persisted notification entity
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = notifications)]
pub struct DbNotification {
    pub id: String,
    pub user_id: String,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub priority: String,
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    pub related_ticket_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notifications)]
pub struct NewDbNotification {
    pub id: String,
    pub user_id: String,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub priority: String,
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    pub related_ticket_id: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
// NotificationPreference - Per-user notification settings
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = notification_preferences)]
pub struct NotificationPreference {
    pub id: String,
    pub user_id: String,
    pub preferences: serde_json::Value,
    pub sound_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notification_preferences)]
pub struct NewNotificationPreference {
    pub id: String,
    pub user_id: String,
    pub preferences: serde_json::Value,
    pub sound_enabled: bool,
}

// ═══════════════════════════════════════════════════════════════
// Request / Response DTOs
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub preferences: Option<serde_json::Value>,
    pub sound_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UnreadCountResponse {
    pub count: i64,
}

#[derive(Debug, Deserialize)]
pub struct MarkReadRequest {
    pub notification_ids: Vec<String>,
}

/// Default preferences for new users
pub fn default_preferences() -> serde_json::Value {
    serde_json::json!({
        "ticket_created": {"enabled": true, "sound": true},
        "ticket_assigned": {"enabled": true, "sound": true},
        "ticket_updated": {"enabled": true, "sound": false},
        "ticket_completed": {"enabled": true, "sound": false},
        "ticket_canceled": {"enabled": true, "sound": true},
        "urgent_ticket": {"enabled": true, "sound": true},
        "help_request_received": {"enabled": true, "sound": true},
        "user_message": {"enabled": true, "sound": true},
        "co_partner_added": {"enabled": true, "sound": true},
        "co_partner_removed": {"enabled": true, "sound": true},
        "mission_suspended": {"enabled": true, "sound": true},
        "mission_resumed": {"enabled": true, "sound": false},
        "rdv_activated": {"enabled": true, "sound": true},
        "rdv_overdue": {"enabled": true, "sound": true}
    })
}
