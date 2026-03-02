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

    let password = "password123";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Profils de démonstration
    let demo_users = vec![
        NewUser {
            id: "usr-001".to_string(),
            username: "marie.durand".to_string(),
            password_hash: password_hash.clone(),
            role: "demandeur".to_string(),
            first_name: "Marie".to_string(),
            last_name: "Durand".to_string(),
            email: Some("marie.durand@hopital.fr".to_string()),
            service: Some("Cardiologie".to_string()),
            is_active: true,
        },
        NewUser {
            id: "usr-002".to_string(),
            username: "jean.martin".to_string(),
            password_hash: password_hash.clone(),
            role: "brancardier".to_string(),
            first_name: "Jean".to_string(),
            last_name: "Martin".to_string(),
            email: Some("jean.martin@hopital.fr".to_string()),
            service: Some("Brancardage".to_string()),
            is_active: true,
        },
        NewUser {
            id: "usr-003".to_string(),
            username: "thomas.dubois".to_string(),
            password_hash: password_hash.clone(),
            role: "administrateur".to_string(),
            first_name: "Thomas".to_string(),
            last_name: "Dubois".to_string(),
            email: Some("thomas.dubois@hopital.fr".to_string()),
            service: Some("Régulation".to_string()),
            is_active: true,
        },
        NewUser {
            id: "admin-001".to_string(),
            username: "admin".to_string(),
            password_hash: password_hash.clone(),
            role: "administrateur".to_string(),
            first_name: "Admin".to_string(),
            last_name: "System".to_string(),
            email: Some("admin@hporterly.fr".to_string()),
            service: None,
            is_active: true,
        },
    ];

    for user in demo_users {
        let username = user.username.clone();
        diesel::insert_into(users::table)
            .values(&user)
            .on_conflict(users::username)
            .do_update()
            .set(users::password_hash.eq(user.password_hash.clone()))
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!("Error creating user: {}", username));

        println!("✓ User '{}' created/updated", username);
    }

    println!("\n✅ All demo users created successfully!");

    // Seed Porter (Jean Martin)
    use hporterly::models::NewPorter;
    use hporterly::schema::porters;

    let porter_jean = NewPorter {
        id: "ptr-002".to_string(),      // ID unique pour la table porters
        user_id: "usr-002".to_string(), // Doit correspondre à Jean Martin
        status: "available".to_string(),
        skills: vec![
            Some("brancard".to_string()),
            Some("fauteuil".to_string()),
            Some("lit".to_string()),
            Some("O2".to_string()),
            Some("URG".to_string()),
        ],
    };

    diesel::insert_into(porters::table)
        .values(&porter_jean)
        .on_conflict(porters::user_id) // Si déjà brancardier, on met à jour le statut/skills
        .do_update()
        .set((
            porters::status.eq("available"),
            porters::skills.eq(porter_jean.skills.clone()),
        ))
        .execute(&mut conn)
        .expect("Error seeding porter Jean Martin");

    println!("✓ Porter 'Jean Martin' seeded as 'available' with [O2, URG]");
    println!("Password for all users: password123");
}
