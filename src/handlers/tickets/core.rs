use super::*;

/// Creer une nouvelle demande de transport
#[post("/api/tickets")]
pub(super) async fn create_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    request: web::Json<CreateTicketRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let request = request.into_inner();
    let ticket = TicketCoreService::create_from_pool(&pool, &user, request)?;
    Ok(HttpResponse::Created().json(ticket))
}

/// Lister toutes les demandes (avec filtres optionnels)
#[get("/api/tickets")]
pub(super) async fn list_tickets(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> ApiResult<HttpResponse> {
    let query_map = query.into_inner();
    let params = TicketListParams::from_query(&query_map);
    let payload = TicketCoreService::list_payloads_from_pool(&pool, &user, params)?;
    Ok(HttpResponse::Ok().json(payload))
}

/// Obtenir une demande par ID
#[get("/api/tickets/{id}")]
pub(super) async fn get_ticket(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    ticket_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let payload = TicketCoreService::get_payload_from_pool(&pool, &user, ticket_id.as_str())?;
    Ok(HttpResponse::Ok().json(payload))
}
