use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::{
    CreateReferentialEquipmentRequest, CreateReferentialServiceRequest,
    CreateReferentialSpecimenRequest, CreateReferentialTransportModeRequest,
    NewReferentialEquipment, NewReferentialService, NewReferentialSpecimen,
    NewReferentialTransportMode, ReferentialEquipment, ReferentialService, ReferentialSpecimen,
    ReferentialTransportMode, UpdateReferentialEquipmentRequest, UpdateReferentialServiceRequest,
    UpdateReferentialSpecimenRequest, UpdateReferentialTransportModeRequest, User,
};
use crate::repositories::{
    EquipmentUpdateData, ReferentialRepository, ServiceUpdateData, SpecimenUpdateData,
    TransportModeUpdateData,
};
use crate::utils::{require_admin, ApiError, ApiResult};
use crate::DbPool;

pub struct ReferentialCatalogService;

impl ReferentialCatalogService {
    pub fn list_services_admin(pool: &DbPool, user: &User) -> ApiResult<Vec<ReferentialService>> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_services(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load services: {}", e)))
    }

    pub fn list_services_active(pool: &DbPool) -> ApiResult<Vec<ReferentialService>> {
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_active_services(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load services: {}", e)))
    }

    pub fn create_service(
        pool: &DbPool,
        user: &User,
        body: CreateReferentialServiceRequest,
    ) -> ApiResult<ReferentialService> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;

        let zone_name = Self::normalize_non_empty(body.zone_name.clone())
            .or_else(|| Self::normalize_non_empty(body.name.clone()))
            .ok_or_else(|| {
                ApiError::ValidationError("Le champ Zone / Service est obligatoire".to_string())
            })?;
        let site_name = Self::normalize_non_empty(body.site_name.clone()).ok_or_else(|| {
            ApiError::ValidationError("Le champ Site est obligatoire".to_string())
        })?;
        let building_name = Self::normalize_non_empty(body.building_name.clone())
            .or_else(|| Self::normalize_non_empty(body.building.clone()))
            .ok_or_else(|| {
                ApiError::ValidationError("Le champ Batiment est obligatoire".to_string())
            })?;
        let level_name = Self::normalize_non_empty(body.level_name.clone())
            .or_else(|| Self::normalize_non_empty(body.floor.clone()))
            .ok_or_else(|| {
                ApiError::ValidationError("Le champ Niveau est obligatoire".to_string())
            })?;
        let subzone_name = Self::normalize_non_empty(body.subzone_name.clone());

        let site_id = Self::normalize_identifier(body.site_id.clone())
            .unwrap_or_else(|| Self::make_stable_id("SITE", &site_name, "SITE-DEFAULT"));
        let building_id = Self::normalize_identifier(body.building_id.clone())
            .unwrap_or_else(|| Self::make_stable_id("BAT", &building_name, "BAT-DEFAULT"));
        let level_id = Self::normalize_identifier(body.level_id.clone())
            .unwrap_or_else(|| Self::make_stable_id("NIV", &level_name, "NIV-DEFAULT"));
        let zone_id = Self::normalize_identifier(body.zone_id.clone())
            .unwrap_or_else(|| Self::make_stable_id("ZONE", &zone_name, "ZONE-DEFAULT"));
        let subzone_id = if let Some(subzone) = subzone_name.clone() {
            Some(
                Self::normalize_identifier(body.subzone_id.clone())
                    .unwrap_or_else(|| Self::make_stable_id("SUB", &subzone, "SUB-DEFAULT")),
            )
        } else {
            None
        };

        let payload = NewReferentialService {
            id: format!("SVC-{}", Uuid::new_v4()),
            name: zone_name.clone(),
            building: Some(building_name.clone()),
            floor: Some(level_name.clone()),
            site_id,
            site_name,
            building_id,
            building_name,
            level_id,
            level_name,
            zone_id,
            zone_name,
            subzone_id,
            subzone_name,
        };

        ReferentialRepository::insert_service(&mut conn, &payload)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to create service: {}", e)))
    }

    pub fn update_service(
        pool: &DbPool,
        user: &User,
        service_id: &str,
        body: UpdateReferentialServiceRequest,
    ) -> ApiResult<ReferentialService> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;

        let existing = ReferentialRepository::find_service_by_id(&mut conn, service_id)
            .map_err(|_| ApiError::NotFound(format!("Service {} not found", service_id)))?;

        let zone_name = Self::normalize_non_empty(body.zone_name.clone())
            .or_else(|| Self::normalize_non_empty(body.name.clone()))
            .unwrap_or_else(|| existing.zone_name.clone());
        let site_name = Self::normalize_non_empty(body.site_name.clone())
            .unwrap_or_else(|| existing.site_name.clone());
        let building_name = Self::normalize_non_empty(body.building_name.clone())
            .or_else(|| Self::normalize_non_empty(body.building.clone()))
            .unwrap_or_else(|| existing.building_name.clone());
        let level_name = Self::normalize_non_empty(body.level_name.clone())
            .or_else(|| Self::normalize_non_empty(body.floor.clone()))
            .unwrap_or_else(|| existing.level_name.clone());
        let site_name = Self::ensure_required_hierarchy_label(site_name, "Site")?;
        let building_name = Self::ensure_required_hierarchy_label(building_name, "Batiment")?;
        let level_name = Self::ensure_required_hierarchy_label(level_name, "Niveau")?;
        let zone_name = Self::ensure_required_hierarchy_label(zone_name, "Zone / Service")?;
        let subzone_name = match body.subzone_name.clone() {
            Some(v) => Self::normalize_non_empty(Some(v)),
            None => existing.subzone_name.clone(),
        };

        let site_id = Self::normalize_identifier(body.site_id.clone())
            .unwrap_or_else(|| Self::make_stable_id("SITE", &site_name, existing.site_id.as_str()));
        let building_id =
            Self::normalize_identifier(body.building_id.clone()).unwrap_or_else(|| {
                Self::make_stable_id("BAT", &building_name, existing.building_id.as_str())
            });
        let level_id = Self::normalize_identifier(body.level_id.clone()).unwrap_or_else(|| {
            Self::make_stable_id("NIV", &level_name, existing.level_id.as_str())
        });
        let zone_id = Self::normalize_identifier(body.zone_id.clone())
            .unwrap_or_else(|| Self::make_stable_id("ZONE", &zone_name, existing.zone_id.as_str()));
        let subzone_id = if let Some(subzone) = subzone_name.clone() {
            Some(
                Self::normalize_identifier(body.subzone_id.clone()).unwrap_or_else(|| {
                    Self::make_stable_id(
                        "SUB",
                        &subzone,
                        existing.subzone_id.as_deref().unwrap_or("SUB-DEFAULT"),
                    )
                }),
            )
        } else {
            None
        };

        let data = ServiceUpdateData {
            name: zone_name.clone(),
            building: Some(building_name.clone()),
            floor: Some(level_name.clone()),
            site_id,
            site_name,
            building_id,
            building_name,
            level_id,
            level_name,
            zone_id,
            zone_name,
            subzone_id,
            subzone_name,
            is_active: body.is_active.unwrap_or(existing.is_active),
        };

        ReferentialRepository::update_service(&mut conn, service_id, data)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to update service: {}", e)))
    }

    pub fn deactivate_service(
        pool: &DbPool,
        user: &User,
        service_id: &str,
    ) -> ApiResult<ReferentialService> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::deactivate_service(&mut conn, service_id).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to deactivate service: {}", e))
        })
    }

    pub fn hard_delete_service(pool: &DbPool, user: &User, service_id: &str) -> ApiResult<Value> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let deleted =
            ReferentialRepository::hard_delete_service(&mut conn, service_id).map_err(|e| {
                ApiError::InternalServerError(format!(
                    "Failed to permanently delete service: {}",
                    e
                ))
            })?;
        if deleted == 0 {
            return Err(ApiError::NotFound(format!(
                "Service {} not found",
                service_id
            )));
        }
        Ok(json!({ "success": true, "deleted": true, "id": service_id }))
    }

    pub fn list_equipment_admin(
        pool: &DbPool,
        user: &User,
    ) -> ApiResult<Vec<ReferentialEquipment>> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_equipment(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load equipment: {}", e)))
    }

    pub fn list_equipment_active(pool: &DbPool) -> ApiResult<Vec<ReferentialEquipment>> {
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_active_equipment(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load equipment: {}", e)))
    }

    pub fn create_equipment(
        pool: &DbPool,
        user: &User,
        body: CreateReferentialEquipmentRequest,
    ) -> ApiResult<ReferentialEquipment> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let payload = NewReferentialEquipment {
            id: format!("EQUIP-{}", Uuid::new_v4()),
            label: body.label,
            sizes: Self::map_sizes(body.sizes),
            required_fields: body
                .required_fields
                .unwrap_or_else(Self::default_required_fields),
        };
        ReferentialRepository::insert_equipment(&mut conn, &payload).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to create equipment: {}", e))
        })
    }

    pub fn update_equipment(
        pool: &DbPool,
        user: &User,
        equipment_id: &str,
        body: UpdateReferentialEquipmentRequest,
    ) -> ApiResult<ReferentialEquipment> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let existing = ReferentialRepository::find_equipment_by_id(&mut conn, equipment_id)
            .map_err(|_| ApiError::NotFound(format!("Equipment {} not found", equipment_id)))?;

        let sizes = body
            .sizes
            .map(|sizes| sizes.into_iter().map(Some).collect())
            .or(existing.sizes.clone());
        let required_fields = body
            .required_fields
            .unwrap_or(existing.required_fields.clone());
        let data = EquipmentUpdateData {
            label: body.label.unwrap_or(existing.label),
            sizes,
            required_fields,
            is_active: body.is_active.unwrap_or(existing.is_active),
        };

        ReferentialRepository::update_equipment(&mut conn, equipment_id, data).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to update equipment: {}", e))
        })
    }

    pub fn deactivate_equipment(
        pool: &DbPool,
        user: &User,
        equipment_id: &str,
    ) -> ApiResult<ReferentialEquipment> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::deactivate_equipment(&mut conn, equipment_id).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to deactivate equipment: {}", e))
        })
    }

    pub fn hard_delete_equipment(
        pool: &DbPool,
        user: &User,
        equipment_id: &str,
    ) -> ApiResult<Value> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let deleted = ReferentialRepository::hard_delete_equipment(&mut conn, equipment_id)
            .map_err(|e| {
                ApiError::InternalServerError(format!(
                    "Failed to permanently delete equipment: {}",
                    e
                ))
            })?;
        if deleted == 0 {
            return Err(ApiError::NotFound(format!(
                "Equipment {} not found",
                equipment_id
            )));
        }
        Ok(json!({ "success": true, "deleted": true, "id": equipment_id }))
    }

    pub fn list_transport_modes_admin(
        pool: &DbPool,
        user: &User,
    ) -> ApiResult<Vec<ReferentialTransportMode>> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_transport_modes(&mut conn).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to load transport modes: {}", e))
        })
    }

    pub fn list_transport_modes_active(pool: &DbPool) -> ApiResult<Vec<ReferentialTransportMode>> {
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_active_transport_modes(&mut conn).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to load transport modes: {}", e))
        })
    }

    pub fn create_transport_mode(
        pool: &DbPool,
        user: &User,
        body: CreateReferentialTransportModeRequest,
    ) -> ApiResult<ReferentialTransportMode> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let max_sort_order = ReferentialRepository::max_transport_mode_sort_order(&mut conn)
            .ok()
            .flatten();
        let sort_order = body.sort_order.unwrap_or(max_sort_order.unwrap_or(0) + 1);

        let payload = NewReferentialTransportMode {
            id: format!("TM-{}", Uuid::new_v4()),
            label: body.label,
            sort_order,
        };
        ReferentialRepository::insert_transport_mode(&mut conn, &payload).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to create transport mode: {}", e))
        })
    }

    pub fn update_transport_mode(
        pool: &DbPool,
        user: &User,
        mode_id: &str,
        body: UpdateReferentialTransportModeRequest,
    ) -> ApiResult<ReferentialTransportMode> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let existing = ReferentialRepository::find_transport_mode_by_id(&mut conn, mode_id)
            .map_err(|_| ApiError::NotFound(format!("Transport mode {} not found", mode_id)))?;
        let data = TransportModeUpdateData {
            label: body.label.unwrap_or(existing.label),
            sort_order: body.sort_order.unwrap_or(existing.sort_order),
            is_active: body.is_active.unwrap_or(existing.is_active),
        };
        ReferentialRepository::update_transport_mode(&mut conn, mode_id, data).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to update transport mode: {}", e))
        })
    }

    pub fn deactivate_transport_mode(
        pool: &DbPool,
        user: &User,
        mode_id: &str,
    ) -> ApiResult<ReferentialTransportMode> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::deactivate_transport_mode(&mut conn, mode_id).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to deactivate transport mode: {}", e))
        })
    }

    pub fn hard_delete_transport_mode(
        pool: &DbPool,
        user: &User,
        mode_id: &str,
    ) -> ApiResult<Value> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let deleted = ReferentialRepository::hard_delete_transport_mode(&mut conn, mode_id)
            .map_err(|e| {
                ApiError::InternalServerError(format!(
                    "Failed to permanently delete transport mode: {}",
                    e
                ))
            })?;
        if deleted == 0 {
            return Err(ApiError::NotFound(format!(
                "Transport mode {} not found",
                mode_id
            )));
        }
        Ok(json!({ "success": true, "deleted": true, "id": mode_id }))
    }

    pub fn list_specimens_admin(pool: &DbPool, user: &User) -> ApiResult<Vec<ReferentialSpecimen>> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_specimens(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load specimens: {}", e)))
    }

    pub fn list_specimens_active(pool: &DbPool) -> ApiResult<Vec<ReferentialSpecimen>> {
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::list_active_specimens(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load specimens: {}", e)))
    }

    pub fn create_specimen(
        pool: &DbPool,
        user: &User,
        body: CreateReferentialSpecimenRequest,
    ) -> ApiResult<ReferentialSpecimen> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let payload = NewReferentialSpecimen {
            id: format!("SPEC-{}", Uuid::new_v4()),
            label: body.label,
        };
        ReferentialRepository::insert_specimen(&mut conn, &payload)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to create specimen: {}", e)))
    }

    pub fn update_specimen(
        pool: &DbPool,
        user: &User,
        specimen_id: &str,
        body: UpdateReferentialSpecimenRequest,
    ) -> ApiResult<ReferentialSpecimen> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let existing = ReferentialRepository::find_specimen_by_id(&mut conn, specimen_id)
            .map_err(|_| ApiError::NotFound(format!("Specimen {} not found", specimen_id)))?;
        let data = SpecimenUpdateData {
            label: body.label.unwrap_or(existing.label),
            is_active: body.is_active.unwrap_or(existing.is_active),
        };
        ReferentialRepository::update_specimen(&mut conn, specimen_id, data)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to update specimen: {}", e)))
    }

    pub fn deactivate_specimen(
        pool: &DbPool,
        user: &User,
        specimen_id: &str,
    ) -> ApiResult<ReferentialSpecimen> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        ReferentialRepository::deactivate_specimen(&mut conn, specimen_id).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to deactivate specimen: {}", e))
        })
    }

    pub fn hard_delete_specimen(pool: &DbPool, user: &User, specimen_id: &str) -> ApiResult<Value> {
        require_admin(user)?;
        let mut conn = Self::conn(pool)?;
        let deleted =
            ReferentialRepository::hard_delete_specimen(&mut conn, specimen_id).map_err(|e| {
                ApiError::InternalServerError(format!(
                    "Failed to permanently delete specimen: {}",
                    e
                ))
            })?;
        if deleted == 0 {
            return Err(ApiError::NotFound(format!(
                "Specimen {} not found",
                specimen_id
            )));
        }
        Ok(json!({ "success": true, "deleted": true, "id": specimen_id }))
    }

    fn conn(pool: &DbPool) -> ApiResult<PooledConnection<ConnectionManager<PgConnection>>> {
        pool.get()
            .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))
    }

    fn default_required_fields() -> serde_json::Value {
        json!({
            "recipient": false,
            "priority": true,
            "origin": true,
            "destination": true,
            "note_free": false,
            "scheduled": false
        })
    }

    fn map_sizes(input: Option<Vec<String>>) -> Option<Vec<Option<String>>> {
        input.map(|sizes| sizes.into_iter().map(Some).collect())
    }

    fn normalize_non_empty(value: Option<String>) -> Option<String> {
        value
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    }

    fn normalize_identifier(value: Option<String>) -> Option<String> {
        value
            .map(|v| v.trim().to_uppercase())
            .filter(|v| !v.is_empty())
    }

    fn ensure_required_hierarchy_label(value: String, field_label: &str) -> ApiResult<String> {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            Err(ApiError::ValidationError(format!(
                "Le champ {} est obligatoire",
                field_label
            )))
        } else {
            Ok(trimmed)
        }
    }

    fn make_stable_id(prefix: &str, source: &str, fallback: &str) -> String {
        let mut normalized = String::with_capacity(source.len());
        for ch in source.chars() {
            if ch.is_ascii_alphanumeric() {
                normalized.push(ch.to_ascii_uppercase());
            } else {
                normalized.push('-');
            }
        }

        let compact = normalized
            .split('-')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        if compact.is_empty() {
            fallback.to_string()
        } else {
            format!("{}-{}", prefix, compact)
        }
    }
}
