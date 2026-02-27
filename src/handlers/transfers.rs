use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateTransferRequest, TransferEvent, TransferRequest, TransferStatus};
use crate::state::AppState;

pub async fn list_transfers(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let items = state.transfers.read().await.clone();
    Ok(HttpResponse::Ok().json(items))
}

pub async fn create_transfer(
    state: web::Data<AppState>,
    payload: web::Json<CreateTransferRequest>,
) -> Result<HttpResponse, AppError> {
    validate_locations(&payload.origin_location, &payload.destination_location)?;

    if payload.requested_by.trim().is_empty() {
        return Err(AppError::BadRequest("requested_by is required".to_string()));
    }

    let now = Utc::now();
    let transfer = TransferRequest {
        request_id: Uuid::new_v4(),
        origin_location: payload.origin_location.trim().to_string(),
        destination_location: payload.destination_location.trim().to_string(),
        priority: payload.priority.clone(),
        status: TransferStatus::Queued,
        requested_at: now,
        updated_at: now,
        requested_by: payload.requested_by.trim().to_string(),
        assigned_to: payload.assigned_to.as_ref().map(|x| x.trim().to_string()),
        history: vec![TransferEvent {
            at: now,
            actor: payload.requested_by.trim().to_string(),
            action: "created".to_string(),
            details: "Transfer request created (generic workflow)".to_string(),
        }],
    };

    state.transfers.write().await.push(transfer.clone());
    Ok(HttpResponse::Created().json(transfer))
}

fn validate_locations(origin: &str, destination: &str) -> Result<(), AppError> {
    if origin.trim().is_empty() || destination.trim().is_empty() {
        return Err(AppError::BadRequest(
            "origin_location and destination_location are required".to_string(),
        ));
    }
    if origin.trim() == destination.trim() {
        return Err(AppError::BadRequest(
            "origin and destination must differ".to_string(),
        ));
    }
    Ok(())
}
