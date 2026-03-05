use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use std::env;
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub hl7_internal_port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub enable_tls: bool,
    pub environment: String,
    pub cors_allowed_origins: Vec<String>,
    pub mirth_webhook_secret: String,
    pub mirth_allowed_ip: String,
    pub mfa_issuer: String,
    pub mfa_encryption_key: String,
    pub rate_limit_login_per_ip: u32,
    pub rate_limit_mfa_per_account: u32,
    pub rate_limit_api_per_user: u32,
    pub audit_retention_days: u32,
    pub cda_mirth_endpoint: String,
}

/// Get required environment variable with proper error handling
///
/// In production mode, missing required variables will panic.
/// In development mode, a warning is logged and a default value is returned.
pub fn get_required_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        #[cfg(not(debug_assertions))]
        {
            log::error!("Missing required environment variable: {}", key);
            panic!(
                "Missing required environment variable: {}. Please check your .env file.",
                key
            );
        }

        #[cfg(debug_assertions)]
        {
            log::warn!(
                "Using default value for missing environment variable: {}",
                key
            );
            format!("dev-{}", key)
        }
    })
}

/// Get optional environment variable with default
pub fn get_optional_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        log::debug!("Using default value '{}' for {}", default, key);
        default.to_string()
    })
}

fn parse_csv_env(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|origin| origin.trim())
        .filter(|origin| !origin.is_empty())
        .map(|origin| origin.to_string())
        .collect()
}

fn default_environment() -> &'static str {
    if cfg!(debug_assertions) {
        "development"
    } else {
        "production"
    }
}

impl Config {
    pub fn from_env() -> Self {
        let environment = env::var("APP_ENV")
            .or_else(|_| env::var("ENVIRONMENT"))
            .unwrap_or_else(|_| default_environment().to_string());

        Config {
            database_url: get_required_env("DATABASE_URL"),
            host: get_optional_env("HOST", "127.0.0.1"),
            port: get_optional_env("PORT", "8080")
                .parse()
                .expect("PORT must be a valid number"),
            hl7_internal_port: get_optional_env("HL7_INTERNAL_PORT", "8081")
                .parse()
                .expect("HL7_INTERNAL_PORT must be a valid number"),
            jwt_secret: get_required_env("JWT_SECRET"),
            jwt_expiration: get_optional_env("JWT_EXPIRATION", "86400")
                .parse()
                .expect("JWT_EXPIRATION must be a valid number"),
            enable_tls: get_optional_env("ENABLE_TLS", "false")
                .parse()
                .unwrap_or(false),
            environment,
            cors_allowed_origins: parse_csv_env(&get_optional_env("CORS_ALLOWED_ORIGINS", "")),
            mirth_webhook_secret: get_required_env("MIRTH_WEBHOOK_SECRET"),
            mirth_allowed_ip: get_optional_env("MIRTH_ALLOWED_IP", "127.0.0.1"),
            mfa_issuer: get_optional_env("MFA_ISSUER", "HPorterly"),
            mfa_encryption_key: get_optional_env(
                "MFA_ENCRYPTION_KEY",
                "CHANGEME_32_BYTES_BASE64_KEY",
            ),
            rate_limit_login_per_ip: get_optional_env("RATE_LIMIT_LOGIN_PER_IP", "10")
                .parse()
                .expect("RATE_LIMIT_LOGIN_PER_IP must be a valid positive number"),
            rate_limit_mfa_per_account: get_optional_env("RATE_LIMIT_MFA_PER_ACCOUNT", "5")
                .parse()
                .expect("RATE_LIMIT_MFA_PER_ACCOUNT must be a valid positive number"),
            rate_limit_api_per_user: get_optional_env("RATE_LIMIT_API_PER_USER", "300")
                .parse()
                .expect("RATE_LIMIT_API_PER_USER must be a valid positive number"),
            audit_retention_days: get_optional_env("AUDIT_RETENTION_DAYS", "3650")
                .parse()
                .expect("AUDIT_RETENTION_DAYS must be a valid positive number"),
            cda_mirth_endpoint: get_optional_env(
                "CDA_MIRTH_ENDPOINT",
                "http://localhost:6661/receive-cda",
            ),
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate JWT secret is strong enough
        if self.jwt_secret.len() < 32 {
            #[cfg(debug_assertions)]
            {
                log::warn!("JWT_SECRET is shorter than 32 characters (dev mode)");
            }

            #[cfg(not(debug_assertions))]
            {
                return Err("JWT_SECRET must be at least 32 characters long".to_string());
            }
        }

        // Validate JWT expiration is reasonable
        if self.jwt_expiration < 60 || self.jwt_expiration > 86400 * 7 {
            return Err("JWT_EXPIRATION must be between 60 seconds and 7 days".to_string());
        }

        let is_production = self.environment.eq_ignore_ascii_case("production");
        if is_production && self.cors_allowed_origins.is_empty() {
            return Err("CORS_ALLOWED_ORIGINS must be set in production".to_string());
        }

        if self.port == self.hl7_internal_port {
            return Err("PORT and HL7_INTERNAL_PORT must be different".to_string());
        }

        self.mirth_allowed_ip
            .parse::<IpAddr>()
            .map_err(|_| "MIRTH_ALLOWED_IP must be a valid IP address".to_string())?;

        if self.mirth_webhook_secret.len() < 32 {
            if is_production {
                return Err(
                    "MIRTH_WEBHOOK_SECRET must be at least 32 characters in production".to_string(),
                );
            }
            log::warn!("MIRTH_WEBHOOK_SECRET is shorter than 32 characters (dev mode)");
        }

        if self.mirth_webhook_secret == "CHANGEME_256_BIT_SECRET" {
            if is_production {
                return Err(
                    "MIRTH_WEBHOOK_SECRET uses placeholder value and must be replaced".to_string(),
                );
            }
            log::warn!("MIRTH_WEBHOOK_SECRET uses placeholder value (dev mode)");
        }

        if self.mfa_issuer.trim().is_empty() {
            return Err("MFA_ISSUER must not be empty".to_string());
        }

        let decoded_mfa_key = general_purpose::STANDARD
            .decode(self.mfa_encryption_key.trim())
            .map_err(|_| "MFA_ENCRYPTION_KEY must be valid base64".to_string())?;
        if decoded_mfa_key.len() != 32 {
            return Err("MFA_ENCRYPTION_KEY must decode to exactly 32 bytes".to_string());
        }

        if self.rate_limit_login_per_ip == 0
            || self.rate_limit_mfa_per_account == 0
            || self.rate_limit_api_per_user == 0
        {
            return Err("Rate limit values must be greater than zero".to_string());
        }
        if self.audit_retention_days < 3650 {
            return Err("AUDIT_RETENTION_DAYS must be at least 3650 (10 years)".to_string());
        }

        let endpoint = self.cda_mirth_endpoint.trim();
        if endpoint.is_empty() {
            return Err("CDA_MIRTH_ENDPOINT must not be empty".to_string());
        }
        if !endpoint.starts_with("http://localhost:6661/") {
            return Err(
                "CDA_MIRTH_ENDPOINT must target internal Mirth endpoint http://localhost:6661/"
                    .to_string(),
            );
        }

        Ok(())
    }
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-07: secrets sourced from environment variables, not hardcoded.
  - SBD-04/SBD-11: MFA and rate-limit policy variables validated at startup.
  - SBD-21: internal CDA dispatch endpoint policy validated at startup.
  - SBD-20: explicit internal HL7 bind port separation from public port.
  - SBD-21: fail-fast validation on insecure/misconfigured runtime settings.
  - SBD-22: startup policy checks centralized.
- Not fully satisfiable in this file:
  - SBD-08 (TLS 1.3 enforcement) cannot be fully guaranteed by env validation only.
    Alternative: enforce TLS 1.3 at reverse proxy/LB and set ENABLE_TLS policy in deployment manifests.
*/
