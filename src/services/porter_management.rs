use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde_json::json;

use crate::models::{
    CreatePorterRequest, NewPorter, NewUser, Porter, UpdatePorterSkills, UpdatePorterStatus, User,
};
use crate::repositories::{PorterRepository, UserRepository};
use crate::services::AuditService;
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

pub struct PorterManagementService;

impl PorterManagementService {
    pub fn list_porters(pool: &DbPool) -> ApiResult<Vec<Porter>> {
        let mut conn = Self::conn(pool)?;
        PorterRepository::list_ordered_by_status(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to fetch porters: {}", e)))
    }

    pub fn list_available_porters(pool: &DbPool) -> ApiResult<Vec<Porter>> {
        let mut conn = Self::conn(pool)?;
        PorterRepository::list_available(&mut conn).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to fetch available porters: {}", e))
        })
    }

    pub fn get_porter(pool: &DbPool, porter_id: &str) -> ApiResult<Porter> {
        let mut conn = Self::conn(pool)?;
        PorterRepository::find_by_id(&mut conn, porter_id)
            .map_err(|_| ApiError::NotFound(format!("Porter {} not found", porter_id)))
    }

    pub fn update_porter_status(
        pool: &DbPool,
        actor: &User,
        porter_id: &str,
        request: UpdatePorterStatus,
    ) -> ApiResult<Porter> {
        let mut conn = Self::conn(pool)?;
        let porter = PorterRepository::find_by_id(&mut conn, porter_id)
            .map_err(|_| ApiError::NotFound(format!("Porter {} not found", porter_id)))?;

        if actor.role == "brancardier" {
            if porter.user_id != actor.id && porter.id != actor.id {
                return Err(ApiError::Forbidden(
                    "Vous ne pouvez modifier que votre propre statut".to_string(),
                ));
            }
        } else if actor.role == "demandeur" {
            return Err(ApiError::Forbidden("Acces non autorise".to_string()));
        }

        let valid_statuses = ["available", "busy", "break", "offline"];
        if !valid_statuses.contains(&request.status.as_str()) {
            return Err(ApiError::BadRequest("Statut invalide".to_string()));
        }

        let updated_porter = PorterRepository::update_status_and_location(
            &mut conn,
            porter_id,
            request.status.as_str(),
            request.location.clone(),
        )
        .map_err(|e| ApiError::InternalServerError(format!("Failed to update porter: {}", e)))?;

        if let Err(err) = AuditService::log_action(
            pool,
            actor,
            "UPDATE_STATUS",
            "porter",
            porter_id,
            Some(json!({"status": porter.status, "location": porter.current_location})),
            Some(json!({"status": request.status, "location": request.location})),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        log::info!(
            "Porter {} status updated to {} by user {}",
            porter_id,
            request.status,
            actor.id
        );

        Ok(updated_porter)
    }

    pub fn create_porter(
        pool: &DbPool,
        actor: &User,
        request: CreatePorterRequest,
    ) -> ApiResult<Porter> {
        if actor.role != "administrateur" {
            return Err(ApiError::Forbidden("Admin access required".to_string()));
        }

        let mut conn = Self::conn(pool)?;
        if UserRepository::find_by_username_optional(&mut conn, &request.username)
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to validate username: {}", e))
            })?
            .is_some()
        {
            return Err(ApiError::BadRequest(
                "Ce nom d'utilisateur existe deja".to_string(),
            ));
        }

        let user_id = uuid::Uuid::new_v4().to_string();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(request.password.as_bytes(), &salt)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to hash password: {}", e)))?
            .to_string();

        let new_user = NewUser {
            id: user_id.clone(),
            username: request.username.clone(),
            password_hash,
            role: "brancardier".to_string(),
            first_name: request.first_name,
            last_name: request.last_name,
            email: None,
            service: Some("Brancardage".to_string()),
            is_active: true,
        };
        UserRepository::insert(&mut conn, &new_user)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to create user: {}", e)))?;

        let new_porter = NewPorter {
            id: request.username,
            user_id,
            status: "offline".to_string(),
            skills: request.skills.into_iter().map(Some).collect(),
        };

        let porter = PorterRepository::insert(&mut conn, &new_porter).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to create porter profile: {}", e))
        })?;

        if let Err(err) = AuditService::log_action(
            pool,
            actor,
            "CREATE",
            "porter",
            &porter.id,
            None,
            Some(json!(&porter)),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(porter)
    }

    pub fn update_porter_skills(
        pool: &DbPool,
        actor: &User,
        porter_id: &str,
        request: UpdatePorterSkills,
    ) -> ApiResult<Porter> {
        if actor.role != "administrateur" {
            return Err(ApiError::Forbidden("Admin access required".to_string()));
        }

        let mut conn = Self::conn(pool)?;
        let porter = PorterRepository::find_by_id(&mut conn, porter_id)
            .map_err(|_| ApiError::NotFound(format!("Porter {} not found", porter_id)))?;

        let normalized_skills: Vec<Option<String>> = request.skills.into_iter().map(Some).collect();
        let updated_porter =
            PorterRepository::update_skills(&mut conn, porter_id, normalized_skills).map_err(
                |e| ApiError::InternalServerError(format!("Failed to update skills: {}", e)),
            )?;

        if let Err(err) = AuditService::log_action(
            pool,
            actor,
            "UPDATE_SKILLS",
            "porter",
            porter_id,
            Some(json!({"skills": porter.skills})),
            Some(json!({"skills": &updated_porter.skills})),
            None,
            None,
        ) {
            log::warn!("Non-blocking operation failed: {}", err);
        }

        Ok(updated_porter)
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
