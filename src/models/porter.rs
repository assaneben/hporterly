use crate::schema::porters;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PorterStatus {
    Available, // Disponible
    Busy,      // Mission en cours
    Break,     // En pause
    Offline,   // Hors service
}

impl std::fmt::Display for PorterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PorterStatus::Available => write!(f, "available"),
            PorterStatus::Busy => write!(f, "busy"),
            PorterStatus::Break => write!(f, "break"),
            PorterStatus::Offline => write!(f, "offline"),
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = porters)]
pub struct Porter {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub skills: Vec<Option<String>>,
    pub current_location: Option<String>,
    pub completed_missions_today: i32,
    pub total_missions: i32,
    pub rating: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = porters)]
pub struct NewPorter {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub skills: Vec<Option<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePorterStatus {
    pub status: String,
    pub location: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PorterWithScore {
    #[serde(flatten)]
    pub porter: Porter,
    pub score: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePorterRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePorterSkills {
    pub skills: Vec<String>,
}

impl Porter {
    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills
            .iter()
            .filter_map(|s| s.as_ref())
            .any(|s| s == skill)
    }

    pub fn is_available(&self) -> bool {
        self.status == PorterStatus::Available.to_string()
    }
}
