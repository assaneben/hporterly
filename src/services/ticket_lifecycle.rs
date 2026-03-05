use chrono::Utc;
use diesel::{Connection, PgConnection};

use crate::models::{Ticket, UpdateTicketStatus, User};
use crate::repositories::{PorterRepository, TicketRepository};
use crate::services::{
    is_regulation_role, validate_cancellation_reason, validate_suspension_reason,
    TicketAssignmentService, STATUS_CANCELED, STATUS_COMPLETED, STATUS_SUSPENDED,
};
use crate::utils::{require_admin, ApiError, ApiResult};

pub struct TicketLifecycleService;

pub struct TicketStatusTransitionResult {
    pub ticket_before: Ticket,
    pub ticket_after: Ticket,
}

pub struct TicketHardDeleteResult {
    pub ticket_before: Ticket,
    pub deleted_ticket_id: String,
    pub reason: Option<String>,
}

impl TicketLifecycleService {
    pub fn update_status(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        request: &UpdateTicketStatus,
    ) -> ApiResult<TicketStatusTransitionResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        let normalized_status = Ticket::normalize_status(request.status.as_str())
            .ok_or_else(|| ApiError::BadRequest("Statut de mission invalide".to_string()))?;
        let normalized_current =
            Ticket::normalize_status(ticket.status.as_str()).ok_or_else(|| {
                ApiError::BadRequest(format!("Statut actuel invalide: {}", ticket.status))
            })?;

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

            if normalized_status == STATUS_CANCELED {
                return Err(ApiError::Forbidden(
                    "Annulation reservee a la regulation (SUPER_REGUL/ADMIN_SYS)".to_string(),
                ));
            }

            if normalized_status == STATUS_SUSPENDED && ticket.priority == 1 {
                return Err(ApiError::Forbidden(
                    "Une mission P1 ne peut pas etre suspendue par un brancardier seul".to_string(),
                ));
            }
        } else if user.role == "demandeur" {
            let requester_match = ticket.requester_id == user.id
                || ticket.requester_id.eq_ignore_ascii_case(&user.username);
            if !requester_match {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez modifier que vos propres demandes".to_string(),
                ));
            }

            let is_pending_cancel =
                normalized_current == "pending" && normalized_status == STATUS_CANCELED;
            if !is_pending_cancel {
                return Err(ApiError::Forbidden(
                    "Un demandeur ne peut annuler que ses demandes en attente".to_string(),
                ));
            }
        } else if !is_regulation_role(user.role.as_str()) {
            require_admin(user)?;
        }

        if !ticket.can_transition_to(normalized_status) {
            return Err(ApiError::BadRequest(format!(
                "Cannot transition from {} to {}",
                ticket.status, normalized_status
            )));
        }

        if normalized_status == STATUS_COMPLETED
            && Self::is_donjoy_ticket(&ticket)
            && !ticket.equipment_label_returned.unwrap_or(false)
        {
            return Err(ApiError::BadRequest(
                "Etiquette patient non retournee - cloture DonJoy bloquee".to_string(),
            ));
        }

        if normalized_status == STATUS_CANCELED {
            let reason_code = validate_cancellation_reason(
                request.reason_code.as_deref(),
                request.comment.as_deref(),
            )?;
            Self::append_workflow_note(
                conn,
                &ticket,
                format!(
                    "[CANCELED] actor={} reason={} comment={}",
                    user.username,
                    reason_code,
                    request.comment.as_deref().unwrap_or("").trim()
                ),
            )?;
        } else if normalized_status == STATUS_SUSPENDED {
            let reason_code = validate_suspension_reason(
                request.reason_code.as_deref(),
                request.comment.as_deref(),
            )?;
            Self::append_workflow_note(
                conn,
                &ticket,
                format!(
                    "[SUSPENDED] actor={} reason={} comment={}",
                    user.username,
                    reason_code,
                    request.comment.as_deref().unwrap_or("").trim()
                ),
            )?;
        } else if let Some(comment) = request
            .comment
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            Self::append_workflow_note(
                conn,
                &ticket,
                format!(
                    "[STATUS_NOTE] actor={} status={} comment={}",
                    user.username, normalized_status, comment
                ),
            )?;
        }

        let updated_ticket = if normalized_status == STATUS_COMPLETED {
            let now = Utc::now().naive_utc();
            let t = TicketRepository::update_status_completed_and_archive(
                conn,
                ticket_id,
                STATUS_COMPLETED,
                now,
            )
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to update ticket: {}", e))
            })?;

            if let Err(err) =
                TicketAssignmentService::deactivate_all_active_for_ticket(conn, ticket_id)
            {
                log::warn!("Non-blocking operation failed: {}", err);
            }

            if let Some(pid) = &t.porter_id {
                PorterRepository::increment_completed_stats_and_set_available(conn, pid).map_err(
                    |e| {
                        ApiError::InternalServerError(format!(
                            "Failed to update porter stats: {}",
                            e
                        ))
                    },
                )?;
            }

            t
        } else if normalized_status == STATUS_CANCELED {
            let now = Utc::now().naive_utc();
            let t = TicketRepository::update_status_canceled_and_archive(
                conn,
                ticket_id,
                STATUS_CANCELED,
                now,
            )
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to update ticket: {}", e))
            })?;

            if let Err(err) =
                TicketAssignmentService::deactivate_all_active_for_ticket(conn, ticket_id)
            {
                log::warn!("Non-blocking operation failed: {}", err);
            }

            t
        } else {
            TicketRepository::update_status(conn, ticket_id, normalized_status).map_err(|e| {
                ApiError::InternalServerError(format!("Failed to update ticket: {}", e))
            })?
        };

        if normalized_status == STATUS_CANCELED {
            if let Some(pid) = &updated_ticket.porter_id {
                PorterRepository::set_status(conn, pid, "available").map_err(|e| {
                    ApiError::InternalServerError(format!("Failed to release porter: {}", e))
                })?;
            }
        }

        Ok(TicketStatusTransitionResult {
            ticket_before: ticket,
            ticket_after: updated_ticket,
        })
    }

    pub fn pause(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
    ) -> ApiResult<TicketStatusTransitionResult> {
        require_admin(user)?;

        let request = UpdateTicketStatus {
            status: STATUS_SUSPENDED.to_string(),
            comment: Some("Suspension administrative".to_string()),
            reason_code: Some("urgent_interruption".to_string()),
        };

        Self::update_status(conn, user, ticket_id, &request)
    }

    pub fn cancel(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        reason_code: Option<String>,
        comment: Option<String>,
    ) -> ApiResult<TicketStatusTransitionResult> {
        require_admin(user)?;

        let reason = reason_code
            .as_deref()
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "other".to_string());
        let normalized_comment = comment
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());
        let final_comment = if reason == "other" {
            Some(normalized_comment.unwrap_or_else(|| "Annulation administrative".to_string()))
        } else {
            normalized_comment
        };

        let request = UpdateTicketStatus {
            status: STATUS_CANCELED.to_string(),
            comment: final_comment,
            reason_code: Some(reason),
        };

        Self::update_status(conn, user, ticket_id, &request)
    }

    pub fn hard_delete(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
        reason: Option<String>,
    ) -> ApiResult<TicketHardDeleteResult> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if user.role == "demandeur" {
            let requester_match = ticket.requester_id == user.id
                || ticket.requester_id.eq_ignore_ascii_case(&user.username);

            if !requester_match {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez supprimer definitivement que vos propres demandes".to_string(),
                ));
            }

            if ticket.status == "completed" {
                return Err(ApiError::BadRequest(
                    "Impossible de supprimer definitivement une demande terminee".to_string(),
                ));
            }
        } else {
            require_admin(user)?;
        }

        let cleaned_reason = reason
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());

        let release_supervisor_id = match ticket.porter_id.as_deref() {
            Some(porter_id) => {
                if TicketAssignmentService::porter_has_active_supervisor(
                    conn, porter_id, ticket_id,
                )? {
                    None
                } else {
                    Some(porter_id.to_string())
                }
            }
            None => None,
        };

        conn.transaction::<(), diesel::result::Error, _>(|tx| {
            if let Some(ref porter_id) = release_supervisor_id {
                PorterRepository::set_status(tx, porter_id, "available")?;
            }

            TicketRepository::delete_by_id(tx, ticket_id)?;
            Ok(())
        })
        .map_err(|e| {
            ApiError::InternalServerError(format!(
                "Failed to permanently delete ticket {}: {}",
                ticket_id, e
            ))
        })?;

        Ok(TicketHardDeleteResult {
            ticket_before: ticket,
            deleted_ticket_id: ticket_id.to_string(),
            reason: cleaned_reason,
        })
    }

    fn is_donjoy_ticket(ticket: &Ticket) -> bool {
        if !ticket.transport_type.eq_ignore_ascii_case("EQUIPMENT") {
            return false;
        }

        let subtype = ticket.transport_subtype.to_ascii_uppercase();
        if subtype.contains("DONJOY") || subtype.contains("ABDO") {
            return true;
        }

        ticket
            .notes
            .as_deref()
            .map(|value| value.to_ascii_uppercase().contains("DONJOY"))
            .unwrap_or(false)
    }

    fn append_workflow_note(
        conn: &mut PgConnection,
        ticket: &Ticket,
        line: String,
    ) -> ApiResult<()> {
        let cleaned_line = line.trim();
        if cleaned_line.is_empty() {
            return Ok(());
        }

        let merged = match ticket
            .notes
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            Some(existing) => format!("{}\n{}", existing, cleaned_line),
            None => cleaned_line.to_string(),
        };

        TicketRepository::update_notes(conn, ticket.id.as_str(), Some(merged))
            .map(|_| ())
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to store workflow note: {}", e))
            })
    }
}
