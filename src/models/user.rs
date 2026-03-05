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
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: Option<String>,
    pub user: Option<UserInfo>,
    pub mfa_required: bool,
    pub session_token_partiel: Option<String>,
    pub mfa_verified: Option<bool>,
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

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-01: login payload now supports explicit credential source fields.
  - SBD-04: response model supports MFA challenge flow (partial session token).
  - SBD-21: representation encourages explicit MFA state checks by consumers.
- Not fully satisfiable in this file:
  - SBD-07/SBD-08 are implementation-level and cannot be guaranteed by DTOs alone.
    Alternative: enforce through service/middleware and runtime secret policy.
*/
