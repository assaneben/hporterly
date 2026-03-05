use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Mutex;

use actix_web::{post, web, HttpRequest, HttpResponse};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::password_hash::{rand_core::RngCore, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use base64::{engine::general_purpose, Engine as _};
use data_encoding::BASE32_NOPAD;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Bool, Jsonb, Text};
use diesel::OptionalExtension;
use hmac::{Hmac, Mac};
use image::{ImageFormat, Luma};
use once_cell::sync::Lazy;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Sha256;
use subtle::ConstantTimeEq;

use crate::config::Config;
use crate::models::User;
use crate::services::{AuditService, AuthService};
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

type HmacSha256 = Hmac<Sha256>;

static MFA_ATTEMPTS: Lazy<Mutex<HashMap<String, Vec<i64>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

const BACKUP_CODES_COUNT: usize = 8;
const BACKUP_CODE_LENGTH: usize = 8;
const TOTP_DIGITS: u32 = 6;
const TOTP_PERIOD_SECONDS: i64 = 30;
const TOTP_WINDOW_STEPS: i64 = 1;

#[derive(Debug, Deserialize)]
pub struct MfaActivateRequest {
    pub code_totp: String,
}

#[derive(Debug, Deserialize)]
pub struct MfaVerifyRequest {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct MfaSetupResponse {
    pub qr_code_png_base64: String,
    pub secret_clair: String,
}

#[derive(Debug, Serialize)]
pub struct MfaActivateResponse {
    pub backup_codes: Vec<String>,
}

#[derive(Debug, QueryableByName)]
struct MfaSecretRow {
    #[diesel(sql_type = Text)]
    secret_enc: String,
    #[diesel(sql_type = Bool)]
    active: bool,
    #[diesel(sql_type = Jsonb)]
    backup_codes: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct TokenClaims {
    sub: String,
    role: String,
    exp: usize,
    #[serde(default)]
    mfa_verified: bool,
    #[serde(default = "default_token_kind")]
    token_kind: String,
}

fn default_token_kind() -> String {
    "access".to_string()
}

#[post("/api/auth/mfa/setup")]
async fn setup_mfa(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    user: web::ReqData<User>,
    req: HttpRequest,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let key = decode_encryption_key(config.mfa_encryption_key.as_str())?;
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    if let Some(row) = get_mfa_secret(&mut conn, user.id.as_str()).map_err(database_error)? {
        if row.active {
            return Err(ApiError::BadRequest(
                "MFA deja active pour cet utilisateur".to_string(),
            ));
        }
    }

    let mut secret_bytes = [0u8; 20];
    OsRng.fill_bytes(&mut secret_bytes);
    let secret_base32 = BASE32_NOPAD.encode(&secret_bytes);
    let secret_enc = encrypt_secret(&key, secret_base32.as_bytes())?;

    upsert_mfa_secret_inactive(&mut conn, user.id.as_str(), secret_enc.as_str())
        .map_err(database_error)?;

    let issuer = config.mfa_issuer.trim();
    let label = format!("{}:{}", issuer, user.username);
    let otpauth_url = format!(
        "otpauth://totp/{}?secret={}&issuer={}&algorithm=SHA256&digits={}&period={}",
        percent_encode(label.as_str()),
        secret_base32,
        percent_encode(issuer),
        TOTP_DIGITS,
        TOTP_PERIOD_SECONDS
    );
    let qr_code_png_base64 = render_qr_png_base64(otpauth_url.as_str())?;

    if let Err(err) = AuditService::log_action(
        &pool,
        &user,
        "MFA_SETUP",
        "auth",
        &user.id,
        None,
        Some(json!({ "event": "MFA_SETUP" })),
        request_ip(&req),
        request_user_agent(&req),
    ) {
        log::warn!(
            "Failed to write MFA_SETUP audit log for user {}: {}",
            user.id,
            err
        );
    }

    Ok(HttpResponse::Ok().json(MfaSetupResponse {
        qr_code_png_base64,
        secret_clair: secret_base32,
    }))
}

#[post("/api/auth/mfa/activate")]
async fn activate_mfa(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    user: web::ReqData<User>,
    req: HttpRequest,
    payload: web::Json<MfaActivateRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let key = decode_encryption_key(config.mfa_encryption_key.as_str())?;
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let row = get_mfa_secret(&mut conn, user.id.as_str())
        .map_err(database_error)?
        .ok_or_else(|| ApiError::BadRequest("MFA setup requis avant activation".to_string()))?;

    let secret_bytes = decrypt_secret_to_bytes(&key, row.secret_enc.as_str())?;
    let code = payload.code_totp.trim();
    let is_valid = validate_totp_window(secret_bytes.as_slice(), code, TOTP_WINDOW_STEPS)?;
    if !is_valid {
        if let Err(err) = AuditService::log_action(
            &pool,
            &user,
            "MFA_ECHEC",
            "auth",
            &user.id,
            None,
            Some(json!({ "event": "MFA_ACTIVATE_FAILED" })),
            request_ip(&req),
            request_user_agent(&req),
        ) {
            log::warn!(
                "Failed to write MFA_ECHEC audit log for user {}: {}",
                user.id,
                err
            );
        }
        return Err(ApiError::Unauthorized("Code TOTP invalide".to_string()));
    }

    let (backup_codes_clear, backup_hashes) = generate_backup_codes_hashes()?;
    activate_mfa_secret(
        &mut conn,
        user.id.as_str(),
        serde_json::to_string(&backup_hashes).map_err(|e| {
            ApiError::InternalServerError(format!("Backup code serialization error: {}", e))
        })?,
    )
    .map_err(database_error)?;

    if let Err(err) = AuditService::log_action(
        &pool,
        &user,
        "MFA_ACTIF",
        "auth",
        &user.id,
        None,
        Some(json!({ "event": "MFA_ACTIF" })),
        request_ip(&req),
        request_user_agent(&req),
    ) {
        log::warn!(
            "Failed to write MFA_ACTIF audit log for user {}: {}",
            user.id,
            err
        );
    }

    Ok(HttpResponse::Ok().json(MfaActivateResponse {
        backup_codes: backup_codes_clear,
    }))
}

#[post("/api/auth/mfa/verify")]
async fn verify_mfa(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    user: web::ReqData<User>,
    req: HttpRequest,
    payload: web::Json<MfaVerifyRequest>,
) -> ApiResult<HttpResponse> {
    let user = user.into_inner();
    let claims = decode_authorization_claims(&req, config.jwt_secret.as_str())?;
    if claims.sub != user.id || claims.token_kind != "mfa_tmp" {
        return Err(ApiError::Unauthorized(
            "Token MFA temporaire invalide".to_string(),
        ));
    }
    if claims.role != user.role {
        return Err(ApiError::Unauthorized(
            "Token MFA non coherent avec le role utilisateur".to_string(),
        ));
    }
    if claims.mfa_verified {
        return Err(ApiError::BadRequest(
            "Token MFA temporaire deja valide".to_string(),
        ));
    }
    let now_ts = chrono::Utc::now().timestamp() as usize;
    if claims.exp <= now_ts {
        return Err(ApiError::Unauthorized(
            "Token MFA temporaire expire".to_string(),
        ));
    }

    enforce_mfa_attempt_limit(user.id.as_str(), config.rate_limit_mfa_per_account)?;

    let key = decode_encryption_key(config.mfa_encryption_key.as_str())?;
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let row = get_mfa_secret(&mut conn, user.id.as_str())
        .map_err(database_error)?
        .ok_or_else(|| ApiError::Unauthorized("MFA non configure".to_string()))?;
    if !row.active {
        return Err(ApiError::Unauthorized("MFA non active".to_string()));
    }

    let secret_bytes = decrypt_secret_to_bytes(&key, row.secret_enc.as_str())?;
    let submitted_code = payload.code.trim();
    let is_backup_code = is_backup_code_candidate(submitted_code);
    let mut verified = false;

    if is_backup_code {
        let backup_hashes = parse_backup_hashes(&row.backup_codes)?;
        let (is_valid, remaining_hashes) =
            verify_and_consume_backup_code(submitted_code, backup_hashes)?;
        if is_valid {
            persist_backup_hashes(
                &mut conn,
                user.id.as_str(),
                serde_json::to_string(&remaining_hashes).map_err(|e| {
                    ApiError::InternalServerError(format!("Backup code serialization error: {}", e))
                })?,
            )
            .map_err(database_error)?;
            verified = true;
        }
    } else {
        verified =
            validate_totp_window(secret_bytes.as_slice(), submitted_code, TOTP_WINDOW_STEPS)?;
    }

    if !verified {
        register_mfa_attempt(user.id.as_str(), false)?;
        if let Err(err) = AuditService::log_action(
            &pool,
            &user,
            "MFA_ECHEC",
            "auth",
            &user.id,
            None,
            Some(json!({ "event": "MFA_VERIFY_FAILED" })),
            request_ip(&req),
            request_user_agent(&req),
        ) {
            log::warn!(
                "Failed to write MFA_ECHEC audit log for user {}: {}",
                user.id,
                err
            );
        }
        return Err(ApiError::Unauthorized("Code MFA invalide".to_string()));
    }

    register_mfa_attempt(user.id.as_str(), true)?;
    let token = AuthService::generate_full_access_token(&config, &user, true)?;
    let user_info = AuthService::me(&pool, &user)?;

    if let Err(err) = AuditService::log_action(
        &pool,
        &user,
        "CONNEXION",
        "auth",
        &user.id,
        None,
        Some(json!({ "event": "MFA_VERIFY_SUCCESS" })),
        request_ip(&req),
        request_user_agent(&req),
    ) {
        log::warn!(
            "Failed to write CONNEXION audit log for user {}: {}",
            user.id,
            err
        );
    }

    Ok(HttpResponse::Ok().json(json!({
        "token": token,
        "mfa_verified": true,
        "user": user_info
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(setup_mfa)
        .service(activate_mfa)
        .service(verify_mfa);
}

pub fn enforce_mfa_attempt_limit(user_id: &str, max_attempts: u32) -> ApiResult<()> {
    let now = chrono::Utc::now().timestamp();
    let mut guard = MFA_ATTEMPTS
        .lock()
        .map_err(|_| ApiError::InternalServerError("MFA limiter lock error".to_string()))?;
    let entry = guard.entry(user_id.to_string()).or_default();
    entry.retain(|ts| now - *ts <= 3600);
    if entry.len() >= max_attempts as usize {
        return Err(ApiError::TooManyRequests(
            "Trop de tentatives MFA. Reessayez dans une heure.".to_string(),
        ));
    }
    Ok(())
}

fn register_mfa_attempt(user_id: &str, success: bool) -> ApiResult<()> {
    let now = chrono::Utc::now().timestamp();
    let mut guard = MFA_ATTEMPTS
        .lock()
        .map_err(|_| ApiError::InternalServerError("MFA limiter lock error".to_string()))?;
    if success {
        guard.remove(user_id);
    } else {
        let entry = guard.entry(user_id.to_string()).or_default();
        entry.retain(|ts| now - *ts <= 3600);
        entry.push(now);
    }
    Ok(())
}

fn decode_authorization_claims(req: &HttpRequest, jwt_secret: &str) -> ApiResult<TokenClaims> {
    let header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ApiError::Unauthorized("Authorization header manquant".to_string()))?;
    let value = header
        .to_str()
        .map_err(|_| ApiError::Unauthorized("Authorization header invalide".to_string()))?;
    let token = value
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::Unauthorized("Authorization bearer manquant".to_string()))?;

    let token_data = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized("Token JWT invalide".to_string()))?;

    Ok(token_data.claims)
}

fn get_mfa_secret(conn: &mut PgConnection, user_id: &str) -> QueryResult<Option<MfaSecretRow>> {
    sql_query(
        "SELECT secret_enc, active, backup_codes
         FROM mfa_secrets
         WHERE user_id = $1",
    )
    .bind::<Text, _>(user_id)
    .get_result::<MfaSecretRow>(conn)
    .optional()
}

fn upsert_mfa_secret_inactive(
    conn: &mut PgConnection,
    user_id: &str,
    secret_enc: &str,
) -> QueryResult<usize> {
    sql_query(
        "INSERT INTO mfa_secrets (user_id, secret_enc, active, backup_codes, created_at, activated_at)
         VALUES ($1, $2, false, '[]'::jsonb, NOW(), NULL)
         ON CONFLICT (user_id) DO UPDATE
         SET secret_enc = EXCLUDED.secret_enc,
             active = false,
             backup_codes = '[]'::jsonb,
             activated_at = NULL",
    )
    .bind::<Text, _>(user_id)
    .bind::<Text, _>(secret_enc)
    .execute(conn)
}

fn activate_mfa_secret(
    conn: &mut PgConnection,
    user_id: &str,
    backup_codes_json: String,
) -> QueryResult<usize> {
    sql_query(
        "UPDATE mfa_secrets
         SET active = true,
             activated_at = NOW(),
             backup_codes = $2::jsonb
         WHERE user_id = $1",
    )
    .bind::<Text, _>(user_id)
    .bind::<Text, _>(backup_codes_json)
    .execute(conn)
}

fn persist_backup_hashes(
    conn: &mut PgConnection,
    user_id: &str,
    backup_codes_json: String,
) -> QueryResult<usize> {
    sql_query(
        "UPDATE mfa_secrets
         SET backup_codes = $2::jsonb
         WHERE user_id = $1",
    )
    .bind::<Text, _>(user_id)
    .bind::<Text, _>(backup_codes_json)
    .execute(conn)
}

fn parse_backup_hashes(value: &serde_json::Value) -> ApiResult<Vec<String>> {
    serde_json::from_value::<Vec<String>>(value.clone())
        .map_err(|_| ApiError::InternalServerError("Format backup_codes invalide".to_string()))
}

fn verify_and_consume_backup_code(
    submitted_code: &str,
    backup_hashes: Vec<String>,
) -> ApiResult<(bool, Vec<String>)> {
    let mut remaining = Vec::with_capacity(backup_hashes.len());
    let mut matched = false;

    for hash in backup_hashes {
        let parsed = PasswordHash::new(&hash).map_err(|e| {
            ApiError::InternalServerError(format!("Backup code hash parse error: {}", e))
        })?;
        let is_match = Argon2::default()
            .verify_password(submitted_code.as_bytes(), &parsed)
            .is_ok();
        if is_match && !matched {
            matched = true;
            continue;
        }
        remaining.push(hash);
    }

    Ok((matched, remaining))
}

fn generate_backup_codes_hashes() -> ApiResult<(Vec<String>, Vec<String>)> {
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();
    let mut clear_codes = Vec::with_capacity(BACKUP_CODES_COUNT);
    let mut hashed_codes = Vec::with_capacity(BACKUP_CODES_COUNT);

    for _ in 0..BACKUP_CODES_COUNT {
        let code = (0..BACKUP_CODE_LENGTH)
            .map(|_| {
                let idx = rand::Rng::gen_range(&mut rng, 0..ALPHABET.len());
                ALPHABET[idx] as char
            })
            .collect::<String>();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hash = Argon2::default()
            .hash_password(code.as_bytes(), &salt)
            .map_err(|e| ApiError::InternalServerError(format!("Backup code hash error: {}", e)))?
            .to_string();
        clear_codes.push(code);
        hashed_codes.push(hash);
    }

    Ok((clear_codes, hashed_codes))
}

fn validate_totp_window(secret: &[u8], submitted_code: &str, window_steps: i64) -> ApiResult<bool> {
    if !is_totp_code(submitted_code) {
        return Ok(false);
    }
    let step_now = chrono::Utc::now().timestamp() / TOTP_PERIOD_SECONDS;
    for delta in -window_steps..=window_steps {
        let expected = generate_totp_code(secret, step_now + delta)?;
        if expected.as_bytes().ct_eq(submitted_code.as_bytes()).into() {
            return Ok(true);
        }
    }
    Ok(false)
}

fn generate_totp_code(secret: &[u8], counter: i64) -> ApiResult<String> {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(secret)
        .map_err(|_| ApiError::InternalServerError("TOTP HMAC init error".to_string()))?;
    mac.update(&(counter as u64).to_be_bytes());
    let digest = mac.finalize().into_bytes();
    let offset = (digest[digest.len() - 1] & 0x0f) as usize;
    if offset + 4 > digest.len() {
        return Err(ApiError::InternalServerError(
            "TOTP offset out of range".to_string(),
        ));
    }

    let binary = ((u32::from(digest[offset]) & 0x7f) << 24)
        | (u32::from(digest[offset + 1]) << 16)
        | (u32::from(digest[offset + 2]) << 8)
        | u32::from(digest[offset + 3]);
    let modulo = 10_u32.pow(TOTP_DIGITS);
    Ok(format!("{:06}", binary % modulo))
}

fn decode_encryption_key(encoded_key: &str) -> ApiResult<[u8; 32]> {
    let decoded = general_purpose::STANDARD
        .decode(encoded_key.trim())
        .map_err(|_| {
            ApiError::InternalServerError("MFA_ENCRYPTION_KEY must be valid base64".to_string())
        })?;
    if decoded.len() != 32 {
        return Err(ApiError::InternalServerError(
            "MFA_ENCRYPTION_KEY must decode to exactly 32 bytes".to_string(),
        ));
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&decoded);
    Ok(key)
}

fn encrypt_secret(key: &[u8; 32], plaintext: &[u8]) -> ApiResult<String> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| ApiError::InternalServerError("AES-256-GCM key error".to_string()))?;
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext)
        .map_err(|_| ApiError::InternalServerError("MFA secret encryption failed".to_string()))?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(general_purpose::STANDARD.encode(out))
}

fn decrypt_secret_to_bytes(key: &[u8; 32], secret_enc: &str) -> ApiResult<Vec<u8>> {
    let raw = general_purpose::STANDARD.decode(secret_enc).map_err(|_| {
        ApiError::InternalServerError("Invalid encrypted MFA secret format".to_string())
    })?;
    if raw.len() < 13 {
        return Err(ApiError::InternalServerError(
            "Invalid encrypted MFA secret length".to_string(),
        ));
    }
    let (nonce_bytes, ciphertext) = raw.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| ApiError::InternalServerError("AES-256-GCM key error".to_string()))?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| ApiError::InternalServerError("MFA secret decryption failed".to_string()))?;

    BASE32_NOPAD
        .decode(plaintext.as_slice())
        .map_err(|_| ApiError::InternalServerError("MFA secret decode error".to_string()))
}

fn is_totp_code(value: &str) -> bool {
    value.len() == TOTP_DIGITS as usize && value.as_bytes().iter().all(u8::is_ascii_digit)
}

fn is_backup_code_candidate(value: &str) -> bool {
    value.len() == BACKUP_CODE_LENGTH && value.as_bytes().iter().all(u8::is_ascii_alphanumeric)
}

fn render_qr_png_base64(value: &str) -> ApiResult<String> {
    let code = QrCode::new(value.as_bytes())
        .map_err(|e| ApiError::InternalServerError(format!("QR generation error: {}", e)))?;
    let image = code.render::<Luma<u8>>().build();
    let mut png_bytes = Vec::new();
    image::DynamicImage::ImageLuma8(image)
        .write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)
        .map_err(|e| ApiError::InternalServerError(format!("QR PNG serialization error: {}", e)))?;
    Ok(general_purpose::STANDARD.encode(png_bytes))
}

fn request_ip(req: &HttpRequest) -> Option<String> {
    req.peer_addr().map(|addr| addr.ip().to_string())
}

fn request_user_agent(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("User-Agent")
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

fn percent_encode(input: &str) -> String {
    input
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{:02X}", byte).chars().collect(),
        })
        .collect()
}

fn database_error(error: diesel::result::Error) -> ApiError {
    ApiError::InternalServerError(format!("Database query error: {}", error))
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-01: strict input validation for TOTP/backup codes and token claims.
  - SBD-04: MFA enrollment/activation/verification with strong auth and temporary token isolation.
  - SBD-05: default deny on invalid token, invalid codes, or inconsistent claims.
  - SBD-07: encryption key sourced from environment; no hardcoded secrets.
  - SBD-08: AES-256-GCM for secret at rest; Argon2id for backup code hashing.
  - SBD-10: auditable events (MFA_SETUP, MFA_ACTIF, CONNEXION, MFA_ECHEC).
  - SBD-11: per-account MFA attempt limiting with one-hour window.
  - SBD-13: generic external errors, detailed internals server-side only.
  - SBD-21: fail-secure control flow.
  - SBD-22: explicit security workflow and bounded handlers.
- Not fully satisfiable in this file:
  - SBD-24 (durable incident response/retention execution) depends on operations and storage policies.
    Alternative: enforce immutable audit retention and alerting in deployment/DB policy.
  - SBD-25 compliance evidence (regulatory mapping artifacts) requires governance documentation.
    Alternative: attach compliance matrix and evidence checklist at release gates.
*/
