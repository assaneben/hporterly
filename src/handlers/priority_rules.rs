use crate::models::{UpdatePriorityRulesRequest, User};
use crate::services::{AuditService, PriorityRulesService, PRIORITY_RULES_CONFIG_ID};
use crate::utils::{require_role, ApiError, ApiResult};
use crate::DbPool;
use actix_web::{get, post, put, web, HttpResponse};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap};

fn require_rules_manager(user: &User) -> ApiResult<()> {
    require_role(
        user,
        &["administrateur", "regulateur", "moderateur", "admin"],
    )
}

fn build_rule_map(config: &Value) -> HashMap<String, Value> {
    config
        .get("rules")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|rule| {
            let id = rule
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if id.is_empty() {
                None
            } else {
                Some((id, rule))
            }
        })
        .collect()
}

fn build_level_map(config: &Value) -> HashMap<i32, Value> {
    config
        .get("levels")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|level| {
            let priority = level.get("priority").and_then(|v| v.as_i64())? as i32;
            Some((priority, level))
        })
        .collect()
}

fn diff_rule_fields(old_rule: &Value, new_rule: &Value) -> Vec<String> {
    ["level", "enabled", "combinator", "conditions", "reason"]
        .into_iter()
        .filter(|field| old_rule.get(*field) != new_rule.get(*field))
        .map(|field| field.to_string())
        .collect()
}

fn diff_level_fields(old_level: &Value, new_level: &Value) -> Vec<String> {
    ["enabled", "label", "description", "key"]
        .into_iter()
        .filter(|field| old_level.get(*field) != new_level.get(*field))
        .map(|field| field.to_string())
        .collect()
}

fn collect_priority_rule_audit_entries(
    old_rules: &Value,
    new_rules: &Value,
) -> Vec<(String, String, Option<Value>, Option<Value>)> {
    let mut entries = Vec::new();

    let old_rule_map = build_rule_map(old_rules);
    let new_rule_map = build_rule_map(new_rules);
    let rule_ids: BTreeSet<String> = old_rule_map
        .keys()
        .chain(new_rule_map.keys())
        .cloned()
        .collect();

    for rule_id in rule_ids {
        match (old_rule_map.get(&rule_id), new_rule_map.get(&rule_id)) {
            (None, Some(next)) => entries.push((
                "PRIORITY_RULE_CREATE".to_string(),
                format!("rule:{}", rule_id),
                None,
                Some(next.clone()),
            )),
            (Some(prev), None) => entries.push((
                "PRIORITY_RULE_DELETE".to_string(),
                format!("rule:{}", rule_id),
                Some(prev.clone()),
                None,
            )),
            (Some(prev), Some(next)) if prev != next => {
                let changed_fields = diff_rule_fields(prev, next);
                entries.push((
                    "PRIORITY_RULE_UPDATE".to_string(),
                    format!("rule:{}", rule_id),
                    Some(json!({
                        "rule": prev,
                        "changed_fields": changed_fields
                    })),
                    Some(json!({
                        "rule": next,
                        "changed_fields": changed_fields
                    })),
                ));
            }
            _ => {}
        }
    }

    let old_level_map = build_level_map(old_rules);
    let new_level_map = build_level_map(new_rules);
    let level_ids: BTreeSet<i32> = old_level_map
        .keys()
        .chain(new_level_map.keys())
        .copied()
        .collect();

    for level in level_ids {
        let entity_id = format!("level:N{}", level);
        match (old_level_map.get(&level), new_level_map.get(&level)) {
            (None, Some(next)) => entries.push((
                "PRIORITY_LEVEL_CREATE".to_string(),
                entity_id,
                None,
                Some(next.clone()),
            )),
            (Some(prev), None) => entries.push((
                "PRIORITY_LEVEL_DELETE".to_string(),
                entity_id,
                Some(prev.clone()),
                None,
            )),
            (Some(prev), Some(next)) if prev != next => {
                let changed_fields = diff_level_fields(prev, next);
                entries.push((
                    "PRIORITY_LEVEL_UPDATE".to_string(),
                    entity_id,
                    Some(json!({
                        "level": prev,
                        "changed_fields": changed_fields
                    })),
                    Some(json!({
                        "level": next,
                        "changed_fields": changed_fields
                    })),
                ));
            }
            _ => {}
        }
    }

    entries
}

#[get("/api/priority-rules")]
async fn get_priority_rules(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    require_rules_manager(&user)?;

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let config = PriorityRulesService::ensure_rules_config(&mut conn).map_err(|e| {
        ApiError::InternalServerError(format!("Erreur chargement regles priorite: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(config))
}

#[get("/api/priority-rules/default")]
async fn get_priority_rules_default(user: web::ReqData<User>) -> ApiResult<HttpResponse> {
    require_rules_manager(&user)?;

    Ok(HttpResponse::Ok().json(json!({
        "id": PRIORITY_RULES_CONFIG_ID,
        "rules_json": PriorityRulesService::default_rules_json(),
        "is_active": true
    })))
}

#[get("/api/priority-rules/runtime")]
async fn get_priority_rules_runtime(
    pool: web::Data<DbPool>,
    _user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let config = PriorityRulesService::ensure_rules_config(&mut conn).map_err(|e| {
        ApiError::InternalServerError(format!("Erreur chargement regles priorite: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(json!({
        "id": config.id,
        "rules_json": PriorityRulesService::sanitize_rules_json(&config.rules_json),
        "is_active": config.is_active,
        "updated_at": config.updated_at
    })))
}

#[put("/api/priority-rules")]
async fn update_priority_rules(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<UpdatePriorityRulesRequest>,
) -> ApiResult<HttpResponse> {
    require_rules_manager(&user)?;

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let existing = PriorityRulesService::ensure_rules_config(&mut conn).map_err(|e| {
        ApiError::InternalServerError(format!("Erreur chargement regles priorite: {}", e))
    })?;

    let old_rules = PriorityRulesService::sanitize_rules_json(&existing.rules_json);
    let sanitized_rules = PriorityRulesService::sanitize_rules_json(&body.rules_json);
    let granular_entries = collect_priority_rule_audit_entries(&old_rules, &sanitized_rules);
    let next_is_active = body.is_active.unwrap_or(existing.is_active);
    let updated = PriorityRulesService::update_rules_config(
        &mut conn,
        user.id.as_str(),
        sanitized_rules.clone(),
        next_is_active,
    )
    .map_err(|e| {
        ApiError::InternalServerError(format!("Erreur mise a jour regles priorite: {}", e))
    })?;

    AuditService::log_action(
        &pool,
        &user,
        "UPDATE",
        "priority_rules",
        PRIORITY_RULES_CONFIG_ID,
        Some(json!(existing)),
        Some(json!(updated)),
        None,
        None,
    )
    .ok();

    for (action, entity_id, old_value, new_value) in granular_entries {
        AuditService::log_action(
            &pool,
            &user,
            action.as_str(),
            "priority_rule",
            entity_id.as_str(),
            old_value,
            new_value,
            None,
            None,
        )
        .ok();
    }

    Ok(HttpResponse::Ok().json(updated))
}

#[post("/api/priority-rules/restore-default")]
async fn restore_priority_rules_default(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    require_rules_manager(&user)?;

    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let existing = PriorityRulesService::ensure_rules_config(&mut conn).map_err(|e| {
        ApiError::InternalServerError(format!("Erreur chargement regles priorite: {}", e))
    })?;

    let old_rules = PriorityRulesService::sanitize_rules_json(&existing.rules_json);
    let defaults = PriorityRulesService::default_rules_json();
    let next_rules = PriorityRulesService::sanitize_rules_json(&defaults);
    let granular_entries = collect_priority_rule_audit_entries(&old_rules, &next_rules);

    let updated = PriorityRulesService::restore_default_config(&mut conn, user.id.as_str())
        .map_err(|e| {
            ApiError::InternalServerError(format!("Erreur restauration regles priorite: {}", e))
        })?;

    AuditService::log_action(
        &pool,
        &user,
        "RESTORE_DEFAULT",
        "priority_rules",
        PRIORITY_RULES_CONFIG_ID,
        Some(json!(existing)),
        Some(json!(updated)),
        None,
        None,
    )
    .ok();

    for (action, entity_id, old_value, new_value) in granular_entries {
        AuditService::log_action(
            &pool,
            &user,
            action.as_str(),
            "priority_rule",
            entity_id.as_str(),
            old_value,
            new_value,
            None,
            None,
        )
        .ok();
    }

    Ok(HttpResponse::Ok().json(updated))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_priority_rules)
        .service(get_priority_rules_default)
        .service(get_priority_rules_runtime)
        .service(update_priority_rules)
        .service(restore_priority_rules_default);
}
