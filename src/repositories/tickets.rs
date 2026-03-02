use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::PgConnection;

use crate::models::{NewTicket, Ticket};
use crate::schema::{ticket_assignments, tickets, users};

#[derive(Debug, Clone)]
pub struct TicketListFilters {
    pub include_archived: bool,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub porter_id: Option<String>,
    pub transport_type: Option<String>,
    pub visible_to_porters_only: bool,
    pub requester_scope: Option<(String, String)>,
    pub limit: i64,
    pub offset: i64,
}

pub struct TicketRepository;

impl TicketRepository {
    pub fn lock_ticket_id_generation(conn: &mut PgConnection) -> QueryResult<usize> {
        sql_query("SELECT pg_advisory_xact_lock(987654321)").execute(conn)
    }

    pub fn count_by_id_pattern(conn: &mut PgConnection, pattern: &str) -> QueryResult<i64> {
        tickets::table
            .filter(tickets::id.like(pattern))
            .select(diesel::dsl::count_star())
            .first(conn)
    }

    pub fn insert(conn: &mut PgConnection, payload: &NewTicket) -> QueryResult<Ticket> {
        diesel::insert_into(tickets::table)
            .values(payload)
            .get_result::<Ticket>(conn)
    }

    pub fn activate_programmed_tickets_visibility(conn: &mut PgConnection) -> QueryResult<usize> {
        sql_query(
            "UPDATE tickets \
             SET is_visible_to_porters = true \
             WHERE is_visible_to_porters = false \
               AND scheduled_time IS NOT NULL \
               AND activation_minutes_before IS NOT NULL \
               AND NOW() >= scheduled_time - (activation_minutes_before || ' minutes')::interval",
        )
        .execute(conn)
    }

    pub fn list(conn: &mut PgConnection, filters: &TicketListFilters) -> QueryResult<Vec<Ticket>> {
        let mut query_builder = tickets::table.into_boxed();

        if !filters.include_archived {
            query_builder = query_builder.filter(tickets::is_archived.eq(false));
        }
        if filters.visible_to_porters_only {
            query_builder = query_builder.filter(tickets::is_visible_to_porters.eq(true));
        }
        if let Some((requester_id, requester_username)) = &filters.requester_scope {
            query_builder = query_builder.filter(
                tickets::requester_id
                    .eq(requester_id.as_str())
                    .or(tickets::requester_id.eq(requester_username.as_str())),
            );
        }
        if let Some(status) = &filters.status {
            query_builder = query_builder.filter(tickets::status.eq(status));
        }
        if let Some(priority) = filters.priority {
            query_builder = query_builder.filter(tickets::priority.eq(priority));
        }
        if let Some(porter_id) = &filters.porter_id {
            query_builder = query_builder.filter(tickets::porter_id.eq(porter_id));
        }
        if let Some(transport_type) = &filters.transport_type {
            query_builder = query_builder.filter(tickets::transport_type.eq(transport_type));
        }

        query_builder
            .order(tickets::created_at.desc())
            .limit(filters.limit)
            .offset(filters.offset)
            .load::<Ticket>(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, ticket_id: &str) -> QueryResult<Ticket> {
        tickets::table.find(ticket_id).first::<Ticket>(conn)
    }

    pub fn list_by_requester(
        conn: &mut PgConnection,
        requester_id: &str,
    ) -> QueryResult<Vec<Ticket>> {
        tickets::table
            .filter(tickets::requester_id.eq(requester_id))
            .load::<Ticket>(conn)
    }

    pub fn anonymize_requester(
        conn: &mut PgConnection,
        requester_id: &str,
        replacement: &str,
    ) -> QueryResult<usize> {
        diesel::update(tickets::table.filter(tickets::requester_id.eq(requester_id)))
            .set(tickets::requester_id.eq(replacement))
            .execute(conn)
    }

    pub fn update_status(
        conn: &mut PgConnection,
        ticket_id: &str,
        status: &str,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set(tickets::status.eq(status))
            .get_result::<Ticket>(conn)
    }

    pub fn update_status_completed_and_archive(
        conn: &mut PgConnection,
        ticket_id: &str,
        status: &str,
        at: NaiveDateTime,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::status.eq(status),
                tickets::completed_at.eq(Some(at)),
                tickets::is_archived.eq(true),
                tickets::archived_at.eq(Some(at)),
            ))
            .get_result::<Ticket>(conn)
    }

    pub fn update_status_canceled_and_archive(
        conn: &mut PgConnection,
        ticket_id: &str,
        status: &str,
        at: NaiveDateTime,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::status.eq(status),
                tickets::is_archived.eq(true),
                tickets::archived_at.eq(Some(at)),
            ))
            .get_result::<Ticket>(conn)
    }

    pub fn delete_by_id(conn: &mut PgConnection, ticket_id: &str) -> QueryResult<usize> {
        diesel::delete(tickets::table.find(ticket_id)).execute(conn)
    }

    pub fn assign_to_porter_and_clear_reservation(
        conn: &mut PgConnection,
        ticket_id: &str,
        porter_id: &str,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::status.eq("assigned"),
                tickets::porter_id.eq(porter_id),
                tickets::reservation_locked_by.eq::<Option<String>>(None),
                tickets::reservation_locked_at.eq::<Option<chrono::NaiveDateTime>>(None),
            ))
            .get_result::<Ticket>(conn)
    }

    pub fn set_porter_and_status(
        conn: &mut PgConnection,
        ticket_id: &str,
        porter_id: &str,
        status: &str,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set((tickets::porter_id.eq(porter_id), tickets::status.eq(status)))
            .get_result::<Ticket>(conn)
    }

    pub fn clear_porter_and_set_status(
        conn: &mut PgConnection,
        ticket_id: &str,
        status: &str,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::porter_id.eq::<Option<String>>(None),
                tickets::status.eq(status),
            ))
            .get_result::<Ticket>(conn)
    }

    pub fn set_help_request_pending(
        conn: &mut PgConnection,
        ticket_id: &str,
        help_porter_id: &str,
        requested_at: NaiveDateTime,
    ) -> QueryResult<usize> {
        diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::help_requested.eq(Some(true)),
                tickets::help_porter_id.eq(Some(help_porter_id)),
                tickets::help_status.eq(Some("pending")),
                tickets::help_requested_at.eq(Some(requested_at)),
            ))
            .execute(conn)
    }

    pub fn set_help_status(
        conn: &mut PgConnection,
        ticket_id: &str,
        status: &str,
    ) -> QueryResult<usize> {
        diesel::update(tickets::table.find(ticket_id))
            .set(tickets::help_status.eq(Some(status)))
            .execute(conn)
    }

    pub fn update_equipment_status(
        conn: &mut PgConnection,
        ticket_id: &str,
        delivered: bool,
        label_returned: bool,
    ) -> QueryResult<usize> {
        diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::equipment_delivered.eq(Some(delivered)),
                tickets::equipment_label_returned.eq(Some(label_returned)),
            ))
            .execute(conn)
    }

    pub fn update_notes(
        conn: &mut PgConnection,
        ticket_id: &str,
        notes: Option<String>,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set(tickets::notes.eq(notes))
            .get_result::<Ticket>(conn)
    }

    pub fn override_priority_with_notes(
        conn: &mut PgConnection,
        ticket_id: &str,
        priority: i32,
        notes: String,
        updated_at: NaiveDateTime,
    ) -> QueryResult<Ticket> {
        diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::priority.eq(priority),
                tickets::notes.eq(Some(notes)),
                tickets::updated_at.eq(updated_at),
            ))
            .get_result::<Ticket>(conn)
    }

    pub fn find_active_assignments(
        conn: &mut PgConnection,
        ticket_ids: &[String],
    ) -> QueryResult<Vec<(String, String, String)>> {
        if ticket_ids.is_empty() {
            return Ok(Vec::new());
        }

        ticket_assignments::table
            .filter(ticket_assignments::ticket_id.eq_any(ticket_ids))
            .filter(ticket_assignments::is_active.eq(true))
            .order((
                ticket_assignments::ticket_id.asc(),
                ticket_assignments::assigned_at.asc(),
            ))
            .select((
                ticket_assignments::ticket_id,
                ticket_assignments::porter_id,
                ticket_assignments::role,
            ))
            .load::<(String, String, String)>(conn)
    }

    pub fn find_requesters(
        conn: &mut PgConnection,
        requester_ids: &[String],
    ) -> QueryResult<Vec<(String, String, String, String)>> {
        if requester_ids.is_empty() {
            return Ok(Vec::new());
        }

        users::table
            .filter(users::id.eq_any(requester_ids))
            .select((
                users::id,
                users::username,
                users::first_name,
                users::last_name,
            ))
            .load::<(String, String, String, String)>(conn)
    }
}
