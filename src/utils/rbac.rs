use crate::models::User;
use crate::utils::ApiError;

/// Require specific roles for endpoint access
///
/// # Arguments
/// * `user` - The authenticated user
/// * `allowed_roles` - Array of allowed role strings
///
/// # Returns
/// * `Ok(())` if user has one of the allowed roles
/// * `Err(ApiError::Forbidden)` otherwise
pub fn require_role(user: &User, allowed_roles: &[&str]) -> Result<(), ApiError> {
    if !allowed_roles.contains(&user.role.as_str()) {
        return Err(ApiError::Forbidden(format!(
            "Accès réservé aux rôles: {}",
            allowed_roles.join(", ")
        )));
    }
    Ok(())
}

/// Helper to require administrator role only
pub fn require_admin(user: &User) -> Result<(), ApiError> {
    require_role(user, &["administrateur", "regulateur", "admin", "super_regul"])
}

/// Helper to require porter (brancardier) role
pub fn require_porter(user: &User) -> Result<(), ApiError> {
    require_role(user, &["brancardier"])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(role: &str) -> User {
        User {
            id: "test-id".to_string(),
            username: "testuser".to_string(),
            password_hash: "hash".to_string(),
            role: role.to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: None,
            service: None,
            is_active: true,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    #[test]
    fn test_require_admin_allows_admin() {
        let user = create_test_user("administrateur");
        assert!(require_admin(&user).is_ok());
    }

    #[test]
    fn test_require_admin_denies_porter() {
        let user = create_test_user("brancardier");
        assert!(require_admin(&user).is_err());
    }

    #[test]
    fn test_require_admin_denies_demandeur() {
        let user = create_test_user("demandeur");
        assert!(require_admin(&user).is_err());
    }

    #[test]
    fn test_require_admin_allows_regulateur() {
        let user = create_test_user("regulateur");
        assert!(require_admin(&user).is_ok());
    }

    #[test]
    fn test_require_porter_allows_porter() {
        let user = create_test_user("brancardier");
        assert!(require_porter(&user).is_ok());
    }

    #[test]
    fn test_require_porter_denies_demandeur() {
        let user = create_test_user("demandeur");
        assert!(require_porter(&user).is_err());
    }
}
