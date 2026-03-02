use crate::models::{
    CreateReferentialEquipmentRequest, CreateReferentialServiceRequest,
    CreateReferentialSpecimenRequest, CreateReferentialTransportModeRequest,
    UpdateReferentialEquipmentRequest, UpdateReferentialServiceRequest,
    UpdateReferentialSpecimenRequest, UpdateReferentialTransportModeRequest, User,
};
use crate::services::ReferentialCatalogService;
use crate::utils::ApiResult;
use crate::DbPool;
use actix_web::{delete, get, post, put, web, HttpResponse};

// ===================== Services =====================

#[get("/api/referentials/services")]
async fn list_services_admin(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_services_admin(&pool, &user)?;
    Ok(HttpResponse::Ok().json(items))
}

#[get("/api/referentials/services/active")]
async fn list_services_active(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_services_active(&pool)?;
    Ok(HttpResponse::Ok().json(items))
}

#[post("/api/referentials/services")]
async fn create_service(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<CreateReferentialServiceRequest>,
) -> ApiResult<HttpResponse> {
    let created = ReferentialCatalogService::create_service(&pool, &user, body.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

#[put("/api/referentials/services/{id}")]
async fn update_service(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    service_id: web::Path<String>,
    body: web::Json<UpdateReferentialServiceRequest>,
) -> ApiResult<HttpResponse> {
    let updated = ReferentialCatalogService::update_service(
        &pool,
        &user,
        service_id.as_str(),
        body.into_inner(),
    )?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/services/{id}")]
async fn deactivate_service(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    service_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let updated = ReferentialCatalogService::deactivate_service(&pool, &user, service_id.as_str())?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/services/{id}/hard")]
async fn hard_delete_service(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    service_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let payload =
        ReferentialCatalogService::hard_delete_service(&pool, &user, service_id.as_str())?;
    Ok(HttpResponse::Ok().json(payload))
}

// ===================== Equipment =====================

#[get("/api/referentials/equipment")]
async fn list_equipment_admin(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_equipment_admin(&pool, &user)?;
    Ok(HttpResponse::Ok().json(items))
}

#[get("/api/referentials/equipment/active")]
async fn list_equipment_active(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_equipment_active(&pool)?;
    Ok(HttpResponse::Ok().json(items))
}

#[post("/api/referentials/equipment")]
async fn create_equipment(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<CreateReferentialEquipmentRequest>,
) -> ApiResult<HttpResponse> {
    let created = ReferentialCatalogService::create_equipment(&pool, &user, body.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

#[put("/api/referentials/equipment/{id}")]
async fn update_equipment(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    equipment_id: web::Path<String>,
    body: web::Json<UpdateReferentialEquipmentRequest>,
) -> ApiResult<HttpResponse> {
    let updated = ReferentialCatalogService::update_equipment(
        &pool,
        &user,
        equipment_id.as_str(),
        body.into_inner(),
    )?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/equipment/{id}")]
async fn deactivate_equipment(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    equipment_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let updated =
        ReferentialCatalogService::deactivate_equipment(&pool, &user, equipment_id.as_str())?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/equipment/{id}/hard")]
async fn hard_delete_equipment(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    equipment_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let payload =
        ReferentialCatalogService::hard_delete_equipment(&pool, &user, equipment_id.as_str())?;
    Ok(HttpResponse::Ok().json(payload))
}

// ===================== Transport modes =====================

#[get("/api/referentials/transport-modes")]
async fn list_transport_modes_admin(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_transport_modes_admin(&pool, &user)?;
    Ok(HttpResponse::Ok().json(items))
}

#[get("/api/referentials/transport-modes/active")]
async fn list_transport_modes_active(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_transport_modes_active(&pool)?;
    Ok(HttpResponse::Ok().json(items))
}

#[post("/api/referentials/transport-modes")]
async fn create_transport_mode(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<CreateReferentialTransportModeRequest>,
) -> ApiResult<HttpResponse> {
    let created =
        ReferentialCatalogService::create_transport_mode(&pool, &user, body.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

#[put("/api/referentials/transport-modes/{id}")]
async fn update_transport_mode(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    mode_id: web::Path<String>,
    body: web::Json<UpdateReferentialTransportModeRequest>,
) -> ApiResult<HttpResponse> {
    let updated = ReferentialCatalogService::update_transport_mode(
        &pool,
        &user,
        mode_id.as_str(),
        body.into_inner(),
    )?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/transport-modes/{id}")]
async fn deactivate_transport_mode(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    mode_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let updated =
        ReferentialCatalogService::deactivate_transport_mode(&pool, &user, mode_id.as_str())?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/transport-modes/{id}/hard")]
async fn hard_delete_transport_mode(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    mode_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let payload =
        ReferentialCatalogService::hard_delete_transport_mode(&pool, &user, mode_id.as_str())?;
    Ok(HttpResponse::Ok().json(payload))
}

// ===================== Specimens =====================

#[get("/api/referentials/specimens")]
async fn list_specimens_admin(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_specimens_admin(&pool, &user)?;
    Ok(HttpResponse::Ok().json(items))
}

#[get("/api/referentials/specimens/active")]
async fn list_specimens_active(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let items = ReferentialCatalogService::list_specimens_active(&pool)?;
    Ok(HttpResponse::Ok().json(items))
}

#[post("/api/referentials/specimens")]
async fn create_specimen(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<CreateReferentialSpecimenRequest>,
) -> ApiResult<HttpResponse> {
    let created = ReferentialCatalogService::create_specimen(&pool, &user, body.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

#[put("/api/referentials/specimens/{id}")]
async fn update_specimen(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    specimen_id: web::Path<String>,
    body: web::Json<UpdateReferentialSpecimenRequest>,
) -> ApiResult<HttpResponse> {
    let updated = ReferentialCatalogService::update_specimen(
        &pool,
        &user,
        specimen_id.as_str(),
        body.into_inner(),
    )?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/specimens/{id}")]
async fn deactivate_specimen(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    specimen_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let updated =
        ReferentialCatalogService::deactivate_specimen(&pool, &user, specimen_id.as_str())?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/referentials/specimens/{id}/hard")]
async fn hard_delete_specimen(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    specimen_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let payload =
        ReferentialCatalogService::hard_delete_specimen(&pool, &user, specimen_id.as_str())?;
    Ok(HttpResponse::Ok().json(payload))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // services
        .service(list_services_admin)
        .service(list_services_active)
        .service(create_service)
        .service(update_service)
        .service(deactivate_service)
        .service(hard_delete_service)
        // equipment
        .service(list_equipment_admin)
        .service(list_equipment_active)
        .service(create_equipment)
        .service(update_equipment)
        .service(deactivate_equipment)
        .service(hard_delete_equipment)
        // transport modes
        .service(list_transport_modes_admin)
        .service(list_transport_modes_active)
        .service(create_transport_mode)
        .service(update_transport_mode)
        .service(deactivate_transport_mode)
        .service(hard_delete_transport_mode)
        // specimens
        .service(list_specimens_admin)
        .service(list_specimens_active)
        .service(create_specimen)
        .service(update_specimen)
        .service(deactivate_specimen)
        .service(hard_delete_specimen);
}
