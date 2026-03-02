use chrono::Utc;
use diesel::PgConnection;

use crate::models::{Ticket, User};
use crate::repositories::{PorterRepository, TicketRepository};
use crate::services::{is_regulation_role, TicketAssignmentService};
use crate::utils::{require_admin, ApiError, ApiResult};

pub struct TicketUpdateService;

pub struct EquipmentStatusResult {
    pub delivered: bool,
    pub label_returned: bool,
    pub completed: bool,
}

pub struct TicketNotesUpdateResult {
    pub ticket_before: Ticket,
    pub ticket_after: Ticket,
    pub requested_notes: Option<String>,
}

pub struct TicketPriorityOverrideResult {
    pub ticket_before: Ticket,
    pub ticket_after: Ticket,
    pub reason: String,
}

impl TicketUpdateService {
    pub fn update_equipment_status(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        delivered: Option<bool>,
        label_returned: Option<bool>,
    ) -> ApiResult<EquipmentStatusResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        if user.role == "brancardier" {
            let is_assigned = if let Some(ref porter_id) = ticket.porter_id {
                PorterRepository::exists_for_user_and_porter_id(conn, porter_id, &user.id).map_err(
                    |e| {
                        ApiError::InternalServerError(format!("Failed to verify assignment: {}", e))
                    },
                )?
            } else {
                false
            };

            if !is_assigned {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez modifier que vos propres missions".to_string(),
                ));
            }
        } else if user.role == "demandeur" {
            return Err(ApiError::Forbidden("Acces non autorise".to_string()));
        }

        let delivered = delivered.unwrap_or(ticket.equipment_delivered.unwrap_or(false));
        let label_returned =
            label_returned.unwrap_or(ticket.equipment_label_returned.unwrap_or(false));

        TicketRepository::update_equipment_status(conn, ticket_id, delivered, label_returned)
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to update equipment status: {}", e))
            })?;

        let mut completed = false;
        if delivered && label_returned {
            let now = Utc::now().naive_utc();
            TicketRepository::update_status_completed_and_archive(
                conn,
                ticket_id,
                "completed",
                now,
            )
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to complete ticket: {}", e))
            })?;

            if let Err(err) =
                TicketAssignmentService::deactivate_all_active_for_ticket(conn, ticket_id)
            {
                log::warn!("Non-blocking operation failed: {}", err);
            }

            if let Some(porter_id) = &ticket.porter_id {
                if let Err(err) =
                    PorterRepository::increment_completed_stats_and_set_available(conn, porter_id)
                {
                    log::warn!("Non-blocking operation failed: {}", err);
                }
            }

            completed = true;
        }

        Ok(EquipmentStatusResult {
            delivered,
            label_returned,
            completed,
        })
    }

    pub fn update_notes(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        notes: Option<String>,
    ) -> ApiResult<TicketNotesUpdateResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        if user.role == "brancardier" {
            let is_assigned = if let Some(ref porter_id) = ticket.porter_id {
                PorterRepository::exists_for_user_and_porter_id(conn, porter_id, &user.id).map_err(
                    |e| {
                        ApiError::InternalServerError(format!("Failed to verify assignment: {}", e))
                    },
                )?
            } else {
                false
            };

            if !is_assigned {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez modifier que les notes de vos propres missions".to_string(),
                ));
            }
        } else {
            require_admin(user)?;
        }

        let updated_ticket = TicketRepository::update_notes(conn, ticket_id, notes.clone())
            .map_err(|e| ApiError::InternalServerError(format!("Failed to update notes: {}", e)))?;

        Ok(TicketNotesUpdateResult {
            ticket_before: ticket,
            ticket_after: updated_ticket,
            requested_notes: notes,
        })
    }

    pub fn override_priority(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        priority: i32,
        reason: &str,
    ) -> ApiResult<TicketPriorityOverrideResult> {
        if !(1..=4).contains(&priority) {
            return Err(ApiError::BadRequest(
                "Priorite invalide (attendue entre N1 et N4)".to_string(),
            ));
        }

        let reason = reason.trim();
        if reason.is_empty() {
            return Err(ApiError::BadRequest(
                "Le motif de surclassement est obligatoire".to_string(),
            ));
        }

        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        let requester_match = ticket.requester_id == user.id
            || ticket
                .requester_id
                .eq_ignore_ascii_case(user.username.as_str());
        let actor_is_regulation = is_regulation_role(user.role.as_str());

        if user.role == "brancardier" {
            return Err(ApiError::Forbidden(
                "Un brancardier ne peut pas modifier la priorite d'une mission".to_string(),
            ));
        }

        if user.role == "demandeur" {
            if !requester_match {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez modifier la priorite que de vos propres demandes".to_string(),
                ));
            }

            if priority > ticket.priority {
                return Err(ApiError::Forbidden(
                    "Un demandeur ne peut pas retrograder la priorite".to_string(),
                ));
            }
        } else if !actor_is_regulation {
            require_admin(user)?;
        }

        if ticket.is_archived || ["completed", "canceled"].contains(&ticket.status.as_str()) {
            return Err(ApiError::BadRequest(
                "Impossible de surclasser une demande terminee/annulee/archivee".to_string(),
            ));
        }

        if ticket.priority == priority {
            return Err(ApiError::BadRequest(
                "Cette demande est deja sur ce niveau de priorite".to_string(),
            ));
        }

        if priority == 1 && ticket.transport_type.eq_ignore_ascii_case("EQUIPMENT") {
            return Err(ApiError::BadRequest(
                "Le niveau P1 est interdit pour une mission materiel seule".to_string(),
            ));
        }

        let actor_label = format!("{} {}", user.first_name, user.last_name)
            .trim()
            .to_string();
        let actor = if actor_label.is_empty() {
            user.username.clone()
        } else {
            actor_label
        };

        let now = Utc::now().naive_utc();
        let workflow_line = format!(
            "[PRIORITY_OVERRIDE] {} | {} | N{} -> N{} | {}",
            now, actor, ticket.priority, priority, reason
        );
        let merged_notes = match ticket
            .notes
            .as_ref()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
        {
            Some(existing) => format!("{}\n{}", existing, workflow_line),
            None => workflow_line,
        };

        let updated_ticket = TicketRepository::override_priority_with_notes(
            conn,
            ticket_id,
            priority,
            merged_notes,
            now,
        )
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to override priority: {}", e))
        })?;

        Ok(TicketPriorityOverrideResult {
            ticket_before: ticket,
            ticket_after: updated_ticket,
            reason: reason.to_string(),
        })
    }
}
