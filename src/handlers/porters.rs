use crate::models::{CreatePorterRequest, UpdatePorterSkills, UpdatePorterStatus, User};
use crate::services::PorterManagementService;
use crate::utils::ApiResult;
use crate::DbPool;
use actix_web::{get, patch, post, web, HttpResponse};

/// Lister tous les brancardiers
#[get("/api/porters")]
async fn list_porters(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let porters = PorterManagementService::list_porters(&pool)?;
    Ok(HttpResponse::Ok().json(porters))
}

/// Liste des brancardiers disponibles
#[get("/api/porters/available")]
async fn list_available_porters(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let porters = PorterManagementService::list_available_porters(&pool)?;
    Ok(HttpResponse::Ok().json(porters))
}

/// Obtenir un brancardier par ID
#[get("/api/porters/{id}")]
async fn get_porter(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
    porter_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let porter = PorterManagementService::get_porter(&pool, porter_id.as_str())?;
    Ok(HttpResponse::Ok().json(porter))
}

/// Mettre a jour le statut d'un brancardier
#[patch("/api/porters/{id}/status")]
async fn update_porter_status(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    porter_id: web::Path<String>,
    request: web::Json<UpdatePorterStatus>,
) -> ApiResult<HttpResponse> {
    let porter = PorterManagementService::update_porter_status(
        &pool,
        &user,
        porter_id.as_str(),
        request.into_inner(),
    )?;
    Ok(HttpResponse::Ok().json(porter))
}

/// Creer un nouveau brancardier (Admin uniquement)
#[post("/api/porters")]
async fn create_porter(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    request: web::Json<CreatePorterRequest>,
) -> ApiResult<HttpResponse> {
    let porter = PorterManagementService::create_porter(&pool, &user, request.into_inner())?;
    Ok(HttpResponse::Created().json(porter))
}

/// Mettre a jour les competences d'un brancardier (Admin uniquement)
#[patch("/api/porters/{id}/skills")]
async fn update_porter_skills(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    porter_id: web::Path<String>,
    request: web::Json<UpdatePorterSkills>,
) -> ApiResult<HttpResponse> {
    let porter = PorterManagementService::update_porter_skills(
        &pool,
        &user,
        porter_id.as_str(),
        request.into_inner(),
    )?;
    Ok(HttpResponse::Ok().json(porter))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_porters)
        .service(list_available_porters)
        .service(get_porter)
        .service(update_porter_status)
        .service(create_porter)
        .service(update_porter_skills);
}
