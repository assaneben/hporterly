use chrono::Utc;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models::{
    CreateHelpRequest, HelpRequest, NewHelpRequest, Porter, RespondToHelpRequest, User,
};
use crate::repositories::{HelpRequestRepository, PorterRepository, TicketRepository};
use crate::services::TicketAssignmentService;
use crate::utils::{ApiError, ApiResult};

pub struct TicketHelpService;

pub struct HelpRequestCreationResult {
    pub help_request_id: String,
    pub requested_porter_user_id: String,
}

pub struct HelpRequestResponseResult {
    pub accepted: bool,
    pub status: String,
}

impl TicketHelpService {
    pub fn request_help(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        request: &CreateHelpRequest,
    ) -> ApiResult<HelpRequestCreationResult> {
        if user.role != "brancardier" {
            return Err(ApiError::Forbidden("Acces reserve aux brancardiers".to_string()));
        }

        let requester_porter_id = TicketAssignmentService::get_porter_id_for_user(conn, user)?;
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.porter_id.as_ref() != Some(&requester_porter_id) {
            return Err(ApiError::Forbidden("Vous n'etes pas assigne a cette mission".to_string()));
        }

        let requested_porter = PorterRepository::find_by_id(conn, &request.requested_porter_id)
            .map_err(|_| {
                ApiError::NotFound(format!("Porter {} not found", request.requested_porter_id))
            })?;

        if requested_porter.status != "available" {
            return Err(ApiError::BadRequest(
                "Le brancardier demande n'est pas disponible".to_string(),
            ));
        }

        let help_request_id = format!("HELP-{}", &Uuid::new_v4().to_string()[..8]);
        let payload = NewHelpRequest {
            id: help_request_id.clone(),
            ticket_id: ticket_id.to_string(),
            requesting_porter_id: requester_porter_id,
            requested_porter_id: request.requested_porter_id.clone(),
            status: "pending".to_string(),
        };

        HelpRequestRepository::insert(conn, &payload).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to create help request: {}", e))
        })?;

        TicketRepository::set_help_request_pending(
            conn,
            ticket_id,
            &request.requested_porter_id,
            Utc::now().naive_utc(),
        )
        .map_err(|e| ApiError::InternalServerError(format!("Failed to update ticket: {}", e)))?;

        Ok(HelpRequestCreationResult {
            help_request_id,
            requested_porter_user_id: requested_porter.user_id,
        })
    }

    pub fn respond_to_help(
        conn: &mut PgConnection,
        user: &User,
        help_request_id: &str,
        response: &RespondToHelpRequest,
    ) -> ApiResult<HelpRequestResponseResult> {
        if user.role != "brancardier" {
            return Err(ApiError::Forbidden("Acces reserve aux brancardiers".to_string()));
        }

        let responder_porter_id = TicketAssignmentService::get_porter_id_for_user(conn, user)?;
        let help_request =
            HelpRequestRepository::find_by_id(conn, help_request_id).map_err(|_| {
                ApiError::NotFound(format!("Help request {} not found", help_request_id))
            })?;

        if help_request.requested_porter_id != responder_porter_id {
            return Err(ApiError::Forbidden("Cette demande ne vous concerne pas".to_string()));
        }

        let status = if response.accepted { "accepted" } else { "declined" };

        HelpRequestRepository::set_status_with_responded_at(
            conn,
            help_request_id,
            status,
            Utc::now().naive_utc(),
        )
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to update help request: {}", e))
        })?;

        TicketRepository::set_help_status(conn, &help_request.ticket_id, status).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to update ticket: {}", e))
        })?;

        Ok(HelpRequestResponseResult { accepted: response.accepted, status: status.to_string() })
    }

    pub fn get_my_pending_help_requests(
        conn: &mut PgConnection,
        user: &User,
    ) -> ApiResult<Vec<HelpRequest>> {
        if user.role != "brancardier" {
            return Err(ApiError::Forbidden("Acces reserve aux brancardiers".to_string()));
        }

        let porter_id = TicketAssignmentService::get_porter_id_for_user(conn, user)?;
        HelpRequestRepository::list_pending_for_requested_porter(conn, &porter_id).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to load help requests: {}", e))
        })
    }

    pub fn get_available_porters_for_help(
        conn: &mut PgConnection,
        ticket_id: &str,
    ) -> ApiResult<Vec<Porter>> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        PorterRepository::list_available_excluding(conn, ticket.porter_id.as_deref())
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load porters: {}", e)))
    }
}
