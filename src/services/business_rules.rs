use crate::utils::{ApiError, ApiResult};

pub const STATUS_PENDING: &str = "pending";
pub const STATUS_ASSIGNED: &str = "assigned";
pub const STATUS_IN_PROGRESS: &str = "in_progress";
pub const STATUS_ARRIVED: &str = "arrived";
pub const STATUS_SUSPENDED: &str = "suspended";
pub const STATUS_COMPLETED: &str = "completed";
pub const STATUS_CANCELED: &str = "canceled";

pub const CANCEL_REASON_OTHER: &str = "other";
pub const CANCEL_REASON_CODES: [&str; 8] = [
    "patient_not_ready",
    "exam_canceled",
    "patient_discharged",
    "duplicate_request",
    "identity_error",
    "care_refusal",
    "intercurrent_emergency",
    CANCEL_REASON_OTHER,
];

pub const SUSPEND_REASON_OTHER: &str = "other";
pub const SUSPEND_REASON_CODES: [&str; 4] = [
    "patient_not_ready",
    "elevator_outage",
    "urgent_interruption",
    SUSPEND_REASON_OTHER,
];

pub fn normalize_status(raw: &str) -> Option<&'static str> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "pending" => Some(STATUS_PENDING),
        "assigned" => Some(STATUS_ASSIGNED),
        "in_progress" => Some(STATUS_IN_PROGRESS),
        "picked_up" => Some(STATUS_IN_PROGRESS),
        "arrived" => Some(STATUS_ARRIVED),
        "suspended" | "paused" => Some(STATUS_SUSPENDED),
        "completed" => Some(STATUS_COMPLETED),
        "canceled" | "cancelled" => Some(STATUS_CANCELED),
        _ => None,
    }
}

pub fn is_active_status(raw: &str) -> bool {
    matches!(
        normalize_status(raw),
        Some(STATUS_ASSIGNED | STATUS_IN_PROGRESS | STATUS_ARRIVED | STATUS_SUSPENDED)
    )
}

pub fn can_transition(current_status: &str, target_status: &str) -> bool {
    let Some(current) = normalize_status(current_status) else {
        return false;
    };
    let Some(target) = normalize_status(target_status) else {
        return false;
    };

    let allowed: &[&str] = match current {
        STATUS_PENDING => &[STATUS_ASSIGNED, STATUS_CANCELED],
        STATUS_ASSIGNED => &[
            STATUS_IN_PROGRESS,
            STATUS_SUSPENDED,
            STATUS_PENDING,
            STATUS_CANCELED,
        ],
        STATUS_IN_PROGRESS => &[STATUS_ARRIVED, STATUS_SUSPENDED, STATUS_CANCELED],
        STATUS_ARRIVED => &[STATUS_COMPLETED, STATUS_SUSPENDED, STATUS_CANCELED],
        STATUS_SUSPENDED => &[
            STATUS_ASSIGNED,
            STATUS_IN_PROGRESS,
            STATUS_PENDING,
            STATUS_CANCELED,
        ],
        STATUS_COMPLETED | STATUS_CANCELED => &[],
        _ => &[],
    };

    allowed.contains(&target)
}

pub fn normalize_reason_code(input: Option<&str>) -> Option<String> {
    input
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

pub fn validate_cancellation_reason(
    code: Option<&str>,
    comment: Option<&str>,
) -> ApiResult<String> {
    let reason = normalize_reason_code(code)
        .ok_or_else(|| ApiError::BadRequest("Motif d'annulation obligatoire".to_string()))?;

    if !CANCEL_REASON_CODES.contains(&reason.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Motif d'annulation invalide. Valeurs attendues: {}",
            CANCEL_REASON_CODES.join(", ")
        )));
    }

    if reason == CANCEL_REASON_OTHER {
        let has_comment = comment
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        if !has_comment {
            return Err(ApiError::BadRequest(
                "Un commentaire est obligatoire pour le motif d'annulation 'other'".to_string(),
            ));
        }
    }

    Ok(reason)
}

pub fn validate_suspension_reason(code: Option<&str>, comment: Option<&str>) -> ApiResult<String> {
    let reason = normalize_reason_code(code)
        .ok_or_else(|| ApiError::BadRequest("Motif de suspension obligatoire".to_string()))?;

    if !SUSPEND_REASON_CODES.contains(&reason.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Motif de suspension invalide. Valeurs attendues: {}",
            SUSPEND_REASON_CODES.join(", ")
        )));
    }

    if reason == SUSPEND_REASON_OTHER {
        let has_comment = comment
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        if !has_comment {
            return Err(ApiError::BadRequest(
                "Un commentaire est obligatoire pour le motif de suspension 'other'".to_string(),
            ));
        }
    }

    Ok(reason)
}

pub fn is_regulation_role(role: &str) -> bool {
    matches!(
        role.trim().to_ascii_lowercase().as_str(),
        "administrateur" | "admin" | "regulateur" | "super_regul"
    )
}

pub fn can_self_assign_by_priority(priority: i32) -> bool {
    matches!(priority, 1 | 3 | 4)
}
