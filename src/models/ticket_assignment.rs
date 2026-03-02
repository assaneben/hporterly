use crate::schema::ticket_assignments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Rôle d'un brancardier dans une mission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssignmentRole {
    #[serde(rename = "supervisor")]
    Supervisor,
    #[serde(rename = "co_partner")]
    CoPartner,
}

impl std::fmt::Display for AssignmentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssignmentRole::Supervisor => write!(f, "supervisor"),
            AssignmentRole::CoPartner => write!(f, "co_partner"),
        }
    }
}

/// Assignation d'un brancardier à un ticket
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = ticket_assignments)]
pub struct TicketAssignment {
    pub id: String,
    pub ticket_id: String,
    pub porter_id: String,
    pub role: String,
    pub assigned_at: NaiveDateTime,
    pub removed_at: Option<NaiveDateTime>,
    pub is_active: bool,
}

/// Nouvelle assignation à insérer
#[derive(Debug, Insertable)]
#[diesel(table_name = ticket_assignments)]
pub struct NewTicketAssignment {
    pub id: String,
    pub ticket_id: String,
    pub porter_id: String,
    pub role: String,
    pub is_active: bool,
}

/// Requête pour ajouter un co-partner
#[derive(Debug, Deserialize)]
pub struct AddCoPartnerRequest {
    pub porter_id: String,
}

/// Requête pour désigner le prochain supervisor lors d'une désaffectation
#[derive(Debug, Deserialize)]
pub struct DesignateSuccessorRequest {
    pub next_supervisor_porter_id: Option<String>,
    pub comment: Option<String>,
}
