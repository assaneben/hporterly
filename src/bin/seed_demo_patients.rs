use chrono::{Datelike, NaiveDate, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use hporterly::models::NewPatient;
use hporterly::schema::patients;

fn parse_dob(raw: &str) -> NaiveDate {
    NaiveDate::parse_from_str(raw, "%Y-%m-%d")
        .unwrap_or_else(|_| panic!("Invalid date format for patient seed: {}", raw))
}

fn compute_age(dob: NaiveDate) -> i32 {
    let today = Utc::now().date_naive();
    let mut age = today.year() - dob.year();
    if today.ordinal() < dob.ordinal() {
        age -= 1;
    }
    age as i32
}

fn normalize_gender(sex: &str) -> String {
    match sex {
        "M" => "M".to_string(),
        "F" => "F".to_string(),
        _ => "O".to_string(),
    }
}

fn make_patient(
    id: &str,
    first_name: &str,
    last_name: &str,
    dob: &str,
    sex: &str,
    service: &str,
    room: &str,
    building: &str,
    floor: &str,
) -> NewPatient {
    let date_of_birth = parse_dob(dob);
    NewPatient {
        id: id.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        age: Some(compute_age(date_of_birth)),
        gender: Some(normalize_gender(sex)),
        service: Some(service.to_string()),
        room: Some(room.to_string()),
        building: Some(building.to_string()),
        floor: Some(floor.to_string()),
        date_of_birth: Some(date_of_birth),
        sex: Some(sex.to_string()),
    }
}

fn demo_patients() -> Vec<NewPatient> {
    let base_people = vec![
        ("IPP-001001", "Jean", "Dupont", "1957-03-14", "M"),
        ("IPP-001002", "Marie", "Dupuis", "1978-11-22", "F"),
        ("IPP-001003", "Pierre", "Martin", "1952-07-03", "M"),
        ("IPP-001004", "Sophie", "Bernard", "1969-09-30", "F"),
        ("IPP-001005", "Jacques", "Leroy", "1961-01-18", "M"),
        ("IPP-001006", "Isabelle", "Moreau", "1976-05-10", "F"),
        ("IPP-001007", "Robert", "Petit", "1944-12-07", "M"),
        ("IPP-001008", "Anne", "Dubois", "1987-04-15", "F"),
        ("IPP-001009", "Lucas", "Renard", "1993-02-26", "M"),
        ("IPP-001010", "Emma", "Roussel", "1984-08-19", "F"),
        ("IPP-001011", "Thomas", "Mercier", "1971-06-11", "M"),
        ("IPP-001012", "Camille", "Garcia", "1990-10-01", "F"),
        ("IPP-001013", "Nora", "Faure", "1982-03-09", "F"),
        ("IPP-001014", "Hugo", "Riviere", "1966-07-27", "M"),
        ("IPP-001015", "Leo", "Mallet", "2001-09-06", "M"),
        ("IPP-001016", "Sarah", "Legrand", "1989-01-21", "F"),
        ("IPP-001017", "Nina", "Besson", "1995-12-02", "X"),
        ("IPP-001018", "Yanis", "Lombard", "1973-04-04", "M"),
        ("IPP-001019", "Claire", "Blanc", "1959-11-13", "F"),
        ("IPP-001020", "Ahmed", "Benali", "1980-02-17", "M"),
        ("IPP-001021", "Elise", "Navarro", "1992-05-29", "F"),
        ("IPP-001022", "Mathieu", "Arnaud", "1964-08-25", "M"),
        ("IPP-001023", "Aline", "Chevalier", "1975-06-16", "F"),
        ("IPP-001024", "Cedric", "Colin", "1986-03-20", "M"),
        // Serie alphabetique A -> Z pour couvrir l'auto-completion des prenoms.
        ("IPP-001025", "Arnaud", "Garnier", "1981-01-09", "M"),
        ("IPP-001026", "Bruno", "Renaud", "1977-04-24", "M"),
        ("IPP-001027", "Cyril", "Noel", "1989-09-18", "M"),
        ("IPP-001028", "Damien", "Archer", "1991-12-03", "M"),
        ("IPP-001029", "Etienne", "Lefevre", "1974-06-14", "M"),
        ("IPP-001030", "Fabien", "Pelletier", "1986-02-27", "M"),
        ("IPP-001031", "Guillaume", "Masson", "1993-07-12", "M"),
        ("IPP-001032", "Helene", "Roche", "1968-11-05", "F"),
        ("IPP-001033", "Ines", "Bourdin", "1997-03-30", "F"),
        ("IPP-001034", "Julie", "Texier", "1988-08-21", "F"),
        ("IPP-001035", "Karim", "Dumas", "1979-10-08", "M"),
        ("IPP-001036", "Lucie", "Morin", "1990-05-16", "F"),
        ("IPP-001037", "Mehdi", "Vidal", "1984-09-29", "M"),
        ("IPP-001038", "Nadia", "Barbier", "1972-01-31", "F"),
        ("IPP-001039", "Olivier", "Paris", "1965-07-07", "M"),
        ("IPP-001040", "Pauline", "Rolland", "1992-04-11", "F"),
        ("IPP-001041", "Quentin", "Perrin", "1987-12-19", "M"),
        ("IPP-001042", "Romain", "Henry", "1980-03-02", "M"),
        ("IPP-001043", "Salome", "Picard", "1995-06-26", "F"),
        ("IPP-001044", "Theo", "Loiseau", "2000-10-15", "M"),
        ("IPP-001045", "Ulysse", "Faivre", "1983-02-06", "M"),
        ("IPP-001046", "Valerie", "Gilbert", "1971-09-23", "F"),
        ("IPP-001047", "William", "Aubert", "1994-11-28", "M"),
        ("IPP-001048", "Xavier", "Robert", "1976-05-04", "M"),
        ("IPP-001049", "Yasmine", "Caron", "1985-08-17", "F"),
        ("IPP-001050", "Zoe", "Briand", "1998-01-13", "F"),
    ];

    let services = [
        "Unite de soins A",
        "Unite de soins B",
        "Unite de soins C",
        "Urgences",
        "Imagerie",
        "Scanner",
        "Bloc operatoire",
        "USC",
        "Recuperation",
        "Consultations",
    ];
    let buildings = ["BAT-A", "BAT-B", "BAT-C", "BAT-D"];
    let floors = ["Niveau 0", "Niveau 1", "Niveau 2", "Niveau 3"];

    base_people
        .into_iter()
        .enumerate()
        .map(|(idx, (id, first_name, last_name, dob, sex))| {
            let building = buildings[idx % buildings.len()];
            let floor = floors[(idx * 3 + 1) % floors.len()];
            let service = services[(idx * 7 + 2) % services.len()];
            let floor_idx = floors.iter().position(|f| *f == floor).unwrap_or(0) as i32;
            let room_number = 100 * (floor_idx + 1) + (((idx as i32 * 13) % 38) + 1);
            let room = format!("R-{}", room_number);

            make_patient(
                id,
                first_name,
                last_name,
                dob,
                sex,
                service,
                room.as_str(),
                building,
                floor,
            )
        })
        .collect()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");
    let mut conn = pool.get().expect("Failed to get connection");

    let demo_patients = demo_patients();
    let mut upserted = 0usize;

    for patient in demo_patients {
        let patient_id = patient.id.clone();
        let patient_name = format!("{} {}", patient.first_name, patient.last_name);

        diesel::insert_into(patients::table)
            .values(&patient)
            .on_conflict(patients::id)
            .do_update()
            .set((
                patients::first_name.eq(patient.first_name.clone()),
                patients::last_name.eq(patient.last_name.clone()),
                patients::age.eq(patient.age),
                patients::gender.eq(patient.gender.clone()),
                patients::service.eq(patient.service.clone()),
                patients::room.eq(patient.room.clone()),
                patients::building.eq(patient.building.clone()),
                patients::floor.eq(patient.floor.clone()),
                patients::date_of_birth.eq(patient.date_of_birth),
                patients::sex.eq(patient.sex.clone()),
            ))
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!("Error creating/updating patient: {}", patient_id));

        println!("✓ Patient '{}' (IPP: {}) upserted", patient_name, patient_id);
        upserted += 1;
    }

    println!();
    println!("✅ {} patients fictifs prêts pour l'auto-completion", upserted);
}
