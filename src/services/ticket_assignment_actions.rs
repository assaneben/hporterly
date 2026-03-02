use serde_json::{json, Value};

use crate::models::{
    AssignTicketRequest, DesignateSuccessorRequest, Ticket, TicketAssignment, UpdateTicketStatus,
    User,
};
use crate::repositories::{PorterRepository, TicketRepository};
use crate::services::{
    normalize_status, AuditService, DispatchService, NotificationService,
    TicketAssignmentWorkflowService, TicketLifecycleService,
};
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

pub struct TicketAssignmentActionService;

impl TicketAssignmentActionService {
    pub fn assign_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        request: AssignTicketRequest,
    ) -> ApiResult<Ticket> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let assignment =
            TicketAssignmentWorkflowService::assign(&mut conn, user, ticket_id, &request)?;
        let ticket_before = assignment.ticket_before;
        let ticket_after = assignment.ticket_after;
        let porter_id = assignment.porter_id;

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "ASSIGN",
            "ticket",
            ticket_id,
            Some(json!(ticket_before)),
            Some(json!(&ticket_after)),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        log::info!("Ticket {} assigned to porter {} by user {}", ticket_id, porter_id, user.id);

        let porter_name = format!("{} {}", user.first_name, user.last_name);
        let notif = NotificationService::ticket_assigned(ticket_id, &porter_name);
        if let Ok(reg_admin_ids) = NotificationService::find_user_ids_by_roles(
            &mut conn,
            &["administrateur", "regulateur"],
        ) {
            for uid in &reg_admin_ids {
                if let Err(err) =
                    NotificationService::persist_if_enabled(&mut conn, uid, &notif, Some(ticket_id))
                {
                    log::warn!("Non-blocking operation failed: {}", err);
                }
            }
        }

        Ok(ticket_after)
    }

    pub fn add_co_partner_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        target_porter_id: &str,
    ) -> ApiResult<TicketAssignment> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let result = TicketAssignmentWorkflowService::add_co_partner(
            &mut conn,
            user,
            ticket_id,
            target_porter_id,
        )?;

        let notif = NotificationService::co_partner_added(ticket_id, &result.target_porter_id);
        if let Err(err) = NotificationService::persist_if_enabled(
            &mut conn,
            &result.target_user_id,
            &notif,
            Some(ticket_id),
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(result.assignment)
    }

    pub fn remove_co_partner_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        target_porter_id: &str,
    ) -> ApiResult<TicketAssignment> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let result = TicketAssignmentWorkflowService::remove_co_partner(
            &mut conn,
            user,
            ticket_id,
            target_porter_id,
        )?;

        let notif = NotificationService::co_partner_removed(ticket_id, &result.target_porter_id);
        if let Err(err) = NotificationService::persist_if_enabled(
            &mut conn,
            &result.target_user_id,
            &notif,
            Some(ticket_id),
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(result.assignment)
    }

    pub fn update_status_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        request: UpdateTicketStatus,
    ) -> ApiResult<Ticket> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let transition =
            TicketLifecycleService::update_status(&mut conn, user, ticket_id, &request)?;
        let ticket_before = transition.ticket_before;
        let ticket_after = transition.ticket_after;
        let normalized_status =
            normalize_status(request.status.as_str()).unwrap_or(request.status.as_str());

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "UPDATE_STATUS",
            "ticket",
            ticket_id,
            Some(json!({"status": ticket_before.status})),
            Some(json!({
                "status": normalized_status,
                "reason_code": request.reason_code,
                "comment": request.comment
            })),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        log::info!(
            "Ticket {} status updated to {} by user {}",
            ticket_id,
            normalized_status,
            user.id
        );

        let notif = if normalized_status == "completed" {
            NotificationService::ticket_completed(ticket_id)
        } else {
            NotificationService::ticket_updated(ticket_id, normalized_status)
        };

        if let Err(err) = NotificationService::persist_if_enabled(
            &mut conn,
            &ticket_before.requester_id,
            &notif,
            Some(ticket_id),
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        if let Ok(reg_admin_ids) = NotificationService::find_user_ids_by_roles(
            &mut conn,
            &["administrateur", "regulateur"],
        ) {
            for uid in reg_admin_ids.iter().filter(|uid| uid.as_str() != user.id.as_str()) {
                if let Err(err) =
                    NotificationService::persist_if_enabled(&mut conn, uid, &notif, Some(ticket_id))
                {
                    log::warn!("Non-blocking operation failed: {}", err);
                }
            }
        }

        Ok(ticket_after)
    }

    pub fn recommendations_from_pool(pool: &DbPool, ticket_id: &str) -> ApiResult<Vec<Value>> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let ticket = TicketRepository::find_by_id(&mut conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;
        if ticket.is_archived {
            return Err(ApiError::BadRequest("Ticket is archived".to_string()));
        }

        let available_porters = PorterRepository::list_available(&mut conn).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to fetch porters: {}", e))
        })?;
        let recommendations = DispatchService::get_recommendations(&ticket, available_porters);

        Ok(recommendations
            .into_iter()
            .map(|(porter, score)| {
                json!({
                    "porter": porter,
                    "score": score
                })
            })
            .collect())
    }

    pub fn pause_from_pool(pool: &DbPool, user: &User, ticket_id: &str) -> ApiResult<Ticket> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let transition = TicketLifecycleService::pause(&mut conn, user, ticket_id)?;
        let ticket_before = transition.ticket_before;
        let ticket_after = transition.ticket_after;

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "PAUSE",
            "ticket",
            ticket_id,
            Some(json!({"status": ticket_before.status})),
            Some(json!({"status": "suspended"})),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        log::info!("Ticket {} paused by admin {}", ticket_id, user.id);
        Ok(ticket_after)
    }

    pub fn reassign_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        request: AssignTicketRequest,
    ) -> ApiResult<Ticket> {
        let new_porter_id = request
            .porter_id
            .as_deref()
            .ok_or_else(|| {
                ApiError::BadRequest("porter_id is required for reassignment".to_string())
            })?
            .to_string();

        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let reassignment =
            TicketAssignmentWorkflowService::reassign(&mut conn, user, ticket_id, &new_porter_id)?;
        let ticket_before = reassignment.ticket_before;
        let ticket_after = reassignment.ticket_after;
        let old_supervisor_id = reassignment.old_supervisor_id;

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "REASSIGN",
            "ticket",
            ticket_id,
            Some(json!({
                "old_porter_id": old_supervisor_id,
                "status": ticket_before.status
            })),
            Some(json!({
                "new_porter_id": &new_porter_id,
                "status": "assigned"
            })),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(ticket_after)
    }

    pub fn unassign_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        request: Option<DesignateSuccessorRequest>,
    ) -> ApiResult<Ticket> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let unassignment = TicketAssignmentWorkflowService::unassign(
            &mut conn,
            user,
            ticket_id,
            request.as_ref(),
        )?;
        let ticket_before = unassignment.ticket_before;
        let ticket_after = unassignment.ticket_after;
        let old_supervisor_id = unassignment.old_supervisor_id;
        let successor_id = unassignment.successor_id;

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "UNASSIGN",
            "ticket",
            ticket_id,
            Some(json!({
                "porter_id": old_supervisor_id,
                "status": ticket_before.status
            })),
            Some(json!({
                "porter_id": successor_id,
                "status": ticket_after.status
            })),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(ticket_after)
    }

    pub fn cancel_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        reason_code: Option<String>,
        comment: Option<String>,
    ) -> ApiResult<Ticket> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let transition =
            TicketLifecycleService::cancel(&mut conn, user, ticket_id, reason_code, comment)?;
        let ticket_before = transition.ticket_before;
        let ticket_after = transition.ticket_after;

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "CANCEL",
            "ticket",
            ticket_id,
            Some(json!({"status": ticket_before.status})),
            Some(json!({"status": "canceled"})),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        log::info!("Ticket {} canceled by admin {}", ticket_id, user.id);

        let notif = NotificationService::ticket_canceled(ticket_id, "Annulee par l'administrateur");
        if let Err(err) = NotificationService::persist_if_enabled(
            &mut conn,
            &ticket_before.requester_id,
            &notif,
            Some(ticket_id),
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        if let Some(pid) = &ticket_before.porter_id {
            if let Ok(porter) = PorterRepository::find_by_id(&mut conn, pid) {
                if let Err(err) = NotificationService::persist_if_enabled(
                    &mut conn,
                    &porter.user_id,
                    &notif,
                    Some(ticket_id),
                ) {
                    log::warn!("Non-blocking operation failed: {}", err);
                }
            }
        }

        Ok(ticket_after)
    }

    pub fn hard_delete_from_pool(
        pool: &DbPool,
        user: &User,
        ticket_id: &str,
        reason: Option<String>,
    ) -> ApiResult<Value> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let deletion = TicketLifecycleService::hard_delete(&mut conn, user, ticket_id, reason)?;
        let ticket_before = deletion.ticket_before;
        let reason = deletion.reason;
        let deleted_ticket_id = deletion.deleted_ticket_id;

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "HARD_DELETE",
            "ticket",
            ticket_id,
            Some(json!(&ticket_before)),
            Some(json!({
                "deleted": true,
                "reason": reason,
            })),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        log::info!("Ticket {} permanently deleted by user {} ({})", ticket_id, user.id, user.role);

        Ok(json!({
            "id": deleted_ticket_id,
            "deleted": true
        }))
    }
}
