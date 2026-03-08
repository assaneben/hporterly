use std::collections::HashMap;

use chrono::{DateTime, Duration, NaiveDateTime};
use serde::Serialize;
use serde_json::Value;

use crate::models::User;
use crate::repositories::TicketRepository;
use crate::services::TicketReadService;
use crate::utils::{require_admin, ApiError, ApiResult};
use crate::DbPool;

const REPORTS_MAX_WINDOW_DAYS: i64 = 800;
const REPORTS_MAX_ROWS: i64 = 50_000;

#[derive(Debug, Clone)]
pub struct OperationalReportsQuery {
    pub dataset_start: NaiveDateTime,
    pub dataset_end: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct OperationalReportsDatasetResponse {
    pub tickets: Vec<Value>,
    pub available_years: Vec<i32>,
    pub truncated: bool,
    pub dataset_start: NaiveDateTime,
    pub dataset_end: NaiveDateTime,
}

pub struct ReportsService;

impl OperationalReportsQuery {
    pub fn from_query(query: &HashMap<String, String>) -> ApiResult<Self> {
        let dataset_start = parse_datetime(query.get("dataset_start").map(String::as_str))
            .ok_or_else(|| {
                ApiError::BadRequest("Le parametre dataset_start est requis".to_string())
            })?;
        let dataset_end =
            parse_datetime(query.get("dataset_end").map(String::as_str)).ok_or_else(|| {
                ApiError::BadRequest("Le parametre dataset_end est requis".to_string())
            })?;

        if dataset_start > dataset_end {
            return Err(ApiError::BadRequest(
                "dataset_start doit etre inferieur ou egal a dataset_end".to_string(),
            ));
        }

        let span = dataset_end - dataset_start;
        if span > Duration::days(REPORTS_MAX_WINDOW_DAYS) {
            return Err(ApiError::BadRequest(format!(
                "La fenetre statistique ne peut pas depasser {} jours",
                REPORTS_MAX_WINDOW_DAYS
            )));
        }

        Ok(Self {
            dataset_start,
            dataset_end,
        })
    }
}

impl ReportsService {
    pub fn get_operational_dataset(
        pool: &DbPool,
        user: &User,
        query: OperationalReportsQuery,
    ) -> ApiResult<OperationalReportsDatasetResponse> {
        require_admin(user)?;

        let mut conn = pool.get().map_err(|error| {
            ApiError::InternalServerError(format!("Database connection error: {}", error))
        })?;

        let available_years = TicketRepository::list_report_years(&mut conn).map_err(|error| {
            ApiError::InternalServerError(format!("Failed to load report years: {}", error))
        })?;

        let mut tickets = TicketRepository::list_for_reports_range(
            &mut conn,
            query.dataset_start,
            query.dataset_end,
            REPORTS_MAX_ROWS + 1,
        )
        .map_err(|error| {
            ApiError::InternalServerError(format!("Failed to load report dataset: {}", error))
        })?;

        let truncated = tickets.len() as i64 > REPORTS_MAX_ROWS;
        if truncated {
            tickets.truncate(REPORTS_MAX_ROWS as usize);
        }

        let payloads =
            TicketReadService::build_payload_for_role(&mut conn, user.role.as_str(), &tickets)?;

        Ok(OperationalReportsDatasetResponse {
            tickets: payloads,
            available_years,
            truncated,
            dataset_start: query.dataset_start,
            dataset_end: query.dataset_end,
        })
    }
}

fn parse_datetime(input: Option<&str>) -> Option<NaiveDateTime> {
    let raw = input?.trim();
    if raw.is_empty() {
        return None;
    }

    DateTime::parse_from_rfc3339(raw)
        .map(|value| value.naive_utc())
        .or_else(|_| NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S%.f"))
        .or_else(|_| NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S%.f"))
        .ok()
}
