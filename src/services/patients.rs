use crate::models::{PatientSearchResult, SearchPatientsQuery};
use crate::repositories::PatientRepository;
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

pub struct PatientQueryService;

impl PatientQueryService {
    pub fn search(
        pool: &DbPool,
        query: &SearchPatientsQuery,
    ) -> ApiResult<Vec<PatientSearchResult>> {
        let normalized_search = query.search.trim();
        if normalized_search.is_empty() {
            return Ok(Vec::new());
        }

        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;
        let search_term = format!("%{}%", normalized_search.to_lowercase());
        let patients =
            PatientRepository::search(&mut conn, &search_term, query.limit).map_err(|e| {
                ApiError::InternalServerError(format!("Failed to search patients: {}", e))
            })?;
        Ok(patients.into_iter().map(PatientSearchResult::from).collect())
    }
}
