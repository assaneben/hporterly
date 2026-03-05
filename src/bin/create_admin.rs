use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use hporterly::models::NewUser;
use hporterly::schema::users;

#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let admin_password =
        std::env::var("ADMIN_INITIAL_PASSWORD").expect("ADMIN_INITIAL_PASSWORD must be set");
    let admin_email =
        std::env::var("ADMIN_INITIAL_EMAIL").unwrap_or_else(|_| "admin@example.invalid".to_string());
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");
    let mut conn = pool.get().expect("Failed to get connection");

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(admin_password.as_bytes(), &salt)
        .expect("Failed to hash ADMIN_INITIAL_PASSWORD")
        .to_string();

    let new_admin = NewUser {
        id: "admin-real".to_string(),
        username: "admin".to_string(),
        password_hash,
        role: "administrateur".to_string(), // Correspond au format string attendu
        first_name: "Admin".to_string(),
        last_name: "Real".to_string(),
        email: Some(admin_email),
        service: None,
        is_active: true,
    };

    diesel::insert_into(users::table)
        .values(&new_admin)
        .on_conflict(users::username)
        .do_update()
        .set(users::password_hash.eq(new_admin.password_hash.clone()))
        .execute(&mut conn)
        .expect("Error creating admin user");

    println!(
        "Admin user 'admin' created/updated. Password sourced from ADMIN_INITIAL_PASSWORD."
    );
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-07: bootstrap password is now sourced from environment instead of hardcoded.
  - SBD-13: explicit startup failures on missing bootstrap secrets.
  - SBD-22: bootstrap identity creation remains deterministic and auditable.
- Not fully satisfiable in this file:
  - SBD-11 is not relevant to this offline bootstrap utility.
    Alternative: restrict execution of this binary to controlled admin environments only.
*/
