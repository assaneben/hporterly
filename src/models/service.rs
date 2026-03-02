use crate::schema::services;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = services)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub building: String,
    pub floor: String,
    pub full_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = services)]
pub struct NewService {
    pub id: String,
    pub name: String,
    pub building: String,
    pub floor: String,
    pub full_name: String,
}
