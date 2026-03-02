use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::Service;
use crate::schema::services;

pub struct HospitalServiceRepository;

impl HospitalServiceRepository {
    pub fn list_all(conn: &mut PgConnection) -> QueryResult<Vec<Service>> {
        services::table.order(services::name.asc()).load::<Service>(conn)
    }
}
