use diesel::dsl::max;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::{
    NewReferentialEquipment, NewReferentialService, NewReferentialSpecimen,
    NewReferentialTransportMode, ReferentialEquipment, ReferentialService, ReferentialSpecimen,
    ReferentialTransportMode,
};
use crate::schema::{
    referential_equipment, referential_services, referential_specimens, referential_transport_modes,
};

pub struct ServiceUpdateData {
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
    pub is_active: bool,
}

pub struct EquipmentUpdateData {
    pub label: String,
    pub sizes: Option<Vec<Option<String>>>,
    pub required_fields: serde_json::Value,
    pub is_active: bool,
}

pub struct TransportModeUpdateData {
    pub label: String,
    pub sort_order: i32,
    pub is_active: bool,
}

pub struct SpecimenUpdateData {
    pub label: String,
    pub is_active: bool,
}

pub struct ReferentialRepository;

impl ReferentialRepository {
    pub fn list_services(conn: &mut PgConnection) -> QueryResult<Vec<ReferentialService>> {
        referential_services::table
            .order((
                referential_services::site_name.asc(),
                referential_services::building_name.asc(),
                referential_services::level_name.asc(),
                referential_services::zone_name.asc(),
                referential_services::subzone_name.asc(),
            ))
            .load::<ReferentialService>(conn)
    }

    pub fn list_active_services(conn: &mut PgConnection) -> QueryResult<Vec<ReferentialService>> {
        referential_services::table
            .filter(referential_services::is_active.eq(true))
            .order((
                referential_services::site_name.asc(),
                referential_services::building_name.asc(),
                referential_services::level_name.asc(),
                referential_services::zone_name.asc(),
                referential_services::subzone_name.asc(),
            ))
            .load::<ReferentialService>(conn)
    }

    pub fn find_service_by_id(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialService> {
        referential_services::table
            .find(id)
            .first::<ReferentialService>(conn)
    }

    pub fn insert_service(
        conn: &mut PgConnection,
        payload: &NewReferentialService,
    ) -> QueryResult<ReferentialService> {
        diesel::insert_into(referential_services::table)
            .values(payload)
            .get_result::<ReferentialService>(conn)
    }

    pub fn update_service(
        conn: &mut PgConnection,
        id: &str,
        data: ServiceUpdateData,
    ) -> QueryResult<ReferentialService> {
        diesel::update(referential_services::table.find(id))
            .set((
                referential_services::name.eq(data.name),
                referential_services::building.eq(data.building),
                referential_services::floor.eq(data.floor),
                referential_services::site_id.eq(data.site_id),
                referential_services::site_name.eq(data.site_name),
                referential_services::building_id.eq(data.building_id),
                referential_services::building_name.eq(data.building_name),
                referential_services::level_id.eq(data.level_id),
                referential_services::level_name.eq(data.level_name),
                referential_services::zone_id.eq(data.zone_id),
                referential_services::zone_name.eq(data.zone_name),
                referential_services::subzone_id.eq(data.subzone_id),
                referential_services::subzone_name.eq(data.subzone_name),
                referential_services::is_active.eq(data.is_active),
            ))
            .get_result::<ReferentialService>(conn)
    }

    pub fn deactivate_service(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialService> {
        diesel::update(referential_services::table.find(id))
            .set(referential_services::is_active.eq(false))
            .get_result::<ReferentialService>(conn)
    }

    pub fn hard_delete_service(conn: &mut PgConnection, id: &str) -> QueryResult<usize> {
        diesel::delete(referential_services::table.find(id)).execute(conn)
    }

    pub fn list_equipment(conn: &mut PgConnection) -> QueryResult<Vec<ReferentialEquipment>> {
        referential_equipment::table
            .order(referential_equipment::label.asc())
            .load::<ReferentialEquipment>(conn)
    }

    pub fn list_active_equipment(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<ReferentialEquipment>> {
        referential_equipment::table
            .filter(referential_equipment::is_active.eq(true))
            .order(referential_equipment::label.asc())
            .load::<ReferentialEquipment>(conn)
    }

    pub fn find_equipment_by_id(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialEquipment> {
        referential_equipment::table
            .find(id)
            .first::<ReferentialEquipment>(conn)
    }

    pub fn insert_equipment(
        conn: &mut PgConnection,
        payload: &NewReferentialEquipment,
    ) -> QueryResult<ReferentialEquipment> {
        diesel::insert_into(referential_equipment::table)
            .values(payload)
            .get_result::<ReferentialEquipment>(conn)
    }

    pub fn update_equipment(
        conn: &mut PgConnection,
        id: &str,
        data: EquipmentUpdateData,
    ) -> QueryResult<ReferentialEquipment> {
        diesel::update(referential_equipment::table.find(id))
            .set((
                referential_equipment::label.eq(data.label),
                referential_equipment::sizes.eq(data.sizes),
                referential_equipment::required_fields.eq(data.required_fields),
                referential_equipment::is_active.eq(data.is_active),
            ))
            .get_result::<ReferentialEquipment>(conn)
    }

    pub fn deactivate_equipment(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialEquipment> {
        diesel::update(referential_equipment::table.find(id))
            .set(referential_equipment::is_active.eq(false))
            .get_result::<ReferentialEquipment>(conn)
    }

    pub fn hard_delete_equipment(conn: &mut PgConnection, id: &str) -> QueryResult<usize> {
        diesel::delete(referential_equipment::table.find(id)).execute(conn)
    }

    pub fn list_transport_modes(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<ReferentialTransportMode>> {
        referential_transport_modes::table
            .order(referential_transport_modes::sort_order.asc())
            .load::<ReferentialTransportMode>(conn)
    }

    pub fn list_active_transport_modes(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<ReferentialTransportMode>> {
        referential_transport_modes::table
            .filter(referential_transport_modes::is_active.eq(true))
            .order(referential_transport_modes::sort_order.asc())
            .load::<ReferentialTransportMode>(conn)
    }

    pub fn max_transport_mode_sort_order(conn: &mut PgConnection) -> QueryResult<Option<i32>> {
        referential_transport_modes::table
            .select(max(referential_transport_modes::sort_order))
            .first(conn)
    }

    pub fn find_transport_mode_by_id(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialTransportMode> {
        referential_transport_modes::table
            .find(id)
            .first::<ReferentialTransportMode>(conn)
    }

    pub fn insert_transport_mode(
        conn: &mut PgConnection,
        payload: &NewReferentialTransportMode,
    ) -> QueryResult<ReferentialTransportMode> {
        diesel::insert_into(referential_transport_modes::table)
            .values(payload)
            .get_result::<ReferentialTransportMode>(conn)
    }

    pub fn update_transport_mode(
        conn: &mut PgConnection,
        id: &str,
        data: TransportModeUpdateData,
    ) -> QueryResult<ReferentialTransportMode> {
        diesel::update(referential_transport_modes::table.find(id))
            .set((
                referential_transport_modes::label.eq(data.label),
                referential_transport_modes::sort_order.eq(data.sort_order),
                referential_transport_modes::is_active.eq(data.is_active),
            ))
            .get_result::<ReferentialTransportMode>(conn)
    }

    pub fn deactivate_transport_mode(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialTransportMode> {
        diesel::update(referential_transport_modes::table.find(id))
            .set(referential_transport_modes::is_active.eq(false))
            .get_result::<ReferentialTransportMode>(conn)
    }

    pub fn hard_delete_transport_mode(conn: &mut PgConnection, id: &str) -> QueryResult<usize> {
        diesel::delete(referential_transport_modes::table.find(id)).execute(conn)
    }

    pub fn list_specimens(conn: &mut PgConnection) -> QueryResult<Vec<ReferentialSpecimen>> {
        referential_specimens::table
            .order(referential_specimens::label.asc())
            .load::<ReferentialSpecimen>(conn)
    }

    pub fn list_active_specimens(conn: &mut PgConnection) -> QueryResult<Vec<ReferentialSpecimen>> {
        referential_specimens::table
            .filter(referential_specimens::is_active.eq(true))
            .order(referential_specimens::label.asc())
            .load::<ReferentialSpecimen>(conn)
    }

    pub fn find_specimen_by_id(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialSpecimen> {
        referential_specimens::table
            .find(id)
            .first::<ReferentialSpecimen>(conn)
    }

    pub fn insert_specimen(
        conn: &mut PgConnection,
        payload: &NewReferentialSpecimen,
    ) -> QueryResult<ReferentialSpecimen> {
        diesel::insert_into(referential_specimens::table)
            .values(payload)
            .get_result::<ReferentialSpecimen>(conn)
    }

    pub fn update_specimen(
        conn: &mut PgConnection,
        id: &str,
        data: SpecimenUpdateData,
    ) -> QueryResult<ReferentialSpecimen> {
        diesel::update(referential_specimens::table.find(id))
            .set((
                referential_specimens::label.eq(data.label),
                referential_specimens::is_active.eq(data.is_active),
            ))
            .get_result::<ReferentialSpecimen>(conn)
    }

    pub fn deactivate_specimen(
        conn: &mut PgConnection,
        id: &str,
    ) -> QueryResult<ReferentialSpecimen> {
        diesel::update(referential_specimens::table.find(id))
            .set(referential_specimens::is_active.eq(false))
            .get_result::<ReferentialSpecimen>(conn)
    }

    pub fn hard_delete_specimen(conn: &mut PgConnection, id: &str) -> QueryResult<usize> {
        diesel::delete(referential_specimens::table.find(id)).execute(conn)
    }
}
