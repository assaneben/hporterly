use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::{NewPorter, Porter};
use crate::schema::porters;

pub struct PorterRepository;

impl PorterRepository {
    pub fn list_ordered_by_status(conn: &mut PgConnection) -> QueryResult<Vec<Porter>> {
        porters::table.order(porters::status.asc()).load::<Porter>(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, porter_id: &str) -> QueryResult<Porter> {
        porters::table.find(porter_id).first::<Porter>(conn)
    }

    pub fn find_by_id_optional(
        conn: &mut PgConnection,
        porter_id: &str,
    ) -> QueryResult<Option<Porter>> {
        porters::table.find(porter_id).first::<Porter>(conn).optional()
    }

    pub fn find_by_user_id(conn: &mut PgConnection, user_id: &str) -> QueryResult<Porter> {
        porters::table.filter(porters::user_id.eq(user_id)).first::<Porter>(conn)
    }

    pub fn find_by_user_id_optional(
        conn: &mut PgConnection,
        user_id: &str,
    ) -> QueryResult<Option<Porter>> {
        porters::table.filter(porters::user_id.eq(user_id)).first::<Porter>(conn).optional()
    }

    pub fn find_id_by_user_id_optional(
        conn: &mut PgConnection,
        user_id: &str,
    ) -> QueryResult<Option<String>> {
        porters::table
            .filter(porters::user_id.eq(user_id))
            .select(porters::id)
            .first::<String>(conn)
            .optional()
    }

    pub fn exists_for_user_and_porter_id(
        conn: &mut PgConnection,
        porter_id: &str,
        user_id: &str,
    ) -> QueryResult<bool> {
        let match_count = porters::table
            .filter(porters::id.eq(porter_id))
            .filter(porters::user_id.eq(user_id))
            .select(diesel::dsl::count_star())
            .first::<i64>(conn)?;

        Ok(match_count > 0)
    }

    pub fn set_status(
        conn: &mut PgConnection,
        porter_id: &str,
        status: &str,
    ) -> QueryResult<usize> {
        diesel::update(porters::table.find(porter_id)).set(porters::status.eq(status)).execute(conn)
    }

    pub fn set_status_by_user_id(
        conn: &mut PgConnection,
        user_id: &str,
        status: &str,
    ) -> QueryResult<usize> {
        diesel::update(porters::table.filter(porters::user_id.eq(user_id)))
            .set(porters::status.eq(status))
            .execute(conn)
    }

    pub fn update_status_and_location(
        conn: &mut PgConnection,
        porter_id: &str,
        status: &str,
        location: Option<String>,
    ) -> QueryResult<Porter> {
        diesel::update(porters::table.find(porter_id))
            .set((porters::status.eq(status), porters::current_location.eq(location)))
            .get_result::<Porter>(conn)
    }

    pub fn update_skills(
        conn: &mut PgConnection,
        porter_id: &str,
        skills: Vec<Option<String>>,
    ) -> QueryResult<Porter> {
        diesel::update(porters::table.find(porter_id))
            .set(porters::skills.eq(skills))
            .get_result::<Porter>(conn)
    }

    pub fn insert(conn: &mut PgConnection, payload: &NewPorter) -> QueryResult<Porter> {
        diesel::insert_into(porters::table).values(payload).get_result::<Porter>(conn)
    }

    pub fn increment_completed_stats_and_set_available(
        conn: &mut PgConnection,
        porter_id: &str,
    ) -> QueryResult<usize> {
        diesel::update(porters::table.find(porter_id))
            .set((
                porters::completed_missions_today.eq(porters::completed_missions_today + 1),
                porters::total_missions.eq(porters::total_missions + 1),
                porters::status.eq("available"),
            ))
            .execute(conn)
    }

    pub fn list_available(conn: &mut PgConnection) -> QueryResult<Vec<Porter>> {
        porters::table.filter(porters::status.eq("available")).load::<Porter>(conn)
    }

    pub fn list_available_excluding(
        conn: &mut PgConnection,
        excluded_porter_id: Option<&str>,
    ) -> QueryResult<Vec<Porter>> {
        let mut query = porters::table.filter(porters::status.eq("available")).into_boxed();
        if let Some(excluded) = excluded_porter_id.filter(|id| !id.is_empty()) {
            query = query.filter(porters::id.ne(excluded));
        }
        query.load::<Porter>(conn)
    }
}
