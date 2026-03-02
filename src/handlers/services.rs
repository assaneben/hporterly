use crate::models::User;
use crate::services::HospitalServiceQueryService;
use crate::utils::ApiResult;
use crate::DbPool;
use actix_web::{get, web, HttpResponse};

/// Lister tous les services hospitaliers
#[get("/api/services")]
async fn list_services(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let services_list = HospitalServiceQueryService::list_all(&pool)?;
    Ok(HttpResponse::Ok().json(services_list))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_services);
}
