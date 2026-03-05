use super::*;

pub(super) async fn assign_ticket_impl(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<AssignTicketRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();
    let updated_ticket =
        TicketAssignmentActionService::assign_from_pool(&pool, &user, ticket_id.as_str(), request)?;
    Ok(HttpResponse::Ok().json(updated_ticket))
}

#[post("/api/tickets/{id}/assign")]
pub(super) async fn assign_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<AssignTicketRequest>,
) -> ApiResult<HttpResponse> {
    assign_ticket_impl(pool, user, ticket_id, request).await
}

#[post("/api/tickets/{id}/take")]
pub(super) async fn take_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<AssignTicketRequest>,
) -> ApiResult<HttpResponse> {
    assign_ticket_impl(pool, user, ticket_id, request).await
}

/// Ajouter un co-partner a une mission (supervisor ou admin)
#[post("/api/tickets/{id}/co-partners")]
pub(super) async fn add_co_partner(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<AddCoPartnerRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();
    let assignment = TicketAssignmentActionService::add_co_partner_from_pool(
        &pool,
        &user,
        ticket_id.as_str(),
        &request.porter_id,
    )?;
    Ok(HttpResponse::Ok().json(assignment))
}

/// Retirer un co-partner d'une mission (supervisor ou admin)
#[delete("/api/tickets/{id}/co-partners/{porter_id}")]
pub(super) async fn remove_co_partner(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    path: web::Path<(String, String)>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let (ticket_id, porter_id) = path.into_inner();
    let assignment = TicketAssignmentActionService::remove_co_partner_from_pool(
        &pool,
        &user,
        ticket_id.as_str(),
        porter_id.as_str(),
    )?;
    Ok(HttpResponse::Ok().json(assignment))
}

/// Mettre a jour le statut d'une demande
#[patch("/api/tickets/{id}/status")]
pub(super) async fn update_ticket_status(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<UpdateTicketStatus>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();
    let updated_ticket = TicketAssignmentActionService::update_status_from_pool(
        &pool,
        &user,
        ticket_id.as_str(),
        request,
    )?;

    if crate::models::Ticket::normalize_status(updated_ticket.status.as_str()) == Some("completed")
    {
        cda::trigger_transport_report_generation(
            pool.get_ref().clone(),
            config.get_ref().clone(),
            updated_ticket.id.clone(),
        );
    }

    Ok(HttpResponse::Ok().json(updated_ticket))
}

/// Obtenir les recommandations IA pour un ticket
#[get("/api/tickets/{id}/recommendations")]
pub(super) async fn get_recommendations(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
    ticket_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let response =
        TicketAssignmentActionService::recommendations_from_pool(&pool, ticket_id.as_str())?;
    Ok(HttpResponse::Ok().json(response))
}

/// Mettre une mission en pause (admin uniquement)
#[post("/api/tickets/{id}/pause")]
pub(super) async fn pause_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let updated_ticket =
        TicketAssignmentActionService::pause_from_pool(&pool, &user, ticket_id.as_str())?;
    Ok(HttpResponse::Ok().json(updated_ticket))
}

/// Reassign a mission to another porter (admin or supervisor)
#[post("/api/tickets/{id}/reassign")]
pub(super) async fn reassign_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<AssignTicketRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();
    let updated_ticket = TicketAssignmentActionService::reassign_from_pool(
        &pool,
        &user,
        ticket_id.as_str(),
        request,
    )?;
    Ok(HttpResponse::Ok().json(updated_ticket))
}

/// Renvoyer une mission en attente (desassigner) (admin ou supervisor)
#[post("/api/tickets/{id}/unassign")]
pub(super) async fn unassign_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: Option<web::Json<DesignateSuccessorRequest>>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.map(web::Json::into_inner);
    let updated_ticket = TicketAssignmentActionService::unassign_from_pool(
        &pool,
        &user,
        ticket_id.as_str(),
        request,
    )?;
    Ok(HttpResponse::Ok().json(updated_ticket))
}

/// Annuler une mission (admin uniquement)
#[derive(Debug, Deserialize)]
struct CancelTicketRequest {
    reason_code: Option<String>,
    comment: Option<String>,
}

#[post("/api/tickets/{id}/cancel")]
pub(super) async fn cancel_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: Option<web::Json<CancelTicketRequest>>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.map(web::Json::into_inner);
    let updated_ticket = TicketAssignmentActionService::cancel_from_pool(
        &pool,
        &user,
        ticket_id.as_str(),
        request
            .as_ref()
            .and_then(|payload| payload.reason_code.clone()),
        request.as_ref().and_then(|payload| payload.comment.clone()),
    )?;
    Ok(HttpResponse::Ok().json(updated_ticket))
}

#[derive(Debug, Deserialize)]
struct HardDeleteTicketRequest {
    reason: Option<String>,
}

/// Supprimer definitivement une demande (admin, ou demandeur proprietaire)
#[post("/api/tickets/{id}/hard-delete")]
pub(super) async fn hard_delete_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<HardDeleteTicketRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();
    let response = TicketAssignmentActionService::hard_delete_from_pool(
        &pool,
        &user,
        ticket_id.as_str(),
        request.reason,
    )?;
    Ok(HttpResponse::Ok().json(response))
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-10: completion events are auditable through downstream CDA dispatch logs.
  - SBD-21: CDA dispatch trigger is best-effort background and does not bypass status controls.
  - SBD-24: completed transport now triggers report generation for continuity/recovery.
- Not fully satisfiable in this file:
  - SBD-08 secure transport to Mirth is enforced by internal endpoint policy/config and infra.
    Alternative: enforce mTLS/TLS at Mirth ingress with certificate pinning policy.
*/
