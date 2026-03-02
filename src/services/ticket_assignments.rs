use chrono::Utc;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models::{AssignmentRole, NewTicketAssignment, Ticket, TicketAssignment, User};
use crate::repositories::TicketAssignmentRepository;
use crate::utils::{ApiError, ApiResult};

pub struct TicketAssignmentService;

impl TicketAssignmentService {
    pub fn get_porter_id_for_user(conn: &mut PgConnection, user: &User) -> ApiResult<String> {
        TicketAssignmentRepository::find_porter_id_by_user_id(conn, &user.id).map_err(|_| {
            ApiError::Forbidden("Aucun profil brancardier associe a cet utilisateur".to_string())
        })
    }

    pub fn find_active_supervisor_assignment(
        conn: &mut PgConnection,
        ticket_id: &str,
    ) -> ApiResult<Option<TicketAssignment>> {
        TicketAssignmentRepository::find_active_supervisor_assignment(conn, ticket_id).map_err(
            |e| {
                ApiError::InternalServerError(format!(
                    "Failed to load supervisor assignment: {}",
                    e
                ))
            },
        )
    }

    pub fn ensure_supervisor_assignment(
        conn: &mut PgConnection,
        ticket_id: &str,
        porter_id: &str,
    ) -> ApiResult<TicketAssignment> {
        if let Some(existing) = TicketAssignmentRepository::find_active_assignment_for_porter(
            conn, ticket_id, porter_id,
        )
        .map_err(|e| ApiError::InternalServerError(format!("Failed to load assignment: {}", e)))?
        {
            if existing.role == AssignmentRole::CoPartner.to_string() {
                return TicketAssignmentRepository::promote_assignment_to_supervisor(
                    conn,
                    &existing.id,
                )
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Failed to promote co-partner: {}", e))
                });
            }
            return Ok(existing);
        }

        let new_assignment = NewTicketAssignment {
            id: format!("asgn-{}", Uuid::new_v4()),
            ticket_id: ticket_id.to_string(),
            porter_id: porter_id.to_string(),
            role: AssignmentRole::Supervisor.to_string(),
            is_active: true,
        };

        TicketAssignmentRepository::insert_assignment(conn, &new_assignment).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to create assignment: {}", e))
        })
    }

    pub fn find_active_co_partners(
        conn: &mut PgConnection,
        ticket_id: &str,
    ) -> ApiResult<Vec<TicketAssignment>> {
        TicketAssignmentRepository::find_active_co_partners(conn, ticket_id).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to load co-partners: {}", e))
        })
    }

    pub fn add_co_partner_assignment(
        conn: &mut PgConnection,
        ticket_id: &str,
        porter_id: &str,
    ) -> ApiResult<TicketAssignment> {
        let existing = TicketAssignmentRepository::find_active_assignment_for_porter(
            conn, ticket_id, porter_id,
        )
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to check assignments: {}", e))
        })?;

        if existing.is_some() {
            return Err(ApiError::BadRequest(
                "Porter already assigned to this ticket".to_string(),
            ));
        }

        let new_assignment = NewTicketAssignment {
            id: format!("asgn-{}", Uuid::new_v4()),
            ticket_id: ticket_id.to_string(),
            porter_id: porter_id.to_string(),
            role: AssignmentRole::CoPartner.to_string(),
            is_active: true,
        };

        TicketAssignmentRepository::insert_assignment(conn, &new_assignment)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to add co-partner: {}", e)))
    }

    pub fn deactivate_assignment_by_id(
        conn: &mut PgConnection,
        assignment_id: &str,
    ) -> ApiResult<TicketAssignment> {
        TicketAssignmentRepository::deactivate_assignment(
            conn,
            assignment_id,
            Utc::now().naive_utc(),
        )
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to deactivate assignment: {}", e))
        })
    }

    pub fn deactivate_active_co_partner(
        conn: &mut PgConnection,
        ticket_id: &str,
        porter_id: &str,
    ) -> ApiResult<TicketAssignment> {
        TicketAssignmentRepository::deactivate_active_co_partner(
            conn,
            ticket_id,
            porter_id,
            Utc::now().naive_utc(),
        )
        .map_err(|_| ApiError::NotFound("Active co-partner assignment not found".to_string()))
    }

    pub fn deactivate_all_active_for_ticket(
        conn: &mut PgConnection,
        ticket_id: &str,
    ) -> ApiResult<usize> {
        TicketAssignmentRepository::deactivate_all_active_for_ticket(
            conn,
            ticket_id,
            Utc::now().naive_utc(),
        )
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to deactivate ticket assignments: {}", e))
        })
    }

    pub fn porter_has_active_supervisor(
        conn: &mut PgConnection,
        porter_id: &str,
        exclude_ticket_id: &str,
    ) -> ApiResult<bool> {
        TicketAssignmentRepository::has_active_supervisor_elsewhere(
            conn,
            porter_id,
            exclude_ticket_id,
        )
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to check supervisor assignment: {}", e))
        })
    }

    pub fn resolve_supervisor_id(
        conn: &mut PgConnection,
        ticket: &Ticket,
        ticket_id: &str,
    ) -> ApiResult<Option<String>> {
        Ok(Self::find_active_supervisor_assignment(conn, ticket_id)?
            .map(|a| a.porter_id)
            .or(ticket.porter_id.clone()))
    }
}
