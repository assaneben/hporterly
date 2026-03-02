use crate::schema::tickets;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TicketStatus {
    Pending,     // En attente
    Assigned,    // Assignee
    InProgress,  // En cours
    Paused,      // Suspendue
    PickedUp,    // Prise en charge (interne phase brancardier)
    Arrived,     // Arrive (interne phase brancardier)
    Completed,   // Terminee
    Canceled,    // Annulee
    Desaffectee, // Desaffectee (retour en attente)
    Reassignee,  // Reassignee (transferee)
}

impl std::fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TicketStatus::Pending => write!(f, "pending"),
            TicketStatus::Assigned => write!(f, "assigned"),
            TicketStatus::InProgress => write!(f, "in_progress"),
            TicketStatus::Paused => write!(f, "suspended"),
            TicketStatus::PickedUp => write!(f, "in_progress"),
            TicketStatus::Arrived => write!(f, "arrived"),
            TicketStatus::Completed => write!(f, "completed"),
            TicketStatus::Canceled => write!(f, "canceled"),
            TicketStatus::Desaffectee => write!(f, "pending"),
            TicketStatus::Reassignee => write!(f, "assigned"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportMode {
    APied,           // A pied
    APiedAssistance, // A pied avec assistance
    FauteuilRoulant, // En fauteuil roulant
    Brancard,        // En brancard
    Lit,             // En lit
}

// ENUMS Transport v2.0
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransportType {
    Patient,
    Equipment,
    Specimen,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransportSubtype {
    // Patient (8 types)
    TpPied,
    TpFr,
    TpBranc,
    TpLit,
    TpCont,
    TpAgit,
    TpLourd,
    TpMonit,
    // Materiel (5 types)
    MatAbdo,
    MatCryo,
    MatPress,
    MatEcho,
    MatScd,
    // Prelevements (3 labos)
    LabAlpha,
    LabBeta,
    LabGamma,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = tickets)]
pub struct Ticket {
    pub id: String,
    pub patient_id: String,
    pub patient_name: String,
    pub origin: String,
    pub destination: String,
    pub priority: i32,
    pub mode: String,
    pub status: String,
    pub porter_id: Option<String>,
    pub requester_id: String,
    pub needs_o2: bool,
    pub needs_perfusion: bool,
    pub isolation: bool,
    pub patient_weight: Option<i32>,
    pub patient_agitated: bool,
    pub patient_monitoring: bool,
    pub needs_two_porters: bool,
    pub notes: Option<String>,
    pub scheduled_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,

    // Champs extension transport v2.0
    pub transport_type: String,
    pub transport_subtype: String,

    // Champs Materiel
    pub equipment_recipient_patient_id: Option<String>,
    pub equipment_recipient_patient_name: Option<String>,
    pub equipment_size: Option<String>,
    pub equipment_return_service: Option<String>,
    pub equipment_delivered: Option<bool>,
    pub equipment_label_returned: Option<bool>,

    // Champs Prelevements
    pub laboratory_name: Option<String>,
    pub specimen_types: Option<Vec<Option<String>>>,
    pub notes_for_reception: Option<String>,

    // Champs Demande d'aide
    pub help_requested: Option<bool>,
    pub help_porter_id: Option<String>,
    pub help_status: Option<String>,
    pub help_requested_at: Option<NaiveDateTime>,

    // Champs Archivage
    pub is_archived: bool,
    pub archived_at: Option<NaiveDateTime>,

    // Champs Reservation temporaire
    pub reservation_locked_by: Option<String>,
    pub reservation_locked_at: Option<NaiveDateTime>,

    // Champs Visibilite RDV programme
    pub activation_minutes_before: Option<i32>,
    pub is_visible_to_porters: bool,

    // Vigilances patient (nouvelles)
    pub patient_contentious: bool,
    pub patient_confused: bool,
    pub patient_over_120kg: bool,
    pub patient_bariatric: bool,
    pub patient_psychiatry: bool,
    pub patient_dialysis: bool,
    pub patient_icu: bool,
    pub other_precautions: Option<String>,

    // Champs patient decomposes
    pub patient_first_name: Option<String>,
    pub patient_last_name: Option<String>,
    pub patient_dob: Option<NaiveDate>,
    pub patient_sex: Option<String>,
    pub patient_ipp: Option<String>,
    pub motif: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tickets)]
pub struct NewTicket {
    pub id: String,
    pub patient_id: String,
    pub patient_name: String,
    pub origin: String,
    pub destination: String,
    pub priority: i32,
    pub mode: String,
    pub status: String,
    pub requester_id: String,
    pub needs_o2: bool,
    pub needs_perfusion: bool,
    pub isolation: bool,
    pub patient_weight: Option<i32>,
    pub patient_agitated: bool,
    pub patient_monitoring: bool,
    pub needs_two_porters: bool,
    pub notes: Option<String>,
    pub scheduled_time: Option<NaiveDateTime>,

    // Champs extension transport v2.0
    pub transport_type: String,
    pub transport_subtype: String,
    pub equipment_recipient_patient_id: Option<String>,
    pub equipment_recipient_patient_name: Option<String>,
    pub equipment_size: Option<String>,
    pub equipment_return_service: Option<String>,
    pub laboratory_name: Option<String>,
    pub specimen_types: Option<Vec<Option<String>>>,
    pub notes_for_reception: Option<String>,

    // Visibilite RDV programme
    pub activation_minutes_before: Option<i32>,
    pub is_visible_to_porters: bool,

    // Vigilances patient
    pub patient_contentious: bool,
    pub patient_confused: bool,
    pub patient_over_120kg: bool,
    pub patient_bariatric: bool,
    pub patient_psychiatry: bool,
    pub patient_dialysis: bool,
    pub patient_icu: bool,
    pub other_precautions: Option<String>,

    // Champs patient decomposes
    pub patient_first_name: Option<String>,
    pub patient_last_name: Option<String>,
    pub patient_dob: Option<NaiveDate>,
    pub patient_sex: Option<String>,
    pub patient_ipp: Option<String>,
    pub motif: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTicketRequest {
    #[serde(default)]
    pub patient_id: String,
    #[serde(default)]
    pub patient_name: String,
    #[validate(length(min = 1))]
    pub origin: String,
    #[validate(length(min = 1))]
    pub destination: String,
    #[serde(default = "default_priority")]
    #[validate(range(min = 1, max = 4))]
    pub priority: i32,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub needs_o2: bool,
    #[serde(default)]
    pub needs_perfusion: bool,
    #[serde(default)]
    pub isolation: bool,
    pub patient_weight: Option<i32>,
    #[serde(default)]
    pub patient_agitated: bool,
    #[serde(default)]
    pub patient_monitoring: bool,
    #[serde(default)]
    pub needs_two_porters: bool,
    #[validate(length(max = 5000))]
    pub notes: Option<String>,
    pub scheduled_time: Option<NaiveDateTime>,

    // Champs extension transport v2.0
    #[serde(default = "default_transport_type")]
    pub transport_type: String,
    #[serde(default = "default_transport_subtype")]
    pub transport_subtype: String,
    pub equipment_recipient_patient_id: Option<String>,
    pub equipment_recipient_patient_name: Option<String>,
    pub equipment_size: Option<String>,
    pub equipment_return_service: Option<String>,
    pub laboratory_name: Option<String>,
    pub specimen_types: Option<Vec<String>>,
    pub notes_for_reception: Option<String>,

    // Visibilite RDV programme
    pub activation_minutes_before: Option<i32>,

    // Vigilances patient (nouvelles)
    #[serde(default)]
    pub patient_contentious: bool,
    #[serde(default)]
    pub patient_confused: bool,
    #[serde(default)]
    pub patient_over_120kg: bool,
    #[serde(default)]
    pub patient_bariatric: bool,
    #[serde(default)]
    pub patient_psychiatry: bool,
    #[serde(default)]
    pub patient_dialysis: bool,
    #[serde(default)]
    pub patient_icu: bool,
    pub other_precautions: Option<String>,

    // Champs patient decomposes
    pub patient_first_name: Option<String>,
    pub patient_last_name: Option<String>,
    pub patient_dob: Option<String>, // Format "YYYY-MM-DD", parse dans le handler
    pub patient_sex: Option<String>,
    pub patient_ipp: Option<String>,
    pub motif: Option<String>,
}

fn default_transport_type() -> String {
    "PATIENT".to_string()
}

fn default_priority() -> i32 {
    3
}

fn default_transport_subtype() -> String {
    "TP-BRANC".to_string()
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketStatus {
    pub status: String,
    pub comment: Option<String>,
    pub reason_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignTicketRequest {
    /// porter_id is optional for self-assignment by porters
    /// If omitted, the backend will determine it from the authenticated user
    pub porter_id: Option<String>,
}

impl Ticket {
    pub fn normalize_status(raw_status: &str) -> Option<&'static str> {
        match raw_status.trim().to_ascii_lowercase().as_str() {
            "pending" | "desaffectee" => Some("pending"),
            "assigned" | "reassignee" => Some("assigned"),
            "in_progress" | "picked_up" => Some("in_progress"),
            "arrived" => Some("arrived"),
            "suspended" | "paused" => Some("suspended"),
            "completed" => Some("completed"),
            "canceled" | "cancelled" => Some("canceled"),
            _ => None,
        }
    }

    pub fn normalized_status_value(&self) -> Option<&'static str> {
        Self::normalize_status(self.status.as_str())
    }

    /// Verifie si une transition de statut est valide selon la spec IMM.
    pub fn can_transition_to(&self, new_status: &str) -> bool {
        let Some(current_status) = Self::normalize_status(self.status.as_str()) else {
            return false;
        };
        let Some(target_status) = Self::normalize_status(new_status) else {
            return false;
        };

        let valid_transitions: &[&str] = match current_status {
            "pending" => &["assigned", "canceled"],
            "assigned" => &["in_progress", "suspended", "pending", "canceled"],
            "in_progress" => &["arrived", "suspended", "canceled"],
            "arrived" => &["completed", "suspended", "canceled"],
            "suspended" => &["assigned", "in_progress", "pending", "canceled"],
            "completed" | "canceled" => &[],
            _ => &[],
        };

        valid_transitions.contains(&target_status)
    }

    pub fn requires_urgent_skill(&self) -> bool {
        self.priority == 1
    }

    pub fn requires_o2_skill(&self) -> bool {
        self.needs_o2
    }

    /// Verifie si le ticket est archive (lecture seule).
    pub fn is_read_only(&self) -> bool {
        self.is_archived
    }
}
