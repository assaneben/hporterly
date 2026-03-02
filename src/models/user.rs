use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Demandeur,      // Requester (soignant)
    Brancardier,    // Porter
    Regulateur,     // Regulateur / superviseur
    Administrateur, // Administrator
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserRole::Demandeur => write!(f, "demandeur"),
            UserRole::Brancardier => write!(f, "brancardier"),
            UserRole::Regulateur => write!(f, "regulateur"),
            UserRole::Administrateur => write!(f, "administrateur"),
        }
    }
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub service: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub service: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub service: Option<String>,
    pub porter_id: Option<String>,
}

// La conversion est geree manuellement dans les handlers pour inclure porter_id via une requete DB.
