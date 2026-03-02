use chrono::Utc;
use diesel::prelude::*;
use hporterly::models::{AssignmentRole, NewTicketAssignment, Porter, Ticket, TicketAssignment};
use hporterly::schema::{porters, ticket_assignments, tickets};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

const ACTIVE_STATUSES: [&str; 5] = ["assigned", "in_progress", "arrived", "suspended", "paused"];

#[derive(Default, Debug)]
struct ReconcileStats {
    assignments_disabled: usize,
    assignments_created: usize,
    assignments_promoted: usize,
    tickets_relinked: usize,
    tickets_unassigned: usize,
    tickets_reassigned_to_copartner: usize,
    porter_status_busy: usize,
    porter_status_available: usize,
    multi_supervisor_porters_fixed: usize,
}

fn is_active_status(status: &str) -> bool {
    ACTIVE_STATUSES.contains(&status)
}

fn status_rank(status: &str) -> i32 {
    match status {
        "in_progress" => 5,
        "picked_up" => 4,
        "arrived" => 3,
        "assigned" => 2,
        "suspended" | "paused" => 1,
        _ => 0,
    }
}

fn deactivate_assignment(
    conn: &mut PgConnection,
    assignment_id: &str,
    now: chrono::NaiveDateTime,
    stats: &mut ReconcileStats,
) -> Result<(), diesel::result::Error> {
    let changed = diesel::update(ticket_assignments::table.find(assignment_id))
        .set((
            ticket_assignments::is_active.eq(false),
            ticket_assignments::removed_at.eq(Some(now)),
        ))
        .execute(conn)?;

    if changed > 0 {
        stats.assignments_disabled += changed;
    }

    Ok(())
}

fn ensure_supervisor_assignment(
    conn: &mut PgConnection,
    ticket_id: &str,
    porter_id: &str,
    stats: &mut ReconcileStats,
) -> Result<(), diesel::result::Error> {
    let existing = ticket_assignments::table
        .filter(ticket_assignments::ticket_id.eq(ticket_id))
        .filter(ticket_assignments::porter_id.eq(porter_id))
        .filter(ticket_assignments::is_active.eq(true))
        .first::<TicketAssignment>(conn)
        .optional()?;

    if let Some(assignment) = existing {
        if assignment.role != AssignmentRole::Supervisor.to_string() {
            let changed = diesel::update(ticket_assignments::table.find(&assignment.id))
                .set(ticket_assignments::role.eq(AssignmentRole::Supervisor.to_string()))
                .execute(conn)?;
            if changed > 0 {
                stats.assignments_promoted += changed;
            }
        }
    } else {
        let new_assignment = NewTicketAssignment {
            id: format!("asgn-{}", Uuid::new_v4()),
            ticket_id: ticket_id.to_string(),
            porter_id: porter_id.to_string(),
            role: AssignmentRole::Supervisor.to_string(),
            is_active: true,
        };

        let changed =
            diesel::insert_into(ticket_assignments::table).values(&new_assignment).execute(conn)?;

        if changed > 0 {
            stats.assignments_created += changed;
        }
    }

    Ok(())
}

fn active_assignments_for_ticket(
    conn: &mut PgConnection,
    ticket_id: &str,
) -> Result<Vec<TicketAssignment>, diesel::result::Error> {
    ticket_assignments::table
        .filter(ticket_assignments::ticket_id.eq(ticket_id))
        .filter(ticket_assignments::is_active.eq(true))
        .order(ticket_assignments::assigned_at.asc())
        .load::<TicketAssignment>(conn)
}

fn update_ticket_owner(
    conn: &mut PgConnection,
    ticket_id: &str,
    porter_id: Option<&str>,
    status: Option<&str>,
) -> Result<usize, diesel::result::Error> {
    match (porter_id, status) {
        (Some(pid), Some(st)) => diesel::update(tickets::table.find(ticket_id))
            .set((tickets::porter_id.eq(Some(pid.to_string())), tickets::status.eq(st.to_string())))
            .execute(conn),
        (Some(pid), None) => diesel::update(tickets::table.find(ticket_id))
            .set(tickets::porter_id.eq(Some(pid.to_string())))
            .execute(conn),
        (None, Some(st)) => diesel::update(tickets::table.find(ticket_id))
            .set((
                tickets::porter_id.eq::<Option<String>>(None),
                tickets::status.eq(st.to_string()),
            ))
            .execute(conn),
        (None, None) => diesel::update(tickets::table.find(ticket_id))
            .set(tickets::porter_id.eq::<Option<String>>(None))
            .execute(conn),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url).expect("Failed to connect to database");

    println!("[reconcile] Demarrage reconciliation assignments/tickets...");

    let result = conn.transaction::<ReconcileStats, diesel::result::Error, _>(|conn| {
        let now = Utc::now().naive_utc();
        let mut stats = ReconcileStats::default();

        let all_tickets = tickets::table.load::<Ticket>(conn)?;

        // 1) Desactiver toutes les assignations actives des tickets non actifs
        for ticket in all_tickets.iter().filter(|t| !is_active_status(&t.status)) {
            let active = active_assignments_for_ticket(conn, &ticket.id)?;
            for assignment in active {
                deactivate_assignment(conn, &assignment.id, now, &mut stats)?;
            }
        }

        // 2) Synchroniser ticket.porter_id <-> supervisor assignment pour tickets actifs
        for ticket in all_tickets.iter().filter(|t| is_active_status(&t.status)) {
            let active = active_assignments_for_ticket(conn, &ticket.id)?;

            let supervisor_ids: Vec<String> = active
                .iter()
                .filter(|a| a.role == AssignmentRole::Supervisor.to_string())
                .map(|a| a.porter_id.clone())
                .collect();

            let co_partner_ids: Vec<String> = active
                .iter()
                .filter(|a| a.role == AssignmentRole::CoPartner.to_string())
                .map(|a| a.porter_id.clone())
                .collect();

            let mut desired_supervisor = ticket.porter_id.clone();
            if desired_supervisor.is_none() {
                desired_supervisor =
                    supervisor_ids.first().cloned().or_else(|| co_partner_ids.first().cloned());
            }

            if let Some(supervisor_pid) = desired_supervisor {
                // Garantir ticket.owner
                if ticket.porter_id.as_deref() != Some(supervisor_pid.as_str()) {
                    let changed =
                        update_ticket_owner(conn, &ticket.id, Some(&supervisor_pid), None)?;
                    if changed > 0 {
                        stats.tickets_relinked += changed;
                    }
                }

                // Desactiver superviseurs concurrents
                for assignment in active.iter().filter(|a| {
                    a.role == AssignmentRole::Supervisor.to_string()
                        && a.porter_id != supervisor_pid
                }) {
                    deactivate_assignment(conn, &assignment.id, now, &mut stats)?;
                }

                // Garantir une assignation supervisor pour le owner
                ensure_supervisor_assignment(conn, &ticket.id, &supervisor_pid, &mut stats)?;
            } else {
                // Aucun owner legitime -> remettre en attente
                for assignment in active {
                    deactivate_assignment(conn, &assignment.id, now, &mut stats)?;
                }

                let changed = update_ticket_owner(conn, &ticket.id, None, Some("pending"))?;
                if changed > 0 {
                    stats.tickets_unassigned += changed;
                }
            }
        }

        // 3) Regle metier: un seul ticket superviseur actif par brancardier
        let mut active_owned = tickets::table
            .filter(tickets::status.eq_any(&ACTIVE_STATUSES))
            .filter(tickets::porter_id.is_not_null())
            .load::<Ticket>(conn)?;

        let mut by_porter: HashMap<String, Vec<Ticket>> = HashMap::new();
        for ticket in active_owned.drain(..) {
            if let Some(pid) = ticket.porter_id.clone() {
                by_porter.entry(pid).or_default().push(ticket);
            }
        }

        for (porter_id, mut owned_tickets) in by_porter {
            if owned_tickets.len() <= 1 {
                continue;
            }

            stats.multi_supervisor_porters_fixed += 1;

            owned_tickets.sort_by(|a, b| {
                let rank_cmp = status_rank(&b.status).cmp(&status_rank(&a.status));
                if rank_cmp != std::cmp::Ordering::Equal {
                    return rank_cmp;
                }
                b.updated_at.cmp(&a.updated_at)
            });

            let _kept = owned_tickets.remove(0);

            for ticket in owned_tickets {
                let active = active_assignments_for_ticket(conn, &ticket.id)?;

                // Retirer l'ancien superviseur de ce ticket
                for assignment in active.iter().filter(|a| {
                    a.role == AssignmentRole::Supervisor.to_string() && a.porter_id == porter_id
                }) {
                    deactivate_assignment(conn, &assignment.id, now, &mut stats)?;
                }

                // Chercher un co-partner pour reprendre
                let successor = active
                    .iter()
                    .filter(|a| {
                        a.role == AssignmentRole::CoPartner.to_string() && a.porter_id != porter_id
                    })
                    .map(|a| a.porter_id.clone())
                    .next();

                if let Some(next_pid) = successor {
                    ensure_supervisor_assignment(conn, &ticket.id, &next_pid, &mut stats)?;

                    let changed =
                        update_ticket_owner(conn, &ticket.id, Some(&next_pid), Some("assigned"))?;
                    if changed > 0 {
                        stats.tickets_reassigned_to_copartner += changed;
                    }
                } else {
                    // Personne pour reprendre -> renvoi file d'attente
                    for assignment in active {
                        if assignment.is_active {
                            deactivate_assignment(conn, &assignment.id, now, &mut stats)?;
                        }
                    }

                    let changed = update_ticket_owner(conn, &ticket.id, None, Some("pending"))?;
                    if changed > 0 {
                        stats.tickets_unassigned += changed;
                    }
                }
            }
        }

        // 4) Recaler les statuts porters busy/available
        let busy_porter_ids: HashSet<String> = tickets::table
            .filter(tickets::status.eq_any(&ACTIVE_STATUSES))
            .filter(tickets::porter_id.is_not_null())
            .select(tickets::porter_id)
            .load::<Option<String>>(conn)?
            .into_iter()
            .flatten()
            .collect();

        let all_porters = porters::table.load::<Porter>(conn)?;
        for porter in all_porters {
            let should_be_busy = busy_porter_ids.contains(&porter.id);

            if should_be_busy && porter.status == "available" {
                let changed = diesel::update(porters::table.find(&porter.id))
                    .set(porters::status.eq("busy"))
                    .execute(conn)?;
                if changed > 0 {
                    stats.porter_status_busy += changed;
                }
            }

            if !should_be_busy && porter.status == "busy" {
                let changed = diesel::update(porters::table.find(&porter.id))
                    .set(porters::status.eq("available"))
                    .execute(conn)?;
                if changed > 0 {
                    stats.porter_status_available += changed;
                }
            }
        }

        Ok(stats)
    });

    match result {
        Ok(stats) => {
            println!("[reconcile] Termine avec succes");
            println!("  - Assignations desactivees            : {}", stats.assignments_disabled);
            println!("  - Assignations supervisor creees      : {}", stats.assignments_created);
            println!("  - Assignations promues co->supervisor : {}", stats.assignments_promoted);
            println!("  - Tickets reliees au bon owner        : {}", stats.tickets_relinked);
            println!("  - Tickets renvoyes en attente         : {}", stats.tickets_unassigned);
            println!(
                "  - Tickets transferes a co-partner     : {}",
                stats.tickets_reassigned_to_copartner
            );
            println!("  - Porters passes busy                 : {}", stats.porter_status_busy);
            println!("  - Porters passes available            : {}", stats.porter_status_available);
            println!(
                "  - Porters multi-supervisor corriges   : {}",
                stats.multi_supervisor_porters_fixed
            );
        }
        Err(error) => {
            eprintln!("[reconcile] Echec: {}", error);
            std::process::exit(1);
        }
    }
}
