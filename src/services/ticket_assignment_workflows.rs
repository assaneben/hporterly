use chrono::{Duration, Utc};
use diesel::PgConnection;

use crate::models::{
    AssignTicketRequest, DesignateSuccessorRequest, Ticket, TicketAssignment, User,
};
use crate::repositories::{PorterRepository, TicketRepository};
use crate::services::{
    can_self_assign_by_priority, is_regulation_role, normalize_status, TicketAssignmentService,
};
use crate::utils::{require_admin, ApiError, ApiResult};

pub struct TicketAssignmentWorkflowService;

pub struct TicketAssignResult {
    pub ticket_before: Ticket,
    pub ticket_after: Ticket,
    pub porter_id: String,
}

pub struct TicketReassignResult {
    pub ticket_before: Ticket,
    pub ticket_after: Ticket,
    pub old_supervisor_id: Option<String>,
    pub new_porter_id: String,
}

pub struct TicketUnassignResult {
    pub ticket_before: Ticket,
    pub ticket_after: Ticket,
    pub old_supervisor_id: Option<String>,
    pub successor_id: Option<String>,
}

pub struct TicketAddCoPartnerResult {
    pub assignment: TicketAssignment,
    pub target_porter_id: String,
    pub target_user_id: String,
}

pub struct TicketRemoveCoPartnerResult {
    pub assignment: TicketAssignment,
    pub target_porter_id: String,
    pub target_user_id: String,
}

impl TicketAssignmentWorkflowService {
    pub fn add_co_partner(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        target_porter_id: &str,
    ) -> ApiResult<TicketAddCoPartnerResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        if user.role == "brancardier" {
            let porter_id = TicketAssignmentService::get_porter_id_for_user(conn, user)?;
            let supervisor_id =
                TicketAssignmentService::resolve_supervisor_id(conn, &ticket, ticket_id)?;

            if supervisor_id.as_deref() != Some(&porter_id) {
                return Err(ApiError::Forbidden(
                    "Only supervisor can add co-partners".to_string(),
                ));
            }
        } else {
            require_admin(user)?;
        }

        let target_porter = PorterRepository::find_by_id(conn, target_porter_id)
            .map_err(|_| ApiError::NotFound(format!("Porter {} not found", target_porter_id)))?;

        let assignment =
            TicketAssignmentService::add_co_partner_assignment(conn, ticket_id, &target_porter.id)?;

        Ok(TicketAddCoPartnerResult {
            assignment,
            target_porter_id: target_porter.id,
            target_user_id: target_porter.user_id,
        })
    }

    pub fn remove_co_partner(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        target_porter_id: &str,
    ) -> ApiResult<TicketRemoveCoPartnerResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        if user.role == "brancardier" {
            let porter_id = TicketAssignmentService::get_porter_id_for_user(conn, user)?;
            let supervisor_id =
                TicketAssignmentService::resolve_supervisor_id(conn, &ticket, ticket_id)?;

            if supervisor_id.as_deref() != Some(&porter_id) {
                return Err(ApiError::Forbidden(
                    "Only supervisor can remove co-partners".to_string(),
                ));
            }
        } else {
            require_admin(user)?;
        }

        let target_porter = PorterRepository::find_by_id(conn, target_porter_id)
            .map_err(|_| ApiError::NotFound(format!("Porter {} not found", target_porter_id)))?;

        let assignment = TicketAssignmentService::deactivate_active_co_partner(
            conn,
            ticket_id,
            &target_porter.id,
        )?;

        Ok(TicketRemoveCoPartnerResult {
            assignment,
            target_porter_id: target_porter.id,
            target_user_id: target_porter.user_id,
        })
    }

    pub fn assign(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        request: &AssignTicketRequest,
    ) -> ApiResult<TicketAssignResult> {
        let actor_is_porter = user.role == "brancardier";
        let actor_is_regulation = is_regulation_role(user.role.as_str());
        if !actor_is_porter && !actor_is_regulation {
            require_admin(user)?;
        }

        let porter_id = if let Some(pid) = &request.porter_id {
            let porter = PorterRepository::find_by_id(conn, pid)
                .map_err(|_| ApiError::NotFound(format!("Porter {} not found", pid)))?;

            if actor_is_porter && porter.user_id != user.id {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez vous assigner que vous-meme a une mission".to_string(),
                ));
            }
            pid.clone()
        } else {
            if !actor_is_porter {
                return Err(ApiError::BadRequest(
                    "porter_id est obligatoire pour une assignation manuelle".to_string(),
                ));
            }
            let porter = PorterRepository::find_by_user_id(conn, &user.id).map_err(|_| {
                ApiError::NotFound("Aucun profil brancardier associe a cet utilisateur".to_string())
            })?;
            porter.id
        };

        if TicketAssignmentService::porter_has_active_supervisor(conn, &porter_id, ticket_id)? {
            return Err(ApiError::BadRequest(
                "Porter already supervisor on another ticket".to_string(),
            ));
        }

        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        if actor_is_porter {
            if !can_self_assign_by_priority(ticket.priority) {
                return Err(ApiError::Forbidden(
                    "Auto-affectation autorisee uniquement pour P1, P3 et P4".to_string(),
                ));
            }
            if ticket.priority == 1 && ticket.transport_type.eq_ignore_ascii_case("EQUIPMENT") {
                return Err(ApiError::BadRequest(
                    "P1 interdit pour une mission materiel seule".to_string(),
                ));
            }
        }

        if !ticket.can_transition_to("assigned") {
            return Err(ApiError::BadRequest(format!(
                "Cannot assign ticket with status {}",
                ticket.status
            )));
        }

        if let Some(locked_by) = &ticket.reservation_locked_by {
            if locked_by != &porter_id {
                if let Some(locked_at) = ticket.reservation_locked_at {
                    if Utc::now().naive_utc() - locked_at < Duration::seconds(30) {
                        return Err(ApiError::BadRequest("Ticket already reserved".to_string()));
                    }
                }
            }
        }

        TicketAssignmentService::ensure_supervisor_assignment(conn, ticket_id, &porter_id)?;

        let updated_ticket =
            TicketRepository::assign_to_porter_and_clear_reservation(conn, ticket_id, &porter_id)
                .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to assign ticket: {}", e))
            })?;

        if let Err(err) = PorterRepository::set_status(conn, &porter_id, "busy") {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(TicketAssignResult {
            ticket_before: ticket,
            ticket_after: updated_ticket,
            porter_id,
        })
    }

    pub fn reassign(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        new_porter_id: &str,
    ) -> ApiResult<TicketReassignResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        if user.role == "brancardier" {
            let porter_id = TicketAssignmentService::get_porter_id_for_user(conn, user)?;
            let supervisor_id =
                TicketAssignmentService::resolve_supervisor_id(conn, &ticket, ticket_id)?;

            if supervisor_id.as_deref() != Some(&porter_id) {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez reassigner que vos propres missions".to_string(),
                ));
            }
        } else {
            require_admin(user)?;
        }

        let can_reassign = matches!(
            normalize_status(ticket.status.as_str()),
            Some("assigned" | "in_progress" | "arrived" | "suspended")
        );
        if !can_reassign {
            return Err(ApiError::BadRequest(format!(
                "Cannot reassign ticket with status {}",
                ticket.status
            )));
        }

        if TicketAssignmentService::porter_has_active_supervisor(conn, new_porter_id, ticket_id)? {
            return Err(ApiError::BadRequest(
                "Porter already supervisor on another ticket".to_string(),
            ));
        }

        PorterRepository::find_by_id(conn, new_porter_id)
            .map_err(|_| ApiError::NotFound(format!("Porter {} not found", new_porter_id)))?;

        let old_supervisor_id =
            TicketAssignmentService::resolve_supervisor_id(conn, &ticket, ticket_id)?;

        if let Some(old_assignment) =
            TicketAssignmentService::find_active_supervisor_assignment(conn, ticket_id)?
        {
            if let Err(err) =
                TicketAssignmentService::deactivate_assignment_by_id(conn, &old_assignment.id)
            {
                log::warn!("Non-blocking operation failed: {}", err);
            }
        }

        TicketAssignmentService::ensure_supervisor_assignment(conn, ticket_id, new_porter_id)?;

        let updated_ticket =
            TicketRepository::set_porter_and_status(conn, ticket_id, new_porter_id, "assigned")
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Failed to reassign ticket: {}", e))
                })?;

        if let Some(old_pid) = &old_supervisor_id {
            if old_pid != new_porter_id {
                if let Err(err) = PorterRepository::set_status(conn, old_pid, "available") {
                    log::warn!("Non-blocking operation failed: {}", err);
                }
            }
        }

        if let Err(err) = PorterRepository::set_status(conn, new_porter_id, "busy") {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(TicketReassignResult {
            ticket_before: ticket,
            ticket_after: updated_ticket,
            old_supervisor_id,
            new_porter_id: new_porter_id.to_string(),
        })
    }

    pub fn unassign(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        request: Option<&DesignateSuccessorRequest>,
    ) -> ApiResult<TicketUnassignResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        if user.role == "brancardier" {
            let porter_id = TicketAssignmentService::get_porter_id_for_user(conn, user)?;
            let is_ticket_owner = ticket.porter_id.as_deref() == Some(&porter_id);
            let is_active_supervisor =
                TicketAssignmentService::find_active_supervisor_assignment(conn, ticket_id)?
                    .map(|a| a.porter_id == porter_id)
                    .unwrap_or(false);

            if !is_ticket_owner && !is_active_supervisor {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez renvoyer que vos propres missions".to_string(),
                ));
            }
        } else {
            require_admin(user)?;
        }

        let can_unassign = matches!(
            normalize_status(ticket.status.as_str()),
            Some("assigned" | "in_progress" | "arrived" | "suspended")
        );
        if !can_unassign {
            return Err(ApiError::BadRequest(format!(
                "Cannot unassign ticket with status {}",
                ticket.status
            )));
        }

        let mut supervisor_assignment =
            TicketAssignmentService::find_active_supervisor_assignment(conn, ticket_id)?;
        if supervisor_assignment.is_none() {
            if let Some(ref pid) = ticket.porter_id {
                supervisor_assignment = Some(
                    TicketAssignmentService::ensure_supervisor_assignment(conn, ticket_id, pid)?,
                );
            }
        }

        let old_supervisor_id = supervisor_assignment
            .as_ref()
            .map(|a| a.porter_id.clone())
            .or(ticket.porter_id.clone());

        let co_partners = TicketAssignmentService::find_active_co_partners(conn, ticket_id)?;

        let mut successor_id: Option<String> = None;
        if let Some(req) = request {
            if let Some(ref requested) = req.next_supervisor_porter_id {
                let valid = co_partners.iter().any(|a| a.porter_id == *requested);
                if !valid {
                    return Err(ApiError::BadRequest(
                        "Requested successor must be an active co-partner".to_string(),
                    ));
                }
                successor_id = Some(requested.clone());
            }
        }
        if successor_id.is_none() {
            successor_id = co_partners.first().map(|a| a.porter_id.clone());
        }

        if let Some(assign) = supervisor_assignment {
            if let Err(err) = TicketAssignmentService::deactivate_assignment_by_id(conn, &assign.id)
            {
                log::warn!("Non-blocking operation failed: {}", err);
            }
        }

        let updated_ticket = if let Some(next_id) = successor_id.clone() {
            TicketAssignmentService::ensure_supervisor_assignment(conn, ticket_id, &next_id)?;
            TicketRepository::set_porter_and_status(conn, ticket_id, &next_id, "assigned").map_err(
                |e| ApiError::InternalServerError(format!("Failed to unassign ticket: {}", e)),
            )?
        } else {
            TicketRepository::clear_porter_and_set_status(conn, ticket_id, "pending").map_err(
                |e| ApiError::InternalServerError(format!("Failed to unassign ticket: {}", e)),
            )?
        };

        if let Some(old_pid) = &old_supervisor_id {
            if let Err(err) = PorterRepository::set_status(conn, old_pid, "available") {
                log::warn!("Non-blocking operation failed: {}", err);
            }
        }

        if let Some(next_id) = successor_id.as_deref() {
            if let Err(err) = PorterRepository::set_status(conn, next_id, "busy") {
                log::warn!("Non-blocking operation failed: {}", err);
            }
        }

        Ok(TicketUnassignResult {
            ticket_before: ticket,
            ticket_after: updated_ticket,
            old_supervisor_id,
            successor_id,
        })
    }
}
