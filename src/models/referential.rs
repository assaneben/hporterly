use crate::schema::{
    referential_equipment, referential_services, referential_specimens, referential_transport_modes,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════
// Référentiel Services/Unités
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = referential_services)]
pub struct ReferentialService {
    pub id: String,
    pub name: String,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub site_id: String,
    pub site_name: String,
    pub building_id: String,
    pub building_name: String,
    pub level_id: String,
    pub level_name: String,
    pub zone_id: String,
    pub zone_name: String,
    pub subzone_id: Option<String>,
    pub subzone_name: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = referential_services)]
pub struct NewReferentialService {
    pub id: String,
    pub name: String,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub site_id: String,
    pub site_name: String,
    pub building_id: String,
    pub building_name: String,
    pub level_id: String,
    pub level_name: String,
    pub zone_id: String,
    pub zone_name: String,
    pub subzone_id: Option<String>,
    pub subzone_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReferentialServiceRequest {
    pub name: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub building_id: Option<String>,
    pub building_name: Option<String>,
    pub level_id: Option<String>,
    pub level_name: Option<String>,
    pub zone_id: Option<String>,
    pub zone_name: Option<String>,
    pub subzone_id: Option<String>,
    pub subzone_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReferentialServiceRequest {
    pub name: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub building_id: Option<String>,
    pub building_name: Option<String>,
    pub level_id: Option<String>,
    pub level_name: Option<String>,
    pub zone_id: Option<String>,
    pub zone_name: Option<String>,
    pub subzone_id: Option<String>,
    pub subzone_name: Option<String>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════
// Référentiel Équipements/Matériels
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = referential_equipment)]
pub struct ReferentialEquipment {
    pub id: String,
    pub label: String,
    pub sizes: Option<Vec<Option<String>>>,
    pub required_fields: serde_json::Value,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = referential_equipment)]
pub struct NewReferentialEquipment {
    pub id: String,
    pub label: String,
    pub sizes: Option<Vec<Option<String>>>,
    pub required_fields: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CreateReferentialEquipmentRequest {
    pub label: String,
    pub sizes: Option<Vec<String>>,
    pub required_fields: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReferentialEquipmentRequest {
    pub label: Option<String>,
    pub sizes: Option<Vec<String>>,
    pub required_fields: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════
// Référentiel Modes de Transport
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = referential_transport_modes)]
pub struct ReferentialTransportMode {
    pub id: String,
    pub label: String,
    pub is_active: bool,
    pub sort_order: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = referential_transport_modes)]
pub struct NewReferentialTransportMode {
    pub id: String,
    pub label: String,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateReferentialTransportModeRequest {
    pub label: String,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReferentialTransportModeRequest {
    pub label: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════
// Référentiel Types de Prélèvement
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = referential_specimens)]
pub struct ReferentialSpecimen {
    pub id: String,
    pub label: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = referential_specimens)]
pub struct NewReferentialSpecimen {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReferentialSpecimenRequest {
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReferentialSpecimenRequest {
    pub label: Option<String>,
    pub is_active: Option<bool>,
}
