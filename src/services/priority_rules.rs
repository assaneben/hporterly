use crate::models::{CreateTicketRequest, NewPriorityRulesConfig, PriorityRulesConfig};
use crate::schema::priority_rules_config;
use crate::utils::{ApiError, ApiResult};
use chrono::Utc;
use diesel::prelude::*;

pub const PRIORITY_RULES_CONFIG_ID: &str = "global";

#[derive(Debug, Clone)]
pub struct PriorityEvaluationResult {
    pub priority: i32,
    pub reason: String,
    pub rule_id: Option<String>,
}

pub struct PriorityRulesService;

impl PriorityRulesService {
    pub fn default_rules_json() -> serde_json::Value {
        serde_json::json!({
            "version": 1,
            "fallback_reason": "Patient stable - aucune precaution",
            "levels": [
                {
                    "key": "N1",
                    "priority": 1,
                    "label": "N1 Urgence",
                    "description": "Une seule condition suffit - evaluation absolue",
                    "enabled": true
                },
                {
                    "key": "N2",
                    "priority": 2,
                    "label": "N2 Prioritaire",
                    "description": "N1 non declenche ET au moins une condition vraie",
                    "enabled": true
                },
                {
                    "key": "N3",
                    "priority": 3,
                    "label": "N3 Standard",
                    "description": "N1/N2 non declenches ET conditions standard",
                    "enabled": true
                },
                {
                    "key": "N4",
                    "priority": 4,
                    "label": "N4 Programme",
                    "description": "Par defaut si aucune regle N1/N2/N3",
                    "enabled": true
                }
            ],
            "rules": [
                {
                    "id": "n1-vital",
                    "level": 1,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "vital_emergency", "operator": "is_checked" }
                    ],
                    "reason": "Situation vitale engagee"
                },
                {
                    "id": "n1-intubated",
                    "level": 1,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "intubated_ventilated", "operator": "is_checked" }
                    ],
                    "reason": "Patient intube/ventile"
                },
                {
                    "id": "n1-monitoring-urgences",
                    "level": 1,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "patient_monitoring", "operator": "is_checked" },
                        { "field": "destination", "operator": "contains", "value": "urgences" }
                    ],
                    "reason": "Monitoring actif - destination Urgences"
                },

                {
                    "id": "n2-monitoring",
                    "level": 2,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "patient_monitoring", "operator": "is_checked" }
                    ],
                    "reason": "Monitoring actif - IDE requis"
                },
                {
                    "id": "n2-destination-bloc",
                    "level": 2,
                    "enabled": true,
                    "combinator": "OR",
                    "conditions": [
                        { "field": "destination", "operator": "contains", "value": "bloc operatoire" },
                        { "field": "destination", "operator": "contains", "value": "usc" },
                        { "field": "destination", "operator": "contains", "value": "sspi" },
                        { "field": "destination", "operator": "contains", "value": "salle de reveil" }
                    ],
                    "reason": "Destination : {destination}"
                },
                {
                    "id": "n2-perfusion-critical-destination",
                    "level": 2,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "needs_perfusion", "operator": "is_checked" },
                        {
                            "field": "destination",
                            "operator": "contains_any",
                            "value": ["bloc", "usc"]
                        }
                    ],
                    "reason": "Perfusion active - destination critique"
                },
                {
                    "id": "n2-dialysis",
                    "level": 2,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "patient_dialysis", "operator": "is_checked" }
                    ],
                    "reason": "Patient en dialyse"
                },
                {
                    "id": "n2-agitated-no-escort",
                    "level": 2,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "patient_agitated", "operator": "is_checked" },
                        { "field": "ide_accompanying_confirmed", "operator": "is_unchecked" }
                    ],
                    "reason": "Patient agite sans accompagnant"
                },

                {
                    "id": "n3-o2-only",
                    "level": 3,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "needs_o2", "operator": "is_checked" }
                    ],
                    "reason": "Oxygene requis"
                },
                {
                    "id": "n3-perfusion-standard",
                    "level": 3,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "needs_perfusion", "operator": "is_checked" }
                    ],
                    "reason": "Perfusion active"
                },
                {
                    "id": "n3-isolation",
                    "level": 3,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "isolation", "operator": "is_checked" }
                    ],
                    "reason": "Precaution isolement requise"
                },
                {
                    "id": "n3-restrained",
                    "level": 3,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "patient_contentious", "operator": "is_checked" }
                    ],
                    "reason": "Patient contentionne"
                },
                {
                    "id": "n3-bariatric",
                    "level": 3,
                    "enabled": true,
                    "combinator": "OR",
                    "conditions": [
                        { "field": "patient_bariatric", "operator": "is_checked" },
                        { "field": "patient_over_120kg", "operator": "is_checked" }
                    ],
                    "reason": "Materiel adapte requis"
                },
                {
                    "id": "n3-psychiatry",
                    "level": 3,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "patient_psychiatry", "operator": "is_checked" }
                    ],
                    "reason": "Patient en psychiatrie"
                },
                {
                    "id": "n3-destination-standard-imaging",
                    "level": 3,
                    "enabled": true,
                    "combinator": "OR",
                    "conditions": [
                        { "field": "destination", "operator": "contains", "value": "radiologie" },
                        { "field": "destination", "operator": "contains", "value": "echo" },
                        { "field": "destination", "operator": "contains", "value": "scanner" },
                        { "field": "destination", "operator": "contains", "value": "kine" }
                    ],
                    "reason": "Destination : {destination}"
                },
                {
                    "id": "n3-interservice-stable",
                    "level": 3,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "inter_service_transfer", "operator": "is_checked" },
                        { "field": "patient_stable", "operator": "is_checked" }
                    ],
                    "reason": "Transfert inter-service"
                },

                {
                    "id": "n4-stable-no-precaution",
                    "level": 4,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "patient_stable", "operator": "is_checked" },
                        { "field": "any_precaution", "operator": "is_unchecked" }
                    ],
                    "reason": "Patient stable - aucune precaution"
                },
                {
                    "id": "n4-consultation",
                    "level": 4,
                    "enabled": true,
                    "combinator": "OR",
                    "conditions": [
                        { "field": "destination", "operator": "contains", "value": "consultation" },
                        { "field": "destination", "operator": "contains", "value": "hdj" },
                        { "field": "destination", "operator": "contains", "value": "sortie" }
                    ],
                    "reason": "Destination : {destination}"
                },
                {
                    "id": "n4-rdv",
                    "level": 4,
                    "enabled": true,
                    "combinator": "AND",
                    "conditions": [
                        { "field": "scheduled_time_set", "operator": "is_checked" }
                    ],
                    "reason": "RDV programme"
                }
            ]
        })
    }

    pub fn sanitize_rules_json(input: &serde_json::Value) -> serde_json::Value {
        let mut candidate = input.clone();
        if !candidate.is_object() {
            return Self::default_rules_json();
        }

        if candidate.get("levels").and_then(|v| v.as_array()).is_none() {
            candidate["levels"] = Self::default_rules_json()["levels"].clone();
        }

        if candidate.get("rules").and_then(|v| v.as_array()).is_none() {
            candidate["rules"] = Self::default_rules_json()["rules"].clone();
        }

        if candidate
            .get("fallback_reason")
            .and_then(|v| v.as_str())
            .map(|v| v.trim().is_empty())
            .unwrap_or(true)
        {
            candidate["fallback_reason"] =
                serde_json::Value::String("Patient stable - aucune precaution".to_string());
        }

        if candidate.get("version").and_then(|v| v.as_i64()).is_none() {
            candidate["version"] = serde_json::Value::Number(serde_json::Number::from(1));
        }

        candidate
    }

    pub fn ensure_rules_config(
        conn: &mut diesel::PgConnection,
    ) -> Result<PriorityRulesConfig, diesel::result::Error> {
        if let Some(existing) = priority_rules_config::table
            .find(PRIORITY_RULES_CONFIG_ID)
            .first::<PriorityRulesConfig>(conn)
            .optional()?
        {
            return Ok(existing);
        }

        let insert = NewPriorityRulesConfig {
            id: PRIORITY_RULES_CONFIG_ID.to_string(),
            rules_json: Self::default_rules_json(),
            is_active: true,
            updated_by: None,
        };

        let _ = diesel::insert_into(priority_rules_config::table)
            .values(&insert)
            .execute(conn);

        priority_rules_config::table
            .find(PRIORITY_RULES_CONFIG_ID)
            .first::<PriorityRulesConfig>(conn)
    }

    pub fn evaluate_for_request(
        conn: &mut diesel::PgConnection,
        request: &CreateTicketRequest,
    ) -> ApiResult<PriorityEvaluationResult> {
        let config = Self::ensure_rules_config(conn).map_err(|e| {
            ApiError::InternalServerError(format!("Erreur chargement regles priorite: {}", e))
        })?;

        if !config.is_active {
            let fallback = request.priority.clamp(1, 4);
            return Ok(PriorityEvaluationResult {
                priority: fallback,
                reason: "Regles automatiques desactivees".to_string(),
                rule_id: None,
            });
        }

        let rules_json = Self::sanitize_rules_json(&config.rules_json);
        let context = Self::build_request_context(request);
        Ok(Self::evaluate_with_context(&rules_json, &context))
    }

    pub fn update_rules_config(
        conn: &mut diesel::PgConnection,
        user_id: &str,
        rules_json: serde_json::Value,
        is_active: bool,
    ) -> Result<PriorityRulesConfig, diesel::result::Error> {
        let now = Utc::now().naive_utc();
        diesel::update(priority_rules_config::table.find(PRIORITY_RULES_CONFIG_ID))
            .set((
                priority_rules_config::rules_json.eq(rules_json),
                priority_rules_config::is_active.eq(is_active),
                priority_rules_config::updated_by.eq(Some(user_id.to_string())),
                priority_rules_config::updated_at.eq(now),
            ))
            .get_result::<PriorityRulesConfig>(conn)
    }

    pub fn restore_default_config(
        conn: &mut diesel::PgConnection,
        user_id: &str,
    ) -> Result<PriorityRulesConfig, diesel::result::Error> {
        let now = Utc::now().naive_utc();
        diesel::update(priority_rules_config::table.find(PRIORITY_RULES_CONFIG_ID))
            .set((
                priority_rules_config::rules_json.eq(Self::default_rules_json()),
                priority_rules_config::is_active.eq(true),
                priority_rules_config::updated_by.eq(Some(user_id.to_string())),
                priority_rules_config::updated_at.eq(now),
            ))
            .get_result::<PriorityRulesConfig>(conn)
    }

    pub fn evaluate_with_context(
        config: &serde_json::Value,
        context: &serde_json::Value,
    ) -> PriorityEvaluationResult {
        for level in [1, 2, 3, 4] {
            if !Self::is_level_enabled(config, level) {
                continue;
            }

            let rules = config
                .get("rules")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            for rule in rules {
                let rule_level = rule.get("level").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                if rule_level != level {
                    continue;
                }

                let enabled = rule
                    .get("enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                if !enabled {
                    continue;
                }

                if Self::rule_matches(&rule, context) {
                    let reason_template = rule
                        .get("reason")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Priorite calculee automatiquement");

                    let reason = Self::render_reason(reason_template, context);
                    let rule_id = rule
                        .get("id")
                        .and_then(|v| v.as_str())
                        .map(|v| v.to_string());

                    return PriorityEvaluationResult {
                        priority: level,
                        reason,
                        rule_id,
                    };
                }
            }
        }

        let fallback_reason = config
            .get("fallback_reason")
            .and_then(|v| v.as_str())
            .unwrap_or("Patient stable - aucune precaution")
            .to_string();

        PriorityEvaluationResult {
            priority: 4,
            reason: Self::render_reason(&fallback_reason, context),
            rule_id: None,
        }
    }

    fn is_level_enabled(config: &serde_json::Value, level: i32) -> bool {
        config
            .get("levels")
            .and_then(|v| v.as_array())
            .and_then(|levels| {
                levels.iter().find(|entry| {
                    entry
                        .get("priority")
                        .and_then(|v| v.as_i64())
                        .map(|value| value as i32 == level)
                        .unwrap_or(false)
                })
            })
            .and_then(|entry| entry.get("enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    fn rule_matches(rule: &serde_json::Value, context: &serde_json::Value) -> bool {
        let conditions = rule
            .get("conditions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if conditions.is_empty() {
            return true;
        }

        let combinator = rule
            .get("combinator")
            .and_then(|v| v.as_str())
            .unwrap_or("AND")
            .to_uppercase();

        let mut results = conditions
            .iter()
            .map(|condition| Self::condition_matches(condition, context));

        if combinator == "OR" {
            results.any(|matched| matched)
        } else {
            results.all(|matched| matched)
        }
    }

    fn condition_matches(condition: &serde_json::Value, context: &serde_json::Value) -> bool {
        let field = condition
            .get("field")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if field.is_empty() {
            return false;
        }

        let operator = condition
            .get("operator")
            .and_then(|v| v.as_str())
            .unwrap_or("is_checked")
            .to_lowercase();

        let field_value = context
            .get(field)
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        match operator.as_str() {
            "is_checked" => Self::value_to_bool(&field_value),
            "is_unchecked" => !Self::value_to_bool(&field_value),
            "equals" => Self::compare_equals(&field_value, condition.get("value")),
            "not_equals" => !Self::compare_equals(&field_value, condition.get("value")),
            "contains" => Self::compare_contains(&field_value, condition.get("value")),
            "contains_any" => Self::compare_contains_any(&field_value, condition.get("value")),
            "in" => Self::compare_in(&field_value, condition.get("value")),
            "gte" => Self::compare_gte(&field_value, condition.get("value")),
            _ => false,
        }
    }

    fn compare_equals(
        field_value: &serde_json::Value,
        expected: Option<&serde_json::Value>,
    ) -> bool {
        let Some(expected) = expected else {
            return false;
        };

        if field_value.is_boolean() || expected.is_boolean() {
            return Self::value_to_bool(field_value) == Self::value_to_bool(expected);
        }

        Self::normalized_text(field_value) == Self::normalized_text(expected)
    }

    fn compare_contains(
        field_value: &serde_json::Value,
        expected: Option<&serde_json::Value>,
    ) -> bool {
        let haystack = Self::normalized_text(field_value);
        let needle = expected.map(Self::normalized_text).unwrap_or_default();
        !needle.is_empty() && haystack.contains(needle.as_str())
    }

    fn compare_contains_any(
        field_value: &serde_json::Value,
        expected: Option<&serde_json::Value>,
    ) -> bool {
        let haystack = Self::normalized_text(field_value);
        let Some(expected) = expected else {
            return false;
        };

        if let Some(values) = expected.as_array() {
            return values
                .iter()
                .map(Self::normalized_text)
                .filter(|value| !value.is_empty())
                .any(|needle| haystack.contains(needle.as_str()));
        }

        let needle = Self::normalized_text(expected);
        !needle.is_empty() && haystack.contains(needle.as_str())
    }

    fn compare_in(field_value: &serde_json::Value, expected: Option<&serde_json::Value>) -> bool {
        let Some(values) = expected.and_then(|v| v.as_array()) else {
            return false;
        };

        let current = Self::normalized_text(field_value);
        values
            .iter()
            .map(Self::normalized_text)
            .filter(|item| !item.is_empty())
            .any(|item| item == current)
    }

    fn compare_gte(field_value: &serde_json::Value, expected: Option<&serde_json::Value>) -> bool {
        let current = Self::value_to_f64(field_value).unwrap_or(f64::MIN);
        let target = expected.and_then(Self::value_to_f64).unwrap_or(f64::MAX);
        current >= target
    }

    fn render_reason(template: &str, context: &serde_json::Value) -> String {
        let destination = context
            .get("destination")
            .and_then(|v| v.as_str())
            .unwrap_or("destination");

        template
            .replace("{destination}", destination)
            .trim()
            .to_string()
    }

    fn value_to_bool(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::Bool(v) => *v,
            serde_json::Value::Number(v) => v.as_i64().unwrap_or_default() != 0,
            serde_json::Value::String(v) => {
                let normalized = v.trim().to_lowercase();
                matches!(normalized.as_str(), "1" | "true" | "oui" | "yes" | "on")
            }
            _ => false,
        }
    }

    fn value_to_f64(value: &serde_json::Value) -> Option<f64> {
        match value {
            serde_json::Value::Number(v) => v.as_f64(),
            serde_json::Value::String(v) => v.trim().parse::<f64>().ok(),
            serde_json::Value::Bool(v) => Some(if *v { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    fn normalized_text(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(v) => normalize_text(v),
            serde_json::Value::Number(v) => normalize_text(&v.to_string()),
            serde_json::Value::Bool(v) => normalize_text(if *v { "true" } else { "false" }),
            _ => String::new(),
        }
    }

    fn build_request_context(request: &CreateTicketRequest) -> serde_json::Value {
        let meta = parse_meta_form_block(request.notes.as_deref().unwrap_or(""));

        let vital_emergency = parse_meta_bool(&meta, "V_SITUATION_VITALE_ENGAGEE");
        let intubated_ventilated = parse_meta_bool(&meta, "V_INTUBE_VENTILE");
        let patient_stable = parse_meta_bool(&meta, "V_ETAT_PATIENT_STABLE");
        let ide_accompanying_confirmed = parse_meta_bool(&meta, "IDE_ACCOMPAGNANT");

        let any_precaution = request.needs_o2
            || request.needs_perfusion
            || request.isolation
            || request.patient_monitoring
            || request.patient_agitated
            || request.patient_contentious
            || request.patient_bariatric
            || request.patient_dialysis
            || request.patient_psychiatry
            || request.patient_icu
            || request.patient_over_120kg
            || vital_emergency
            || intubated_ventilated;

        let inter_service_transfer = !request.origin.trim().is_empty()
            && !request.destination.trim().is_empty()
            && normalize_text(&request.origin) != normalize_text(&request.destination);

        serde_json::json!({
            "transport_type": request.transport_type,
            "origin": request.origin,
            "destination": request.destination,
            "needs_o2": request.needs_o2,
            "needs_perfusion": request.needs_perfusion,
            "isolation": request.isolation,
            "patient_agitated": request.patient_agitated,
            "patient_monitoring": request.patient_monitoring,
            "patient_bariatric": request.patient_bariatric,
            "patient_dialysis": request.patient_dialysis,
            "patient_psychiatry": request.patient_psychiatry,
            "patient_contentious": request.patient_contentious,
            "patient_icu": request.patient_icu,
            "patient_over_120kg": request.patient_over_120kg,
            "ide_accompanying_confirmed": ide_accompanying_confirmed,
            "vital_emergency": vital_emergency,
            "intubated_ventilated": intubated_ventilated,
            "patient_stable": patient_stable,
            "scheduled_time_set": request.scheduled_time.is_some(),
            "any_precaution": any_precaution,
            "inter_service_transfer": inter_service_transfer,
        })
    }
}

fn normalize_text(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â'], "a")
        .replace(['î', 'ï'], "i")
        .replace(['ô', 'ö'], "o")
        .replace(['ù', 'û', 'ü'], "u")
        .replace('ç', "c")
}

fn parse_meta_form_block(notes: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let start_tag = "[META-FORM]";
    let end_tag = "[/META-FORM]";

    let Some(start_idx) = notes.find(start_tag) else {
        return map;
    };

    let sliced = &notes[start_idx + start_tag.len()..];
    let Some(end_rel_idx) = sliced.find(end_tag) else {
        return map;
    };

    let block = &sliced[..end_rel_idx];
    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            let normalized_key = key.trim().to_string();
            let normalized_value = value.trim().to_string();
            if !normalized_key.is_empty() {
                map.insert(normalized_key, normalized_value);
            }
        }
    }

    map
}

fn parse_meta_bool(map: &std::collections::HashMap<String, String>, key: &str) -> bool {
    map.get(key)
        .map(|value| {
            let normalized = value.trim().to_lowercase();
            matches!(normalized.as_str(), "1" | "true" | "oui" | "yes" | "on")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::PriorityRulesService;

    #[test]
    fn evaluate_default_rules_triggers_n1_on_vital_emergency() {
        let config = PriorityRulesService::default_rules_json();
        let context = serde_json::json!({
            "vital_emergency": true,
            "destination": "Urgences"
        });

        let result = PriorityRulesService::evaluate_with_context(&config, &context);
        assert_eq!(result.priority, 1);
        assert!(result.reason.to_lowercase().contains("vitale"));
    }

    #[test]
    fn evaluate_default_rules_falls_back_to_n4() {
        let config = PriorityRulesService::default_rules_json();
        let context = serde_json::json!({
            "destination": "Consultation",
            "patient_stable": true,
            "any_precaution": false
        });

        let result = PriorityRulesService::evaluate_with_context(&config, &context);
        assert_eq!(result.priority, 4);
    }
}
