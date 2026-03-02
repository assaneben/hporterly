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
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let mut conn = pool.get().expect("Failed to get connection");

    let password = "password";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_admin = NewUser {
        id: "admin-real".to_string(),
        username: "admin".to_string(),
        password_hash,
        role: "administrateur".to_string(), // Correspond au format string attendu
        first_name: "Admin".to_string(),
        last_name: "Real".to_string(),
        email: Some("admin@hporterly.fr".to_string()),
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

    println!("Admin user 'admin' created/updated with password 'password'");
}
