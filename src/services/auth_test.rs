#[cfg(test)]
mod tests {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2, PasswordHash, PasswordVerifier,
    };

    #[test]
    fn test_hash_password_and_verify() {
        let password = "my_secure_password";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        // 1. Hashage
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        assert_ne!(password, password_hash);

        // 2. Vérification (Succès)
        let parsed_hash = PasswordHash::new(&password_hash).expect("Failed to parse hash");
        assert!(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok());

        // 3. Vérification (Échec)
        assert!(Argon2::default()
            .verify_password("wrong_password".as_bytes(), &parsed_hash)
            .is_err());
    }
}
