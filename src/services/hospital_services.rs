use crate::models::Service;
use crate::repositories::HospitalServiceRepository;
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

pub struct HospitalServiceQueryService;

impl HospitalServiceQueryService {
    pub fn list_all(pool: &DbPool) -> ApiResult<Vec<Service>> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;
        HospitalServiceRepository::list_all(&mut conn)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to fetch services: {}", e)))
    }
}
