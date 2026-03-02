use super::*;

/// Demander de l'aide a un autre brancardier
#[post("/api/tickets/{id}/request-help")]
pub(super) async fn request_help(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
    request: web::Json<CreateHelpRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let result = TicketHelpService::request_help(&mut conn, &user, ticket_id.as_str(), &request)?;

    log::info!(
        "Help request {} created for ticket {} by porter {}",
        result.help_request_id,
        ticket_id,
        user.id
    );

    let requester_name = format!("{} {}", user.first_name, user.last_name);
    let notif = NotificationService::help_requested(ticket_id.as_str(), &requester_name);
    NotificationService::persist_if_enabled(
        &mut conn,
        &result.requested_porter_user_id,
        &notif,
        Some(ticket_id.as_str()),
    )
    .log_if_err("Non-blocking operation failed");

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "help_request_id": result.help_request_id,
        "message": "Demande d'aide envoyee"
    })))
}

/// Repondre a une demande d'aide
#[post("/api/help-requests/{id}/respond")]
pub(super) async fn respond_to_help(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    help_request_id: web::Path<String>,
    response: web::Json<RespondToHelpRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let response = response.into_inner();

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let result =
        TicketHelpService::respond_to_help(&mut conn, &user, help_request_id.as_str(), &response)?;

    log::info!("Help request {} {} by porter {}", help_request_id, result.status, user.id);

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "accepted": result.accepted,
        "status": result.status
    })))
}

/// Recuperer les demandes d'aide en attente pour le brancardier connecte
#[get("/api/porters/me/help-requests")]
pub(super) async fn get_my_help_requests(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let requests = TicketHelpService::get_my_pending_help_requests(&mut conn, &user)?;
    Ok(HttpResponse::Ok().json(requests))
}

/// Recuperer les brancardiers disponibles pour demander de l'aide
#[get("/api/tickets/{id}/available-porters-for-help")]
pub(super) async fn get_available_porters_for_help(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
    ticket_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let available_porters =
        TicketHelpService::get_available_porters_for_help(&mut conn, ticket_id.as_str())?;
    Ok(HttpResponse::Ok().json(available_porters))
}
