use crate::models::User;
use crate::services::{AdminUserService, CreateUserRequest, UpdateUserRequest};
use crate::utils::ApiResult;
use crate::DbPool;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("/api/users")]
async fn list_users(pool: web::Data<DbPool>, user: web::ReqData<User>) -> ApiResult<HttpResponse> {
    let users = AdminUserService::list_users(&pool, &user)?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/api/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<CreateUserRequest>,
) -> ApiResult<HttpResponse> {
    let created = AdminUserService::create_user(&pool, &user, body.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

#[put("/api/users/{id}")]
async fn update_user(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    user_id: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> ApiResult<HttpResponse> {
    let updated = AdminUserService::update_user(&pool, &user, user_id.as_str(), body.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/api/users/{id}")]
async fn deactivate_user(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    user_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let payload = AdminUserService::deactivate_user(&pool, &user, user_id.as_str())?;
    Ok(HttpResponse::Ok().json(payload))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users).service(create_user).service(update_user).service(deactivate_user);
}
