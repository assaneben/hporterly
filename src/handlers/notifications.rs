use crate::models::{MarkReadRequest, UnreadCountResponse, UpdatePreferencesRequest, User};
use crate::services::{
    MessageAdminRecipient, MessageDemandeurRecipient, MessagePorterRecipient, NotificationService,
};
use crate::utils::{ApiError, ApiResult};
use crate::DbPool;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub limit: Option<i64>,
    pub notification_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendUserMessageRequest {
    pub message: String,
    pub channel: Option<String>,
    pub target_type: Option<String>,
    pub target_user_id: Option<String>,
    pub target_porter_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageRecipientsResponse {
    pub admins: Vec<MessageAdminRecipient>,
    pub porters: Vec<MessagePorterRecipient>,
    pub demandeurs: Vec<MessageDemandeurRecipient>,
}

/// GET /api/notifications - List user notifications (most recent first)
#[get("/api/notifications")]
async fn list_notifications(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    query: web::Query<NotificationQuery>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let limit = query.limit.unwrap_or(50).max(1).min(200);
    let notif_type = query
        .notification_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let notifications = NotificationService::get_user_notifications_filtered(
        &mut conn, &user.id, limit, notif_type,
    )
    .map_err(|e| ApiError::InternalServerError(format!("Error fetching notifications: {}", e)))?;

    Ok(HttpResponse::Ok().json(notifications))
}

/// GET /api/notifications/unread-count - Get unread notification count
#[get("/api/notifications/unread-count")]
async fn get_unread_count(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let count = NotificationService::count_unread(&mut conn, &user.id).map_err(|e| {
        ApiError::InternalServerError(format!("Error counting notifications: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(UnreadCountResponse { count }))
}

/// POST /api/notifications/mark-read - Mark specific notifications as read
#[post("/api/notifications/mark-read")]
async fn mark_notifications_read(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<MarkReadRequest>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let updated = NotificationService::mark_as_read(&mut conn, &user.id, &body.notification_ids)
        .map_err(|e| {
            ApiError::InternalServerError(format!("Error marking notifications: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "updated": updated })))
}

/// POST /api/notifications/mark-all-read - Mark all notifications as read
#[post("/api/notifications/mark-all-read")]
async fn mark_all_notifications_read(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let updated = NotificationService::mark_all_as_read(&mut conn, &user.id).map_err(|e| {
        ApiError::InternalServerError(format!("Error marking all notifications: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "updated": updated })))
}

/// GET /api/notifications/preferences - Get user notification preferences
#[get("/api/notifications/preferences")]
async fn get_preferences(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let prefs = NotificationService::get_or_create_preferences(&mut conn, &user.id)
        .map_err(|e| ApiError::InternalServerError(format!("Error fetching preferences: {}", e)))?;

    Ok(HttpResponse::Ok().json(prefs))
}

/// PATCH /api/notifications/preferences - Update user notification preferences
#[patch("/api/notifications/preferences")]
async fn update_preferences(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<UpdatePreferencesRequest>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let updated = NotificationService::update_preferences(
        &mut conn,
        &user.id,
        body.preferences.clone(),
        body.sound_enabled,
    )
    .map_err(|e| ApiError::InternalServerError(format!("Error updating preferences: {}", e)))?;

    Ok(HttpResponse::Ok().json(updated))
}

/// GET /api/notifications/message-recipients - List messaging recipients
#[get("/api/notifications/message-recipients")]
async fn list_message_recipients(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let mut admins = NotificationService::list_active_admin_recipients(&mut conn)
        .map_err(|e| ApiError::InternalServerError(format!("Error fetching admins: {}", e)))?;
    let mut porters = NotificationService::list_active_porter_recipients(&mut conn)
        .map_err(|e| ApiError::InternalServerError(format!("Error fetching porters: {}", e)))?;
    let mut demandeurs = NotificationService::list_active_demandeur_recipients(&mut conn)
        .map_err(|e| ApiError::InternalServerError(format!("Error fetching demandeurs: {}", e)))?;

    admins.retain(|entry| entry.id != user.id);
    porters.retain(|entry| entry.user_id != user.id);
    demandeurs.retain(|entry| entry.id != user.id);

    Ok(HttpResponse::Ok().json(MessageRecipientsResponse {
        admins,
        porters,
        demandeurs,
    }))
}

/// POST /api/notifications/send-message - Send a user message to admins or a specific porter
#[post("/api/notifications/send-message")]
async fn send_message_to_admins(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    body: web::Json<SendUserMessageRequest>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let message = body.message.trim();
    let message_len = message.chars().count();
    if message_len < 2 {
        return Err(ApiError::ValidationError(
            "Le message doit contenir au moins 2 caracteres".to_string(),
        ));
    }
    if message_len > 1000 {
        return Err(ApiError::ValidationError(
            "Le message ne peut pas depasser 1000 caracteres".to_string(),
        ));
    }

    let channel_raw = body.channel.as_deref().unwrap_or("general").trim();
    let channel = if channel_raw.is_empty() {
        "general".to_string()
    } else {
        channel_raw.to_string()
    };

    let full_name = format!("{} {}", user.first_name.trim(), user.last_name.trim())
        .trim()
        .to_string();
    let sender_display = if full_name.is_empty() {
        user.username.clone()
    } else {
        format!("{} ({})", full_name, user.username)
    };

    let target_type = body
        .target_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("admins")
        .to_lowercase();

    let mut recipient_ids: Vec<String> = Vec::new();
    let mut sent_to_admins = 0_i64;
    let mut sent_to_porters = 0_i64;
    let target_label: Option<String>;

    match target_type.as_str() {
        "admins" | "administrateurs" | "administration" => {
            recipient_ids = NotificationService::find_user_ids_by_roles(
                &mut conn,
                &["administrateur", "regulateur", "admin"],
            )
            .map_err(|e| ApiError::InternalServerError(format!("Error fetching admins: {}", e)))?;
            target_label = Some("Administration".to_string());
        }
        "admin" | "administrateur" | "regulateur" => {
            let admin_user_id = body
                .target_user_id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    ApiError::ValidationError(
                        "target_user_id est requis pour un envoi admin cible".to_string(),
                    )
                })?;

            let mut admin_id = NotificationService::resolve_active_user_id_by_role(
                &mut conn,
                admin_user_id,
                "administrateur",
            )
            .map_err(|e| {
                ApiError::InternalServerError(format!("Error resolving admin recipient: {}", e))
            })?;

            if admin_id.is_none() {
                admin_id = NotificationService::resolve_active_user_id_by_role(
                    &mut conn,
                    admin_user_id,
                    "regulateur",
                )
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Error resolving admin recipient: {}", e))
                })?;
            }

            if admin_id.is_none() {
                admin_id = NotificationService::resolve_active_user_id_by_role(
                    &mut conn,
                    admin_user_id,
                    "admin",
                )
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Error resolving admin recipient: {}", e))
                })?;
            }

            let admin_id = admin_id.ok_or_else(|| {
                ApiError::NotFound("Administrateur destinataire introuvable".to_string())
            })?;

            target_label = NotificationService::resolve_active_user_display(&mut conn, &admin_id)
                .map_err(|e| {
                ApiError::InternalServerError(format!("Error resolving admin label: {}", e))
            })?;

            recipient_ids.push(admin_id);
        }
        "porter" | "brancardier" => {
            let porter_id = body
                .target_porter_id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    ApiError::ValidationError(
                        "target_porter_id est requis pour un envoi brancardier".to_string(),
                    )
                })?;

            let porter_user_id =
                NotificationService::resolve_active_porter_user_id(&mut conn, porter_id)
                    .map_err(|e| {
                        ApiError::InternalServerError(format!(
                            "Error resolving porter recipient: {}",
                            e
                        ))
                    })?
                    .ok_or_else(|| {
                        ApiError::NotFound("Brancardier destinataire introuvable".to_string())
                    })?;

            target_label = NotificationService::resolve_active_porter_display(&mut conn, porter_id)
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Error resolving porter label: {}", e))
                })?;

            recipient_ids.push(porter_user_id);
        }
        "all_porters" | "tous_brancardiers" => {
            recipient_ids =
                NotificationService::find_user_ids_by_roles(&mut conn, &["brancardier"]).map_err(
                    |e| ApiError::InternalServerError(format!("Error fetching porters: {}", e)),
                )?;
            target_label = Some("Tous les brancardiers".to_string());
        }
        "demandeur" => {
            let demandeur_user_id = body
                .target_user_id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    ApiError::ValidationError(
                        "target_user_id est requis pour un envoi demandeur".to_string(),
                    )
                })?;

            let resolved_id = NotificationService::resolve_active_user_id_by_role(
                &mut conn,
                demandeur_user_id,
                "demandeur",
            )
            .map_err(|e| {
                ApiError::InternalServerError(format!("Error resolving demandeur recipient: {}", e))
            })?
            .ok_or_else(|| ApiError::NotFound("Demandeur destinataire introuvable".to_string()))?;

            target_label = NotificationService::resolve_active_user_display(
                &mut conn,
                &resolved_id,
            )
            .map_err(|e| {
                ApiError::InternalServerError(format!("Error resolving demandeur label: {}", e))
            })?;

            recipient_ids.push(resolved_id);
        }
        "all_demandeurs" | "tous_demandeurs" => {
            recipient_ids = NotificationService::find_user_ids_by_roles(&mut conn, &["demandeur"])
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Error fetching demandeurs: {}", e))
                })?;
            target_label = Some("Tous les demandeurs".to_string());
        }
        "all_admins" | "tous_admins" => {
            recipient_ids = NotificationService::find_user_ids_by_roles(
                &mut conn,
                &["administrateur", "regulateur", "admin"],
            )
            .map_err(|e| {
                ApiError::InternalServerError(format!("Error fetching all admins: {}", e))
            })?;
            target_label = Some("Tous les administrateurs".to_string());
        }
        _ => {
            return Err(ApiError::ValidationError(
                "target_type invalide (admins|admin|porter|all_porters|all_demandeurs|all_admins|demandeur)".to_string(),
            ));
        }
    }

    recipient_ids.retain(|recipient_id| recipient_id != &user.id);

    if recipient_ids.is_empty() {
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "sent_to_admins": 0,
            "sent_to_porters": 0,
            "sent_to_total": 0,
            "target_type": target_type,
            "target_label": target_label
        })));
    }

    let notif = NotificationService::user_message(
        sender_display.as_str(),
        user.role.as_str(),
        channel.as_str(),
        message,
    );

    for recipient_id in recipient_ids {
        let stored =
            NotificationService::persist_if_enabled(&mut conn, recipient_id.as_str(), &notif, None)
                .map_err(|e| {
                    ApiError::InternalServerError(format!("Error sending message: {}", e))
                })?;

        if stored.is_some() {
            if target_type == "porter" || target_type == "brancardier" {
                sent_to_porters += 1;
            } else {
                sent_to_admins += 1;
            }
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "sent_to_admins": sent_to_admins,
        "sent_to_porters": sent_to_porters,
        "sent_to_total": sent_to_admins + sent_to_porters,
        "target_type": target_type,
        "target_label": target_label
    })))
}

/// DELETE /api/notifications/{id} - Hard delete one notification of current user
#[delete("/api/notifications/{notification_id}")]
async fn delete_notification(
    pool: web::Data<DbPool>,
    user: web::ReqData<User>,
    notification_id: web::Path<String>,
) -> ApiResult<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| ApiError::InternalServerError(format!("Database connection error: {}", e)))?;

    let deleted = NotificationService::delete_user_notification(
        &mut conn,
        &user.id,
        notification_id.as_str(),
    )
    .map_err(|e| ApiError::InternalServerError(format!("Error deleting notification: {}", e)))?;

    if deleted == 0 {
        return Err(ApiError::NotFound("Notification introuvable".to_string()));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({ "deleted": deleted })))
}

/// Register all notification routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_notifications)
        .service(get_unread_count)
        .service(mark_notifications_read)
        .service(mark_all_notifications_read)
        .service(get_preferences)
        .service(update_preferences)
        .service(list_message_recipients)
        .service(delete_notification)
        .service(send_message_to_admins);
}
