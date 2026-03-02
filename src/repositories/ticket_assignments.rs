use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::{NewTicketAssignment, Ticket, TicketAssignment};
use crate::schema::{porters, ticket_assignments, tickets};

pub struct TicketAssignmentRepository;

impl TicketAssignmentRepository {
    pub fn find_porter_id_by_user_id(
        conn: &mut PgConnection,
        user_id: &str,
    ) -> QueryResult<String> {
        porters::table
            .filter(porters::user_id.eq(user_id))
            .select(porters::id)
            .first::<String>(conn)
    }

    pub fn find_active_supervisor_assignment(
        conn: &mut PgConnection,
        ticket_id: &str,
    ) -> QueryResult<Option<TicketAssignment>> {
        ticket_assignments::table
            .filter(ticket_assignments::ticket_id.eq(ticket_id))
            .filter(ticket_assignments::role.eq("supervisor"))
            .filter(ticket_assignments::is_active.eq(true))
            .first::<TicketAssignment>(conn)
            .optional()
    }

    pub fn find_active_assignment_for_porter(
        conn: &mut PgConnection,
        ticket_id: &str,
        porter_id: &str,
    ) -> QueryResult<Option<TicketAssignment>> {
        ticket_assignments::table
            .filter(ticket_assignments::ticket_id.eq(ticket_id))
            .filter(ticket_assignments::porter_id.eq(porter_id))
            .filter(ticket_assignments::is_active.eq(true))
            .first::<TicketAssignment>(conn)
            .optional()
    }

    pub fn insert_assignment(
        conn: &mut PgConnection,
        assignment: &NewTicketAssignment,
    ) -> QueryResult<TicketAssignment> {
        diesel::insert_into(ticket_assignments::table)
            .values(assignment)
            .get_result::<TicketAssignment>(conn)
    }

    pub fn promote_assignment_to_supervisor(
        conn: &mut PgConnection,
        assignment_id: &str,
    ) -> QueryResult<TicketAssignment> {
        diesel::update(ticket_assignments::table.find(assignment_id))
            .set(ticket_assignments::role.eq("supervisor"))
            .get_result::<TicketAssignment>(conn)
    }

    pub fn deactivate_assignment(
        conn: &mut PgConnection,
        assignment_id: &str,
        removed_at: NaiveDateTime,
    ) -> QueryResult<TicketAssignment> {
        diesel::update(ticket_assignments::table.find(assignment_id))
            .set((
                ticket_assignments::is_active.eq(false),
                ticket_assignments::removed_at.eq(Some(removed_at)),
            ))
            .get_result::<TicketAssignment>(conn)
    }

    pub fn find_active_co_partners(
        conn: &mut PgConnection,
        ticket_id: &str,
    ) -> QueryResult<Vec<TicketAssignment>> {
        ticket_assignments::table
            .filter(ticket_assignments::ticket_id.eq(ticket_id))
            .filter(ticket_assignments::role.eq("co_partner"))
            .filter(ticket_assignments::is_active.eq(true))
            .order(ticket_assignments::assigned_at.asc())
            .load::<TicketAssignment>(conn)
    }

    pub fn deactivate_active_co_partner(
        conn: &mut PgConnection,
        ticket_id: &str,
        porter_id: &str,
        removed_at: NaiveDateTime,
    ) -> QueryResult<TicketAssignment> {
        diesel::update(
            ticket_assignments::table
                .filter(ticket_assignments::ticket_id.eq(ticket_id))
                .filter(ticket_assignments::porter_id.eq(porter_id))
                .filter(ticket_assignments::role.eq("co_partner"))
                .filter(ticket_assignments::is_active.eq(true)),
        )
        .set((
            ticket_assignments::is_active.eq(false),
            ticket_assignments::removed_at.eq(Some(removed_at)),
        ))
        .get_result::<TicketAssignment>(conn)
    }

    pub fn deactivate_all_active_for_ticket(
        conn: &mut PgConnection,
        ticket_id: &str,
        removed_at: NaiveDateTime,
    ) -> QueryResult<usize> {
        diesel::update(
            ticket_assignments::table
                .filter(ticket_assignments::ticket_id.eq(ticket_id))
                .filter(ticket_assignments::is_active.eq(true)),
        )
        .set((
            ticket_assignments::is_active.eq(false),
            ticket_assignments::removed_at.eq(Some(removed_at)),
        ))
        .execute(conn)
    }

    pub fn has_active_supervisor_elsewhere(
        conn: &mut PgConnection,
        porter_id: &str,
        exclude_ticket_id: &str,
    ) -> QueryResult<bool> {
        let active_statuses = ["assigned", "in_progress", "arrived", "suspended", "paused"];

        let active_ticket = tickets::table
            .filter(tickets::porter_id.eq(porter_id))
            .filter(tickets::status.eq_any(active_statuses))
            .filter(tickets::id.ne(exclude_ticket_id))
            .first::<Ticket>(conn)
            .optional()?;
        if active_ticket.is_some() {
            return Ok(true);
        }

        let active_assignment = ticket_assignments::table
            .inner_join(tickets::table.on(tickets::id.eq(ticket_assignments::ticket_id)))
            .filter(ticket_assignments::porter_id.eq(porter_id))
            .filter(ticket_assignments::role.eq("supervisor"))
            .filter(ticket_assignments::is_active.eq(true))
            .filter(ticket_assignments::ticket_id.ne(exclude_ticket_id))
            .filter(tickets::status.eq_any(active_statuses))
            .select(ticket_assignments::all_columns)
            .first::<TicketAssignment>(conn)
            .optional()?;

        Ok(active_assignment.is_some())
    }
}
