use crate::schema::help_requests;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct HelpRequest {
    pub id: String,
    pub ticket_id: String,
    pub requesting_porter_id: String,
    pub requested_porter_id: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub responded_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = help_requests)]
pub struct NewHelpRequest {
    pub id: String,
    pub ticket_id: String,
    pub requesting_porter_id: String,
    pub requested_porter_id: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateHelpRequest {
    pub requested_porter_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RespondToHelpRequest {
    pub accepted: bool,
}
