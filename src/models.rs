use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum TransferPriority {
    #[default]
    Routine,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum TransferStatus {
    #[default]
    Queued,
    Assigned,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEvent {
    pub at: DateTime<Utc>,
    pub actor: String,
    pub action: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub request_id: Uuid,
    pub origin_location: String,
    pub destination_location: String,
    pub priority: TransferPriority,
    pub status: TransferStatus,
    pub requested_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub requested_by: String,
    pub assigned_to: Option<String>,
    pub history: Vec<TransferEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferRequest {
    pub origin_location: String,
    pub destination_location: String,
    #[serde(default)]
    pub priority: TransferPriority,
    pub requested_by: String,
    pub assigned_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub app: String,
    pub version: String,
    pub time_utc: DateTime<Utc>,
    pub db_pool_configured: bool,
    pub queue_size: usize,
}

#[cfg(test)]
mod tests {
    use super::TransferPriority;
    use serde_json::json;

    #[test]
    fn priority_is_snake_case() {
        let value = serde_json::to_value(TransferPriority::High).expect("serialize");
        assert_eq!(value, json!("high"));
    }
}
