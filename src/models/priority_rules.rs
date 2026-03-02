use crate::schema::priority_rules_config;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = priority_rules_config)]
pub struct PriorityRulesConfig {
    pub id: String,
    pub rules_json: serde_json::Value,
    pub is_active: bool,
    pub updated_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = priority_rules_config)]
pub struct NewPriorityRulesConfig {
    pub id: String,
    pub rules_json: serde_json::Value,
    pub is_active: bool,
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePriorityRulesRequest {
    pub rules_json: serde_json::Value,
    pub is_active: Option<bool>,
}
