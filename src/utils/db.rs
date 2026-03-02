// Database helper utilities

use crate::{utils::ApiError, utils::ApiResult, DbPool};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

/// Get a database connection from the pool
///
/// This helper centralizes error handling for database connections
/// and provides a consistent error message format.
///
/// # Example
/// ```rust,ignore
/// let mut conn = get_connection(&pool)?;
/// ```
pub fn get_connection(
    pool: &DbPool,
) -> ApiResult<PooledConnection<ConnectionManager<PgConnection>>> {
    pool.get().map_err(|e| {
        log::error!("Failed to get database connection: {}", e);
        ApiError::InternalServerError("Database connection error. Please try again.".to_string())
    })
}

#[cfg(test)]
mod tests {
    // Tests would require a test database setup
    // For now, this is a placeholder
}
