use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct AuthService {
    cfg: Arc<AppConfig>,
    demo_user_hash: Arc<String>,
}

impl AuthService {
    pub fn new(cfg: Arc<AppConfig>) -> Result<Self, AppError> {
        let demo_user_hash = hash_password("demo-password")?;
        Ok(Self {
            cfg,
            demo_user_hash: Arc::new(demo_user_hash),
        })
    }

    pub fn verify_demo_credentials(&self, username: &str, password: &str) -> Result<(), AppError> {
        if username.trim().is_empty() {
            return Err(AppError::Unauthorized("username is required".to_string()));
        }
        verify_password(&self.demo_user_hash, password)?;
        Ok(())
    }

    pub fn issue_token(&self, subject: &str, role: &str) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = now + Duration::minutes(self.cfg.jwt_exp_minutes);
        let claims = Claims {
            sub: subject.to_string(),
            role: role.to_string(),
            iss: self.cfg.jwt_issuer.clone(),
            iat: now.timestamp() as usize,
            exp: exp.timestamp() as usize,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.cfg.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| AppError::Internal(format!("password hashing failed: {err}")))
}

fn verify_password(hash: &str, password: &str) -> Result<(), AppError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|err| AppError::Internal(format!("password hash parse failed: {err}")))?;
    let argon2 = Argon2::default();
    argon2
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized("invalid credentials".to_string()))
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn argon2_round_trip() {
        let hash = hash_password("demo-password").expect("hash");
        verify_password(&hash, "demo-password").expect("verify");
    }
}
