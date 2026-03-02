use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::Patient;
use crate::schema::patients;

pub struct PatientRepository;

impl PatientRepository {
    pub fn search(
        conn: &mut PgConnection,
        search_term: &str,
        limit: i64,
    ) -> QueryResult<Vec<Patient>> {
        patients::table
            .filter(
                patients::last_name
                    .ilike(search_term)
                    .or(patients::first_name.ilike(search_term))
                    .or(patients::id.ilike(search_term))
                    .or(patients::service.ilike(search_term)),
            )
            .limit(limit)
            .order(patients::last_name.asc())
            .load::<Patient>(conn)
    }
}
