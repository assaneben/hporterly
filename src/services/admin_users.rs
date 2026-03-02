use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::{NewPorter, NewUser, User};
use crate::repositories::{PorterRepository, UserRepository, UserUpdateData};
use crate::utils::{require_admin, ApiError, ApiResult};
use crate::DbPool;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub service: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub service: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub id: String,
    pub username: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub service: Option<String>,
    pub porter_id: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct AdminUserService;

impl AdminUserService {
    pub fn list_users(pool: &DbPool, actor: &User) -> ApiResult<Vec<AdminUserResponse>> {
        require_admin(actor)?;
        let mut conn = Self::conn(pool)?;
        let records = UserRepository::list_with_porter_id(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to load users: {}", e)))?;
        Ok(records
            .into_iter()
            .map(|(u, porter_id)| Self::to_response(u, porter_id))
            .collect())
    }

    pub fn create_user(
        pool: &DbPool,
        actor: &User,
        body: CreateUserRequest,
    ) -> ApiResult<AdminUserResponse> {
        require_admin(actor)?;
        let mut conn = Self::conn(pool)?;

        let role = Self::normalize_role(&body.role)?;
        if UserRepository::find_by_username_optional(&mut conn, &body.username)
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to check existing user: {}", e))
            })?
            .is_some()
        {
            return Err(ApiError::BadRequest(
                "Ce nom d'utilisateur existe deja".to_string(),
            ));
        }

        let new_user = NewUser {
            id: format!("usr-{}", Uuid::new_v4()),
            username: body.username.clone(),
            password_hash: Self::hash_password(&body.password)?,
            role: role.clone(),
            first_name: body.first_name,
            last_name: body.last_name,
            email: body.email,
            service: body.service,
            is_active: true,
        };

        let created = UserRepository::insert(&mut conn, &new_user)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to create user: {}", e)))?;

        let mut porter_id: Option<String> = None;
        if role == "brancardier" {
            let new_porter_id = created.username.clone();
            if PorterRepository::find_by_id_optional(&mut conn, &new_porter_id)
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Failed to check existing porter: {}", e))
                })?
                .is_some()
            {
                return Err(ApiError::BadRequest(
                    "Cet identifiant brancardier existe deja".to_string(),
                ));
            }

            let new_porter = NewPorter {
                id: new_porter_id.clone(),
                user_id: created.id.clone(),
                status: "offline".to_string(),
                skills: Vec::new(),
            };

            PorterRepository::insert(&mut conn, &new_porter).map_err(|e| {
                ApiError::InternalServerError(format!("Failed to create porter profile: {}", e))
            })?;
            porter_id = Some(new_porter_id);
        }

        Ok(Self::to_response(created, porter_id))
    }

    pub fn update_user(
        pool: &DbPool,
        actor: &User,
        user_id: &str,
        body: UpdateUserRequest,
    ) -> ApiResult<AdminUserResponse> {
        require_admin(actor)?;
        let mut conn = Self::conn(pool)?;

        let existing = UserRepository::find_by_id(&mut conn, user_id)
            .map_err(|_| ApiError::NotFound(format!("User {} not found", user_id)))?;
        let new_role = body
            .role
            .as_ref()
            .map(|r| Self::normalize_role(r))
            .transpose()?
            .unwrap_or_else(|| existing.role.clone());

        if existing.role == "brancardier" {
            if let Some(ref new_username) = body.username {
                if new_username != &existing.username {
                    return Err(ApiError::BadRequest(
                        "Impossible de modifier le nom d'utilisateur d'un brancardier".to_string(),
                    ));
                }
            }
        }

        if new_role == "brancardier" {
            if let Some(ref new_username) = body.username {
                if new_username != &existing.username {
                    return Err(ApiError::BadRequest(
                        "Impossible de modifier le nom d'utilisateur d'un brancardier".to_string(),
                    ));
                }
            }

            if PorterRepository::find_by_user_id_optional(&mut conn, &existing.id)
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Failed to check porter profile: {}", e))
                })?
                .is_none()
            {
                let new_porter = NewPorter {
                    id: existing.username.clone(),
                    user_id: existing.id.clone(),
                    status: "offline".to_string(),
                    skills: Vec::new(),
                };
                PorterRepository::insert(&mut conn, &new_porter).map_err(|e| {
                    ApiError::InternalServerError(format!("Failed to create porter profile: {}", e))
                })?;
            }
        }

        let new_password_hash = if let Some(ref password) = body.password {
            if password.is_empty() {
                existing.password_hash.clone()
            } else {
                Self::hash_password(password)?
            }
        } else {
            existing.password_hash.clone()
        };

        let data = UserUpdateData {
            username: body.username.unwrap_or(existing.username),
            password_hash: new_password_hash,
            role: new_role,
            first_name: body.first_name.unwrap_or(existing.first_name),
            last_name: body.last_name.unwrap_or(existing.last_name),
            email: body.email.or(existing.email),
            service: body.service.or(existing.service),
            is_active: body.is_active.unwrap_or(existing.is_active),
        };

        let updated = UserRepository::update(&mut conn, user_id, data)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to update user: {}", e)))?;

        if !updated.is_active && updated.role == "brancardier" {
            if let Err(err) =
                PorterRepository::set_status_by_user_id(&mut conn, &updated.id, "offline")
            {
                log::warn!("Non-blocking operation failed: {}", err);
            }
        }

        let porter_id = PorterRepository::find_id_by_user_id_optional(&mut conn, &updated.id)
            .ok()
            .flatten();
        Ok(Self::to_response(updated, porter_id))
    }

    pub fn deactivate_user(pool: &DbPool, actor: &User, user_id: &str) -> ApiResult<Value> {
        require_admin(actor)?;
        let mut conn = Self::conn(pool)?;
        let updated = UserRepository::set_active(&mut conn, user_id, false).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to deactivate user: {}", e))
        })?;

        if updated.role == "brancardier" {
            if let Err(err) =
                PorterRepository::set_status_by_user_id(&mut conn, &updated.id, "offline")
            {
                log::warn!("Non-blocking operation failed: {}", err);
            }
        }

        Ok(json!({ "success": true }))
    }

    fn to_response(user: User, porter_id: Option<String>) -> AdminUserResponse {
        AdminUserResponse {
            id: user.id,
            username: user.username,
            role: user.role,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            service: user.service,
            porter_id,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    fn hash_password(plain: &str) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(plain.as_bytes(), &salt)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to hash password: {}", e)))
            .map(|h| h.to_string())
    }

    fn normalize_role(role: &str) -> Result<String, ApiError> {
        let normalized = role.trim().to_lowercase();
        match normalized.as_str() {
            "demandeur" | "brancardier" | "administrateur" | "regulateur" => Ok(normalized),
            _ => Err(ApiError::BadRequest("Role invalide".to_string())),
        }
    }

    fn conn(
        pool: &DbPool,
    ) -> ApiResult<
        diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    > {
        pool.get()
            .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))
    }
}
