use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use diesel::Connection;
use diesel::PgConnection;
use serde_json::json;
use serde_json::Value;

use crate::models::{CreateTicketRequest, NewTicket, Ticket, User};
use crate::repositories::{PorterRepository, TicketListFilters, TicketRepository};
use crate::services::{
    AuditService, DispatchService, NotificationService, PriorityRulesService,
    TicketAssignmentService, ValidatorService,
};
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;

type RequesterLookup = HashMap<String, (String, String)>;
type TicketAssignmentsLookup = HashMap<String, (Option<String>, Vec<String>)>;

#[derive(Debug, Clone)]
pub struct TicketListParams {
    pub include_archived: bool,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub porter_id: Option<String>,
    pub transport_type: Option<String>,
    pub cursor_created_at: Option<NaiveDateTime>,
    pub cursor_id: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl TicketListParams {
    fn parse_cursor_created_at(query: &HashMap<String, String>) -> Option<NaiveDateTime> {
        let raw_value = query.get("cursor_created_at")?;
        DateTime::parse_from_rfc3339(raw_value)
            .map(|value| value.naive_utc())
            .or_else(|_| NaiveDateTime::parse_from_str(raw_value, "%Y-%m-%dT%H:%M:%S%.f"))
            .or_else(|_| NaiveDateTime::parse_from_str(raw_value, "%Y-%m-%d %H:%M:%S%.f"))
            .ok()
    }

    pub fn from_query(query: &HashMap<String, String>) -> Self {
        let include_archived = query
            .get("include_archived")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        let status = query.get("status").cloned();
        let priority = query
            .get("priority")
            .and_then(|value| value.parse::<i32>().ok());
        let porter_id = query.get("porter_id").cloned();
        let transport_type = query.get("transport_type").cloned();
        let cursor_created_at = Self::parse_cursor_created_at(query);
        let cursor_id = cursor_created_at
            .as_ref()
            .and_then(|_| query.get("cursor_id").cloned());
        let limit = query
            .get("limit")
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(200)
            .clamp(1, 500);
        let offset = query
            .get("offset")
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(0)
            .max(0);

        Self {
            include_archived,
            status,
            priority,
            porter_id,
            transport_type,
            cursor_created_at,
            cursor_id,
            limit,
            offset,
        }
    }
}

pub struct TicketCoreService;

pub struct TicketCreateResult {
    pub ticket: Ticket,
}

impl TicketCoreService {
    pub fn create_from_pool(
        pool: &DbPool,
        user: &User,
        request: CreateTicketRequest,
    ) -> ApiResult<Ticket> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;

        let result = Self::create_ticket(&mut conn, user, request)?;
        let ticket = result.ticket;

        if let Err(err) = AuditService::log_action(
            pool,
            user,
            "CREATE",
            "ticket",
            &ticket.id,
            None,
            Some(json!(&ticket)),
            None,
            None,
        ) {
            log::warn!("Audit log CREATE ticket {} failed: {}", ticket.id, err);
        }

        log::info!("Ticket {} created by user {}", ticket.id, user.id);

        let notif = NotificationService::ticket_created(&ticket.id, ticket.priority);
        if let Ok(reg_admin_ids) = NotificationService::find_user_ids_by_roles(
            &mut conn,
            &["administrateur", "regulateur"],
        ) {
            for uid in &reg_admin_ids {
                if let Err(err) = NotificationService::persist_if_enabled(
                    &mut conn,
                    uid,
                    &notif,
                    Some(&ticket.id),
                ) {
                    log::warn!(
                        "Failed to persist ticket_created notification for {}: {}",
                        uid,
                        err
                    );
                }
            }
        }

        if ticket.priority == 1 {
            let urgent_notif = NotificationService::urgent_ticket(&ticket.id);
            if let Ok(porter_recipients) =
                NotificationService::list_active_porter_recipients(&mut conn)
            {
                for recipient in porter_recipients {
                    if let Err(err) = NotificationService::persist_if_enabled(
                        &mut conn,
                        &recipient.user_id,
                        &urgent_notif,
                        Some(&ticket.id),
                    ) {
                        log::warn!(
                            "Failed to persist urgent ticket alert for {}: {}",
                            recipient.user_id,
                            err
                        );
                    }
                }
            }
        }

        Ok(ticket)
    }

    pub fn list_payloads_from_pool(
        pool: &DbPool,
        user: &User,
        params: TicketListParams,
    ) -> ApiResult<Vec<Value>> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;
        TicketReadService::list_payloads(&mut conn, user, params)
    }

    pub fn get_payload_from_pool(pool: &DbPool, user: &User, ticket_id: &str) -> ApiResult<Value> {
        let mut conn = pool.get().map_err(|e| {
            ApiError::InternalServerError(format!("Database connection error: {}", e))
        })?;
        TicketReadService::get_payload_by_id(&mut conn, user, ticket_id)
    }

    pub fn create_ticket(
        conn: &mut PgConnection,
        user: &User,
        mut request: CreateTicketRequest,
    ) -> ApiResult<TicketCreateResult> {
        ValidatorService::validate_ticket(&request)?;
        ValidatorService::auto_calculate_two_porters(&mut request);

        let auto_priority = PriorityRulesService::evaluate_for_request(conn, &request)?;
        request.priority = auto_priority.priority.clamp(1, 4);

        let ticket = conn
            .transaction::<Ticket, diesel::result::Error, _>(|conn| {
                TicketRepository::lock_ticket_id_generation(conn)?;
                let ticket_id = Self::generate_next_ticket_id(conn)?;
                let new_ticket = Self::build_new_ticket(user, &request, ticket_id);
                TicketRepository::insert(conn, &new_ticket)
            })
            .map_err(|e| {
                ApiError::InternalServerError(format!("Failed to create ticket: {}", e))
            })?;

        Self::try_auto_assign(conn, &ticket);

        Ok(TicketCreateResult { ticket })
    }

    fn generate_next_ticket_id(conn: &mut PgConnection) -> Result<String, diesel::result::Error> {
        let current_year = Utc::now().year();
        let year_pattern = format!("BR-HPly-%-{}", current_year);
        let count = TicketRepository::count_by_id_pattern(conn, &year_pattern)?;
        let next_number = count + 1;
        Ok(format!("BR-HPly-{:03}-{}", next_number, current_year))
    }

    fn build_new_ticket(
        user: &User,
        request: &CreateTicketRequest,
        ticket_id: String,
    ) -> NewTicket {
        let is_programmed = request.priority == 4 && request.scheduled_time.is_some();
        let is_visible_to_porters = !is_programmed;
        let activation_minutes_before = if is_programmed {
            request.activation_minutes_before.or(Some(30))
        } else {
            None
        };

        let patient_dob = request
            .patient_dob
            .as_ref()
            .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok());

        NewTicket {
            id: ticket_id,
            patient_id: request.patient_id.clone(),
            patient_name: request.patient_name.clone(),
            origin: request.origin.clone(),
            destination: request.destination.clone(),
            priority: request.priority,
            mode: request.mode.clone().unwrap_or_default(),
            status: "pending".to_string(),
            requester_id: user.id.clone(),
            needs_o2: request.needs_o2,
            needs_perfusion: request.needs_perfusion,
            isolation: request.isolation,
            patient_weight: request.patient_weight,
            patient_agitated: request.patient_agitated,
            patient_monitoring: request.patient_monitoring,
            needs_two_porters: request.needs_two_porters,
            notes: request.notes.clone(),
            scheduled_time: request.scheduled_time,
            transport_type: request.transport_type.clone(),
            transport_subtype: request.transport_subtype.clone(),
            equipment_recipient_patient_id: request.equipment_recipient_patient_id.clone(),
            equipment_recipient_patient_name: request.equipment_recipient_patient_name.clone(),
            equipment_size: request.equipment_size.clone(),
            equipment_return_service: request.equipment_return_service.clone(),
            laboratory_name: request.laboratory_name.clone(),
            specimen_types: request
                .specimen_types
                .clone()
                .map(|types| types.into_iter().map(Some).collect()),
            notes_for_reception: request.notes_for_reception.clone(),
            activation_minutes_before,
            is_visible_to_porters,
            patient_contentious: request.patient_contentious,
            patient_confused: request.patient_confused,
            patient_over_120kg: request.patient_over_120kg,
            patient_bariatric: request.patient_bariatric,
            patient_psychiatry: request.patient_psychiatry,
            patient_dialysis: request.patient_dialysis,
            patient_icu: request.patient_icu,
            other_precautions: request.other_precautions.clone(),
            patient_first_name: request.patient_first_name.clone(),
            patient_last_name: request.patient_last_name.clone(),
            patient_dob,
            patient_sex: request.patient_sex.clone(),
            patient_ipp: request.patient_ipp.clone(),
            motif: request.motif.clone(),
        }
    }

    fn try_auto_assign(conn: &mut PgConnection, ticket: &Ticket) {
        if ticket.priority <= 2 {
            // IMM workflow: P1/P2 stay pending for regulated assignment / first-responder flow.
            return;
        }

        let available_porters = match PorterRepository::list_available(conn) {
            Ok(rows) => rows,
            Err(err) => {
                log::warn!("Failed to load available porters for auto-assign: {}", err);
                return;
            }
        };

        if let Some((best_porter, score)) =
            DispatchService::get_best_porter(ticket, available_porters)
        {
            if score <= 0 {
                return;
            }

            log::info!(
                "IA auto-assign: ticket {} -> porter {} (score: {})",
                ticket.id,
                best_porter.id,
                score
            );

            if let Err(err) = TicketRepository::set_porter_and_status(
                conn,
                &ticket.id,
                &best_porter.id,
                "assigned",
            ) {
                log::warn!("Failed to auto-assign ticket {}: {}", ticket.id, err);
                return;
            }

            if let Err(err) = PorterRepository::set_status(conn, &best_porter.id, "busy") {
                log::warn!(
                    "Failed to set porter {} busy during auto-assign: {}",
                    best_porter.id,
                    err
                );
            }

            if let Err(err) = TicketAssignmentService::ensure_supervisor_assignment(
                conn,
                &ticket.id,
                &best_porter.id,
            ) {
                log::warn!(
                    "Failed to create supervisor assignment for ticket {}: {}",
                    ticket.id,
                    err
                );
            }
        }
    }
}

pub struct TicketReadService;

impl TicketReadService {
    pub fn list_payloads(
        conn: &mut PgConnection,
        user: &User,
        params: TicketListParams,
    ) -> ApiResult<Vec<serde_json::Value>> {
        if let Err(err) = TicketRepository::activate_programmed_tickets_visibility(conn) {
            log::warn!(
                "Failed to auto-activate programmed tickets visibility: {}",
                err
            );
        }

        let filters = Self::build_filters(user, params);
        let tickets_list = TicketRepository::list(conn, &filters).map_err(|e| {
            ApiError::InternalServerError(format!("Failed to fetch tickets: {}", e))
        })?;

        Self::build_payload_for_role(conn, user.role.as_str(), &tickets_list)
    }

    pub fn get_payload_by_id(
        conn: &mut PgConnection,
        user: &User,
        ticket_id: &str,
    ) -> ApiResult<serde_json::Value> {
        let ticket = TicketRepository::find_by_id(conn, ticket_id)
            .map_err(|_| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

        if user.role == "demandeur" {
            let requester_match = ticket.requester_id == user.id
                || ticket
                    .requester_id
                    .eq_ignore_ascii_case(user.username.as_str());
            if !requester_match {
                return Err(ApiError::Forbidden(
                    "Acces refuse a cette demande".to_string(),
                ));
            }
        }

        let payload =
            Self::build_payload_for_role(conn, user.role.as_str(), std::slice::from_ref(&ticket))?;
        Ok(payload.into_iter().next().unwrap_or_else(|| json!({})))
    }

    fn build_filters(user: &User, params: TicketListParams) -> TicketListFilters {
        let requester_scope = if user.role == "demandeur" {
            Some((user.id.clone(), user.username.clone()))
        } else {
            None
        };

        TicketListFilters {
            include_archived: params.include_archived,
            status: params.status,
            priority: params.priority,
            porter_id: params.porter_id,
            transport_type: params.transport_type,
            visible_to_porters_only: user.role == "brancardier",
            requester_scope,
            cursor_created_at: params.cursor_created_at,
            cursor_id: params.cursor_id,
            limit: params.limit,
            offset: params.offset,
        }
    }

    fn can_view_ticket_creator(role: &str) -> bool {
        role == "administrateur" || role == "brancardier"
    }

    pub(crate) fn build_payload_for_role(
        conn: &mut PgConnection,
        role: &str,
        tickets_list: &[Ticket],
    ) -> ApiResult<Vec<serde_json::Value>> {
        let requester_lookup = if Self::can_view_ticket_creator(role) {
            Self::build_requester_lookup(conn, tickets_list).map_err(|e| {
                ApiError::InternalServerError(format!("Failed to resolve requester info: {}", e))
            })?
        } else {
            RequesterLookup::new()
        };
        let assignments_lookup = Self::build_ticket_assignments_lookup(conn, tickets_list)
            .map_err(|e| {
                ApiError::InternalServerError(format!(
                    "Failed to resolve ticket assignments: {}",
                    e
                ))
            })?;

        Ok(tickets_list
            .iter()
            .map(|ticket| {
                Self::enrich_ticket_payload_for_role(
                    ticket,
                    role,
                    &requester_lookup,
                    &assignments_lookup,
                )
            })
            .collect())
    }

    fn build_ticket_assignments_lookup(
        conn: &mut PgConnection,
        tickets_list: &[Ticket],
    ) -> Result<TicketAssignmentsLookup, diesel::result::Error> {
        let ticket_ids: Vec<String> = tickets_list
            .iter()
            .map(|ticket| ticket.id.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let rows = TicketRepository::find_active_assignments(conn, &ticket_ids)?;
        let mut lookup: TicketAssignmentsLookup = TicketAssignmentsLookup::new();
        for (ticket_id, porter_id, role) in rows {
            let entry = lookup
                .entry(ticket_id)
                .or_insert_with(|| (None, Vec::new()));

            match role.as_str() {
                "supervisor" => {
                    entry.0 = Some(porter_id);
                }
                "co_partner" => {
                    if !entry.1.iter().any(|existing| existing == &porter_id) {
                        entry.1.push(porter_id);
                    }
                }
                _ => {}
            }
        }

        Ok(lookup)
    }

    fn build_requester_lookup(
        conn: &mut PgConnection,
        tickets_list: &[Ticket],
    ) -> Result<RequesterLookup, diesel::result::Error> {
        let requester_ids: Vec<String> = tickets_list
            .iter()
            .map(|ticket| ticket.requester_id.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let rows = TicketRepository::find_requesters(conn, &requester_ids)?;
        let mut lookup = RequesterLookup::new();
        for (id, username, first_name, last_name) in rows {
            let full_name = format!("{} {}", first_name.trim(), last_name.trim())
                .trim()
                .to_string();
            lookup.insert(id, (username.trim().to_string(), full_name));
        }

        Ok(lookup)
    }

    fn resolve_requester_identity(
        ticket: &Ticket,
        lookup: &RequesterLookup,
    ) -> (String, Option<String>, Option<String>) {
        if let Some((username, full_name)) = lookup.get(ticket.requester_id.as_str()) {
            let username = username.trim();
            let full_name = full_name.trim();

            let display = if !full_name.is_empty() && !username.is_empty() {
                format!("{} ({})", full_name, username)
            } else if !full_name.is_empty() {
                full_name.to_string()
            } else if !username.is_empty() {
                username.to_string()
            } else {
                ticket.requester_id.clone()
            };

            let normalized_username = if username.is_empty() {
                None
            } else {
                Some(username.to_string())
            };
            let normalized_full_name = if full_name.is_empty() {
                None
            } else {
                Some(full_name.to_string())
            };

            return (display, normalized_username, normalized_full_name);
        }

        let fallback = ticket.requester_id.trim().to_string();
        let display = if fallback.is_empty() {
            "Inconnu".to_string()
        } else {
            fallback.clone()
        };

        let normalized = if fallback.is_empty() {
            None
        } else {
            Some(fallback)
        };
        (display, normalized, None)
    }

    fn enrich_ticket_payload_for_role(
        ticket: &Ticket,
        role: &str,
        lookup: &RequesterLookup,
        assignments_lookup: &TicketAssignmentsLookup,
    ) -> serde_json::Value {
        let mut payload = serde_json::to_value(ticket).unwrap_or_else(|_| json!({}));
        let (supervisor_porter_id, co_partner_ids) = assignments_lookup
            .get(ticket.id.as_str())
            .cloned()
            .unwrap_or_else(|| (None, Vec::new()));

        if let Some(object) = payload.as_object_mut() {
            object.insert(
                "supervisor_porter_id".to_string(),
                supervisor_porter_id
                    .map(serde_json::Value::String)
                    .unwrap_or(serde_json::Value::Null),
            );
            let co_partner_ids_json = serde_json::Value::Array(
                co_partner_ids
                    .iter()
                    .map(|id| serde_json::Value::String(id.clone()))
                    .collect(),
            );
            object.insert("co_partner_ids".to_string(), co_partner_ids_json.clone());
            object.insert("co_partners".to_string(), co_partner_ids_json);
        }

        if !Self::can_view_ticket_creator(role) {
            return payload;
        }

        let (display, username, full_name) = Self::resolve_requester_identity(ticket, lookup);
        if let Some(object) = payload.as_object_mut() {
            object.insert(
                "requester_display".to_string(),
                serde_json::Value::String(display),
            );
            if let Some(value) = username {
                object.insert(
                    "requester_username".to_string(),
                    serde_json::Value::String(value),
                );
            }
            if let Some(value) = full_name {
                object.insert(
                    "requester_full_name".to_string(),
                    serde_json::Value::String(value),
                );
            }
        }

        payload
    }
}
