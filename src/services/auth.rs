use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::models::{LoginRequest, LoginResponse, User, UserInfo};
use crate::repositories::{PorterRepository, UserRepository};
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    username: String,
    role: String,
    exp: usize,
}

pub struct AuthService;

impl AuthService {
    pub fn login(
        pool: &DbPool,
        config: &Config,
        credentials: LoginRequest,
    ) -> ApiResult<LoginResponse> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let user =
            UserRepository::find_by_username(&mut conn, &credentials.username).map_err(|e| {
                log::error!(
                    "Database error during login for user '{}': {}",
                    credentials.username,
                    e
                );
                ApiError::Unauthorized("Nom d'utilisateur ou mot de passe incorrect".to_string())
            })?;

        if !user.is_active {
            return Err(ApiError::Unauthorized("Compte desactive".to_string()));
        }

        if !Self::verify_password(&credentials.password, &user) {
            return Err(ApiError::Unauthorized(
                "Nom d'utilisateur ou mot de passe incorrect".to_string(),
            ));
        }

        let token = Self::generate_token(config, &user)?;
        let user_info = Self::build_user_info(&mut conn, &user)?;

        log::info!("User {} logged in successfully", credentials.username);
        Ok(LoginResponse {
            token,
            user: user_info,
        })
    }

    pub fn me(pool: &DbPool, user: &User) -> ApiResult<UserInfo> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;
        Self::build_user_info(&mut conn, user)
    }

    fn verify_password(password: &str, user: &User) -> bool {
        PasswordHash::new(&user.password_hash)
            .map(|parsed_hash| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
            })
            .unwrap_or_else(|err| {
                log::warn!(
                    "Invalid password hash for user '{}': {}",
                    user.username,
                    err
                );
                false
            })
    }

    fn generate_token(config: &Config, user: &User) -> ApiResult<String> {
        let exp = (chrono::Utc::now() + chrono::Duration::seconds(config.jwt_expiration))
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )
        .map_err(|e| ApiError::InternalServerError(format!("Token generation error: {}", e)))
    }

    fn build_user_info(conn: &mut diesel::PgConnection, user: &User) -> ApiResult<UserInfo> {
        let porter_id = if user.role == "brancardier" {
            match PorterRepository::find_id_by_user_id_optional(conn, &user.id) {
                Ok(value) => value,
                Err(err) => {
                    log::debug!("No porter profile for user {}: {}", user.id, err);
                    None
                }
            }
        } else {
            None
        };

        Ok(UserInfo {
            id: user.id.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            service: user.service.clone(),
            porter_id,
        })
    }
}
