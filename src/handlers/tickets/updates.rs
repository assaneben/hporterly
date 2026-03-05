use super::*;

/// Struct pour mise a jour statut equipement
#[derive(Debug, Deserialize)]
pub struct UpdateEquipmentStatus {
    pub delivered: Option<bool>,
    pub label_returned: Option<bool>,
}

/// Mettre a jour le statut du workflow DonJoy Abdostrap
#[patch("/api/tickets/{id}/equipment-status")]
pub(super) async fn update_equipment_status(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<UpdateEquipmentStatus>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let result = TicketUpdateService::update_equipment_status(
        &mut conn,
        &user,
        ticket_id.as_str(),
        request.delivered,
        request.label_returned,
    )?;

    if result.completed {
        log::info!(
            "Ticket {} auto-completed (DonJoy Abdostrap workflow)",
            ticket_id
        );
        cda::trigger_transport_report_generation(
            pool.get_ref().clone(),
            config.get_ref().clone(),
            ticket_id.to_string(),
        );
    }

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "delivered": result.delivered,
        "label_returned": result.label_returned,
        "completed": result.completed
    })))
}

// ============ ENDPOINT NOTES ============

#[derive(Debug, Deserialize)]
struct UpdateNotesRequest {
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTicketPriorityRequest {
    priority: i32,
    reason: String,
}

/// Mettre a jour les notes d'un ticket (admin OU brancardier assigne)
#[patch("/api/tickets/{id}/notes")]
pub(super) async fn update_ticket_notes(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<UpdateNotesRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let result =
        TicketUpdateService::update_notes(&mut conn, &user, ticket_id.as_str(), request.notes)?;

    AuditService::log_action(
        &pool,
        &user,
        "UPDATE_NOTES",
        "ticket",
        ticket_id.as_str(),
        Some(json!({"notes": result.ticket_before.notes})),
        Some(json!({"notes": result.requested_notes})),
        None,
        None,
    )
    .log_if_err("Non-blocking operation failed");

    log::info!("Ticket {} notes updated by user {}", ticket_id, user.id);

    Ok(HttpResponse::Ok().json(result.ticket_after))
}

/// Surclassement manuel de priorite (administrateur / regulateur)
#[patch("/api/tickets/{id}/priority")]
pub(super) async fn update_ticket_priority(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<UpdateTicketPriorityRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let result = TicketUpdateService::override_priority(
        &mut conn,
        &user,
        ticket_id.as_str(),
        request.priority,
        request.reason.as_str(),
    )?;

    AuditService::log_action(
        &pool,
        &user,
        "PRIORITY_OVERRIDE",
        "ticket",
        ticket_id.as_str(),
        Some(json!({
            "priority": result.ticket_before.priority,
            "status": result.ticket_before.status,
            "rule_origin": "automatic"
        })),
        Some(json!({
            "priority": result.ticket_after.priority,
            "status": result.ticket_after.status,
            "rule_origin": "manual_override",
            "reason": result.reason
        })),
        None,
        None,
    )
    .log_if_err("Non-blocking operation failed");

    Ok(HttpResponse::Ok().json(result.ticket_after))
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-10: automatic completion path now emits downstream CDA generation trigger.
  - SBD-21: failure in CDA background dispatch does not compromise core workflow state.
  - SBD-24: completion recovery path improved through pending CDA queue behavior.
- Not fully satisfiable in this file:
  - SBD-08 secure endpoint transport is guaranteed by CDA module/config + infrastructure policy.
    Alternative: enforce TLS termination and network ACL around Mirth endpoint.
*/
