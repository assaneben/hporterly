use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::{HelpRequest, NewHelpRequest};
use crate::schema::help_requests;

pub struct HelpRequestRepository;

impl HelpRequestRepository {
    pub fn insert(conn: &mut PgConnection, payload: &NewHelpRequest) -> QueryResult<usize> {
        diesel::insert_into(help_requests::table).values(payload).execute(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, id: &str) -> QueryResult<HelpRequest> {
        help_requests::table.find(id).first::<HelpRequest>(conn)
    }

    pub fn set_status_with_responded_at(
        conn: &mut PgConnection,
        id: &str,
        status: &str,
        responded_at: NaiveDateTime,
    ) -> QueryResult<usize> {
        diesel::update(help_requests::table.find(id))
            .set((
                help_requests::status.eq(status),
                help_requests::responded_at.eq(Some(responded_at)),
            ))
            .execute(conn)
    }

    pub fn list_pending_for_requested_porter(
        conn: &mut PgConnection,
        porter_id: &str,
    ) -> QueryResult<Vec<HelpRequest>> {
        help_requests::table
            .filter(help_requests::requested_porter_id.eq(porter_id))
            .filter(help_requests::status.eq("pending"))
            .load::<HelpRequest>(conn)
    }
}
