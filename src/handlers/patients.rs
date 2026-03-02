use crate::models::{SearchPatientsQuery, User};
use crate::services::PatientQueryService;
use crate::utils::ApiResult;
use crate::DbPool;
use actix_web::{get, web, HttpResponse};

/// Search patients by name, IPP, or service
/// GET /api/patients?search=dupont&limit=10
#[get("/api/patients")]
async fn search_patients(
    pool: web::Data<DbPool>,
    query: web::Query<SearchPatientsQuery>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let results = PatientQueryService::search(&pool, &query)?;
    Ok(HttpResponse::Ok().json(results))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(search_patients);
}
