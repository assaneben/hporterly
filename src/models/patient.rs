use crate::schema::patients;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Patient entity for medical transport system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = patients)]
pub struct Patient {
    pub id: String, // Format: IPP-XXXXXX
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub service: Option<String>,
    pub room: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub date_of_birth: Option<NaiveDate>,
    pub sex: Option<String>,
}

/// Patient data for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = patients)]
pub struct NewPatient {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub service: Option<String>,
    pub room: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub sex: Option<String>,
}

/// Query parameters for patient search endpoint
#[derive(Debug, Deserialize)]
pub struct SearchPatientsQuery {
    pub search: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    10
}

/// Patient search response format for frontend autocomplete
#[derive(Debug, Serialize)]
pub struct PatientSearchResult {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub service: Option<String>,
    pub room: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub sex: Option<String>,
}

impl From<Patient> for PatientSearchResult {
    fn from(patient: Patient) -> Self {
        Self {
            id: patient.id,
            first_name: patient.first_name,
            last_name: patient.last_name,
            age: patient.age,
            gender: patient.gender,
            service: patient.service,
            room: patient.room,
            building: patient.building,
            floor: patient.floor,
            date_of_birth: patient.date_of_birth,
            sex: patient.sex,
        }
    }
}
