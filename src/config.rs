use std::{env, num::ParseIntError};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_exp_minutes: i64,
    pub cors_allowed_origins: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid integer in {0}: {1}")]
    ParseInt(&'static str, #[source] ParseIntError),
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let app_port = parse_u16("APP_PORT", 8080)?;
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://hporterly_user:change_me_placeholder@localhost:5432/hporterly".to_string()
        });
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "replace_with_a_long_random_value".to_string());
        let jwt_issuer = env::var("JWT_ISSUER").unwrap_or_else(|_| "hporterly-backend".to_string());
        let jwt_exp_minutes = parse_i64("JWT_EXP_MINUTES", 60)?;
        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(ToOwned::to_owned)
            .collect();

        Ok(Self {
            app_host,
            app_port,
            database_url,
            jwt_secret,
            jwt_issuer,
            jwt_exp_minutes,
            cors_allowed_origins,
        })
    }
}

fn parse_u16(name: &'static str, default: u16) -> Result<u16, ConfigError> {
    match env::var(name) {
        Ok(v) => v.parse::<u16>().map_err(|e| ConfigError::ParseInt(name, e)),
        Err(_) => Ok(default),
    }
}

fn parse_i64(name: &'static str, default: i64) -> Result<i64, ConfigError> {
    match env::var(name) {
        Ok(v) => v.parse::<i64>().map_err(|e| ConfigError::ParseInt(name, e)),
        Err(_) => Ok(default),
    }
}
