// Utilitaire pour migrer les anciens IDs de tickets
// Usage: cargo run --bin migrate_ticket_ids

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_types::{Integer, Nullable, Text, Timestamp};

#[derive(QueryableByName)]
struct OldTicketRow {
    #[diesel(sql_type = Text)]
    id: String,
    #[diesel(sql_type = Timestamp)]
    created_at: chrono::NaiveDateTime,
}

#[derive(QueryableByName)]
struct MaxSeqRow {
    #[diesel(sql_type = Nullable<Integer>)]
    max_seq: Option<i32>,
}

#[derive(QueryableByName)]
struct VerifyRow {
    #[diesel(sql_type = Text)]
    id: String,
    #[diesel(sql_type = Text)]
    patient_name: String,
}

fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    let mut conn = pool.get().expect("Failed to get connection");

    println!("🔄 Migration des IDs de tickets vers le format BR-HPly-XXX-2026...\n");

    // Récupérer tous les tickets avec ancien format (hash)
    let old_tickets = diesel::sql_query(
        "SELECT id, created_at FROM tickets WHERE id NOT LIKE 'BR-HPly-___-2026' ORDER BY created_at"
    )
    .load::<OldTicketRow>(&mut conn)
    .expect("Failed to load old tickets");

    if old_tickets.is_empty() {
        println!("✅ Aucun ticket à migrer. Tous les tickets utilisent déjà le bon format.");
        return;
    }

    println!("📊 {} tickets à migrer\n", old_tickets.len());

    // Trouver le prochain numéro séquentiel disponible
    let max_seq = diesel::sql_query(
        "SELECT MAX(CAST(SUBSTRING(id FROM 9 FOR 3) AS INTEGER)) as max_seq FROM tickets WHERE id LIKE 'BR-HPly-___-2026'"
    )
    .get_result::<MaxSeqRow>(&mut conn)
    .ok()
    .and_then(|row| row.max_seq);

    let mut next_number = max_seq.unwrap_or(0) + 1;

    // Migrer chaque ticket
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        for row in old_tickets.iter() {
            let old_id = &row.id;
            let new_id = format!("BR-HPly-{:03}-2026", next_number);

            diesel::sql_query(format!(
                "UPDATE tickets SET id = '{}' WHERE id = '{}'",
                new_id, old_id
            ))
            .execute(conn)?;

            println!("  ✓ {} → {}", old_id, new_id);
            next_number += 1;
        }
        Ok(())
    })
    .expect("Migration failed");

    println!("\n✅ Migration terminée avec succès!");
    println!("   {} tickets mis à jour\n", old_tickets.len());

    // Vérification
    println!("📋 Vérification des IDs (10 premiers tickets):");
    let verification =
        diesel::sql_query("SELECT id, patient_name FROM tickets ORDER BY created_at LIMIT 10")
            .load::<VerifyRow>(&mut conn)
            .expect("Failed to verify");

    for row in verification {
        println!("  {} - {}", row.id, row.patient_name);
    }
}
