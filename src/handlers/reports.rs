use std::collections::HashMap;

use actix_web::{get, web, HttpResponse};

use crate::models::User;
use crate::services::{OperationalReportsQuery, ReportsService};
use crate::utils::ApiResult;
use crate::DbPool;

#[get("/api/reports/operations")]
async fn get_operational_reports(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    query: web::Query<HashMap<String, String>>,
) -> ApiResult<HttpResponse> {
    let request = OperationalReportsQuery::from_query(&query.into_inner())?;
    let payload = ReportsService::get_operational_dataset(&pool, &user, request)?;
    Ok(HttpResponse::Ok().json(payload))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_operational_reports);
}
