use crate::cda;
use crate::config::Config;
use crate::models::{
    AddCoPartnerRequest, AssignTicketRequest, CreateHelpRequest, CreateTicketRequest,
    DesignateSuccessorRequest, RespondToHelpRequest, UpdateTicketStatus, User,
};
use crate::services::{
    AuditService, NotificationService, TicketAssignmentActionService, TicketCoreService,
    TicketHelpService, TicketListParams, TicketUpdateService,
};
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use std::fmt::Display;

mod assignment;
mod core;
mod help;
mod updates;

use self::assignment::{
    add_co_partner, assign_ticket, cancel_ticket, get_recommendations, hard_delete_ticket,
    pause_ticket, reassign_ticket, remove_co_partner, take_ticket, unassign_ticket,
    update_ticket_status,
};
use self::core::{create_ticket, get_ticket, list_tickets};
use self::help::{
    get_available_porters_for_help, get_my_help_requests, request_help, respond_to_help,
};
use self::updates::{update_equipment_status, update_ticket_notes, update_ticket_priority};

pub(super) trait ResultLogExt<T, E> {
    fn log_if_err(self, context: &str);
}

impl<T, E> ResultLogExt<T, E> for Result<T, E>
where
    E: Display,
{
    fn log_if_err(self, context: &str) {
        if let Err(err) = self {
            log::warn!("{}: {}", context, err);
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_ticket)
        .service(list_tickets)
        .service(get_ticket)
        .service(assign_ticket)
        .service(take_ticket)
        .service(add_co_partner)
        .service(remove_co_partner)
        .service(update_ticket_status)
        .service(get_recommendations)
        .service(pause_ticket)
        .service(reassign_ticket)
        .service(unassign_ticket)
        .service(cancel_ticket)
        .service(hard_delete_ticket)
        .service(request_help)
        .service(respond_to_help)
        .service(get_my_help_requests)
        .service(get_available_porters_for_help)
        .service(update_equipment_status)
        .service(update_ticket_notes)
        .service(update_ticket_priority);
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-22: centralized route configuration for ticket lifecycle and assignment controls.
  - SBD-24: CDA dispatch hooks are integrated through assignment/updates handlers.
  - SBD-21: no route-level bypass introduced for authorization or workflow transitions.
- Not fully satisfiable in this file:
  - SBD-10 detailed action-level audit remains delegated to service layer calls.
    Alternative: enforce route-level audit middleware for defense in depth.
*/
