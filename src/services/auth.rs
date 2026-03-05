use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Bool, Text};
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
    #[serde(default)]
    mfa_verified: bool,
    token_kind: String,
}

#[derive(Debug, QueryableByName)]
struct MfaActiveRow {
    #[diesel(sql_type = Bool)]
    active: bool,
}

pub struct AuthService;

impl AuthService {
    pub fn login(
        pool: &DbPool,
        config: &Config,
        credentials: LoginRequest,
        source_ip: Option<&str>,
    ) -> ApiResult<LoginResponse> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let user = Self::find_user_for_login(&mut conn, &credentials, source_ip)?;

        if !user.is_active {
            return Err(ApiError::Unauthorized("Compte desactive".to_string()));
        }

        if !Self::verify_password(&credentials.password, &user) {
            return Err(ApiError::Unauthorized(
                "Nom d'utilisateur ou mot de passe incorrect".to_string(),
            ));
        }

        if Self::is_mfa_active(&mut conn, user.id.as_str())? {
            let jwt_tmp = Self::generate_mfa_temporary_token(config, &user)?;
            return Ok(LoginResponse {
                token: None,
                user: None,
                mfa_required: true,
                session_token_partiel: Some(jwt_tmp),
                mfa_verified: Some(false),
            });
        }

        let token = Self::generate_full_access_token(config, &user, true)?;
        let user_info = Self::build_user_info(&mut conn, &user)?;

        log::info!("User {} logged in successfully", user.username);
        Ok(LoginResponse {
            token: Some(token),
            user: Some(user_info),
            mfa_required: false,
            session_token_partiel: None,
            mfa_verified: Some(true),
        })
    }

    pub fn me(pool: &DbPool, user: &User) -> ApiResult<UserInfo> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;
        Self::build_user_info(&mut conn, user)
    }

    pub fn generate_full_access_token(
        config: &Config,
        user: &User,
        mfa_verified: bool,
    ) -> ApiResult<String> {
        Self::generate_token(config, user, mfa_verified, "access", config.jwt_expiration)
    }

    pub fn generate_mfa_temporary_token(config: &Config, user: &User) -> ApiResult<String> {
        Self::generate_token(config, user, false, "mfa_tmp", 300)
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

    fn generate_token(
        config: &Config,
        user: &User,
        mfa_verified: bool,
        token_kind: &str,
        expires_in_seconds: i64,
    ) -> ApiResult<String> {
        let exp = (chrono::Utc::now() + chrono::Duration::seconds(expires_in_seconds)).timestamp()
            as usize;

        let claims = Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
            exp,
            mfa_verified,
            token_kind: token_kind.to_string(),
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

    fn find_user_for_login(
        conn: &mut diesel::PgConnection,
        credentials: &LoginRequest,
        source_ip: Option<&str>,
    ) -> ApiResult<User> {
        let email = credentials
            .email
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let username = credentials
            .username
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());

        let result = if let Some(email_value) = email {
            UserRepository::find_by_email(conn, email_value)
        } else if let Some(username_value) = username {
            UserRepository::find_by_username(conn, username_value)
        } else {
            return Err(ApiError::BadRequest(
                "Le champ 'email' ou 'username' est requis".to_string(),
            ));
        };

        result.map_err(|e| {
            let identity = email.or(username).unwrap_or("unknown");
            log::warn!(
                "Authentication failure for identity '{}', ip '{:?}': {}",
                identity,
                source_ip,
                e
            );
            ApiError::Unauthorized("Nom d'utilisateur ou mot de passe incorrect".to_string())
        })
    }

    fn is_mfa_active(conn: &mut diesel::PgConnection, user_id: &str) -> ApiResult<bool> {
        let result = sql_query(
            "SELECT active
             FROM mfa_secrets
             WHERE user_id = $1",
        )
        .bind::<Text, _>(user_id)
        .get_result::<MfaActiveRow>(conn)
        .optional()
        .map_err(|e| ApiError::InternalServerError(format!("Failed to read MFA state: {}", e)))?;

        Ok(result.map(|row| row.active).unwrap_or(false))
    }
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-04: MFA-aware login flow with temporary restricted token and full token post-verification.
  - SBD-05: strict deny on missing identity or invalid credentials.
  - SBD-07: signing material sourced from runtime configuration.
  - SBD-11: temporary token constrained to 5-minute validity.
  - SBD-21: fail-secure defaults for MFA state resolution errors.
- Not fully satisfiable in this file:
  - SBD-10 complete login-failure audit for unknown identities requires a dedicated auth attempt log model.
    Alternative: add append-only auth_attempts table not tied to existing user FK constraints.
*/
