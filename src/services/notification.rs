use crate::models::{
    default_preferences, DbNotification, NewDbNotification, NewNotificationPreference,
    NotificationPreference,
};
use crate::schema::{notification_preferences, notifications, porters, users};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ===================================================================
// NotificationType enum
// ===================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    TicketCreated,
    TicketAssigned,
    TicketUpdated,
    TicketCompleted,
    TicketCanceled,
    UrgentTicket,
    HelprequestReceived,
    UserMessage,
    CoPartnerAdded,
    CoPartnerRemoved,
    MissionSuspended,
    MissionResumed,
    RdvActivated,
    RdvOverdue,
}

impl NotificationType {
    pub fn as_str(&self) -> &str {
        match self {
            NotificationType::TicketCreated => "ticket_created",
            NotificationType::TicketAssigned => "ticket_assigned",
            NotificationType::TicketUpdated => "ticket_updated",
            NotificationType::TicketCompleted => "ticket_completed",
            NotificationType::TicketCanceled => "ticket_canceled",
            NotificationType::UrgentTicket => "urgent_ticket",
            NotificationType::HelprequestReceived => "help_request_received",
            NotificationType::UserMessage => "user_message",
            NotificationType::CoPartnerAdded => "co_partner_added",
            NotificationType::CoPartnerRemoved => "co_partner_removed",
            NotificationType::MissionSuspended => "mission_suspended",
            NotificationType::MissionResumed => "mission_resumed",
            NotificationType::RdvActivated => "rdv_activated",
            NotificationType::RdvOverdue => "rdv_overdue",
        }
    }
}

// ===================================================================
// In-memory Notification struct (backward compat)
// ===================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub priority: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageAdminRecipient {
    pub id: String,
    pub username: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessagePorterRecipient {
    pub porter_id: String,
    pub user_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageDemandeurRecipient {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
}

// ===================================================================
// NotificationService - Factory + Persistence
// ===================================================================

pub struct NotificationService;

type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

impl NotificationService {
    // --- Factory methods ---

    /// Cree une notification pour un ticket cree
    pub fn ticket_created(ticket_id: &str, priority: i32) -> Notification {
        let priority_label = match priority {
            1 => "Urgence",
            2 => "Prioritaire",
            3 => "Standard",
            4 => "Programme",
            _ => "Standard",
        };

        Notification {
            notification_type: NotificationType::TicketCreated,
            title: "Nouvelle demande".to_string(),
            message: format!("Nouvelle demande {} creee", priority_label),
            priority: if priority == 1 { "high".to_string() } else { "normal".to_string() },
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "priority": priority
            }),
        }
    }

    /// Cree une notification pour un ticket assigne
    pub fn ticket_assigned(ticket_id: &str, porter_name: &str) -> Notification {
        Notification {
            notification_type: NotificationType::TicketAssigned,
            title: "Mission assignee".to_string(),
            message: format!("Assignee a {}", porter_name),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "porter_name": porter_name
            }),
        }
    }

    /// Cree une notification pour un ticket mis a jour
    pub fn ticket_updated(ticket_id: &str, new_status: &str) -> Notification {
        let status_label = match new_status {
            "in_progress" => "En cours",
            "picked_up" => "Prise en charge",
            "arrived" => "Arrive",
            "suspended" | "paused" => "Suspendue",
            "completed" => "Terminee",
            _ => new_status,
        };

        Notification {
            notification_type: NotificationType::TicketUpdated,
            title: "Statut mis a jour".to_string(),
            message: format!("Statut: {}", status_label),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "status": new_status
            }),
        }
    }

    /// Cree une notification pour un ticket termine
    pub fn ticket_completed(ticket_id: &str) -> Notification {
        Notification {
            notification_type: NotificationType::TicketCompleted,
            title: "Mission terminee".to_string(),
            message: "Le transport a ete effectue avec succes".to_string(),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id
            }),
        }
    }

    /// Cree une notification pour un ticket annule
    pub fn ticket_canceled(ticket_id: &str, reason: &str) -> Notification {
        Notification {
            notification_type: NotificationType::TicketCanceled,
            title: "Mission annulee".to_string(),
            message: format!("Raison: {}", reason),
            priority: "high".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "reason": reason
            }),
        }
    }

    /// Cree une alerte urgente P1 a destination des brancardiers disponibles.
    pub fn urgent_ticket(ticket_id: &str) -> Notification {
        Notification {
            notification_type: NotificationType::UrgentTicket,
            title: "Alerte P1".to_string(),
            message: "Mission P1 en attente - premier repondant".to_string(),
            priority: "high".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "priority": 1
            }),
        }
    }

    /// Cree une notification pour une demande d'aide
    pub fn help_requested(ticket_id: &str, requester_name: &str) -> Notification {
        Notification {
            notification_type: NotificationType::HelprequestReceived,
            title: "Demande d'aide".to_string(),
            message: format!("{} a besoin d'assistance", requester_name),
            priority: "high".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "requester_name": requester_name
            }),
        }
    }

    /// Cree une notification de messagerie vers les administrateurs
    pub fn user_message(
        sender_display: &str,
        sender_role: &str,
        channel: &str,
        text: &str,
    ) -> Notification {
        Notification {
            notification_type: NotificationType::UserMessage,
            title: format!("Message ({})", channel),
            message: format!("{}: {}", sender_display, text),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "sender_display": sender_display,
                "sender_role": sender_role,
                "channel": channel,
                "text": text
            }),
        }
    }

    pub fn co_partner_added(ticket_id: &str, porter_name: &str) -> Notification {
        Notification {
            notification_type: NotificationType::CoPartnerAdded,
            title: "Renfort ajoute".to_string(),
            message: format!("{} rejoint la mission", porter_name),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "porter_name": porter_name
            }),
        }
    }

    pub fn co_partner_removed(ticket_id: &str, porter_name: &str) -> Notification {
        Notification {
            notification_type: NotificationType::CoPartnerRemoved,
            title: "Renfort retire".to_string(),
            message: format!("{} a ete retire de la mission", porter_name),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id,
                "porter_name": porter_name
            }),
        }
    }

    pub fn mission_suspended(ticket_id: &str) -> Notification {
        Notification {
            notification_type: NotificationType::MissionSuspended,
            title: "Mission suspendue".to_string(),
            message: "La mission est en pause".to_string(),
            priority: "high".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id
            }),
        }
    }

    pub fn mission_resumed(ticket_id: &str) -> Notification {
        Notification {
            notification_type: NotificationType::MissionResumed,
            title: "Mission reprise".to_string(),
            message: "La mission a repris".to_string(),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id
            }),
        }
    }

    pub fn rdv_activated(ticket_id: &str) -> Notification {
        Notification {
            notification_type: NotificationType::RdvActivated,
            title: "RDV active".to_string(),
            message: "La mission programmee est active".to_string(),
            priority: "normal".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id
            }),
        }
    }

    pub fn rdv_overdue(ticket_id: &str) -> Notification {
        Notification {
            notification_type: NotificationType::RdvOverdue,
            title: "RDV depasse".to_string(),
            message: "Le RDV programme est depasse".to_string(),
            priority: "high".to_string(),
            data: serde_json::json!({
                "ticket_id": ticket_id
            }),
        }
    }

    // --- Persistence methods ---

    /// Persist a notification for a single user
    pub fn persist_for_user(
        conn: &mut DbConn,
        user_id: &str,
        notif: &Notification,
        ticket_id: Option<&str>,
    ) -> Result<DbNotification, diesel::result::Error> {
        let new_notif = NewDbNotification {
            id: format!("notif-{}", Uuid::new_v4()),
            user_id: user_id.to_string(),
            notification_type: notif.notification_type.as_str().to_string(),
            title: notif.title.clone(),
            message: notif.message.clone(),
            priority: notif.priority.clone(),
            data: Some(notif.data.clone()),
            is_read: false,
            related_ticket_id: ticket_id.map(|s| s.to_string()),
        };

        diesel::insert_into(notifications::table)
            .values(&new_notif)
            .get_result::<DbNotification>(conn)
    }

    /// Persist a notification for multiple users at once
    pub fn persist_for_users(
        conn: &mut DbConn,
        user_ids: &[String],
        notif: &Notification,
        ticket_id: Option<&str>,
    ) -> Result<Vec<DbNotification>, diesel::result::Error> {
        let new_notifs: Vec<NewDbNotification> = user_ids
            .iter()
            .map(|uid| NewDbNotification {
                id: format!("notif-{}", Uuid::new_v4()),
                user_id: uid.clone(),
                notification_type: notif.notification_type.as_str().to_string(),
                title: notif.title.clone(),
                message: notif.message.clone(),
                priority: notif.priority.clone(),
                data: Some(notif.data.clone()),
                is_read: false,
                related_ticket_id: ticket_id.map(|s| s.to_string()),
            })
            .collect();

        diesel::insert_into(notifications::table)
            .values(&new_notifs)
            .get_results::<DbNotification>(conn)
    }

    /// Get notifications for a user (most recent first, with limit)
    pub fn get_user_notifications(
        conn: &mut DbConn,
        user_id: &str,
        limit: i64,
    ) -> Result<Vec<DbNotification>, diesel::result::Error> {
        Self::get_user_notifications_filtered(conn, user_id, limit, None)
    }

    /// Get notifications for a user, optionally filtered by type
    pub fn get_user_notifications_filtered(
        conn: &mut DbConn,
        user_id: &str,
        limit: i64,
        notif_type: Option<&str>,
    ) -> Result<Vec<DbNotification>, diesel::result::Error> {
        let mut query =
            notifications::table.filter(notifications::user_id.eq(user_id)).into_boxed();

        if let Some(notification_type) = notif_type {
            query = query.filter(notifications::notification_type.eq(notification_type));
        }

        query.order(notifications::created_at.desc()).limit(limit).load::<DbNotification>(conn)
    }

    /// Delete one notification for the current user (hard delete)
    pub fn delete_user_notification(
        conn: &mut DbConn,
        user_id: &str,
        notification_id: &str,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            notifications::table
                .filter(notifications::user_id.eq(user_id))
                .filter(notifications::id.eq(notification_id)),
        )
        .execute(conn)
    }

    /// List active admin recipients
    pub fn list_active_admin_recipients(
        conn: &mut DbConn,
    ) -> Result<Vec<MessageAdminRecipient>, diesel::result::Error> {
        let rows = users::table
            .filter(users::role.eq_any(["administrateur", "regulateur", "admin"]))
            .filter(users::is_active.eq(true))
            .order((users::last_name.asc(), users::first_name.asc(), users::username.asc()))
            .select((users::id, users::username, users::role, users::first_name, users::last_name))
            .load::<(String, String, String, String, String)>(conn)?;

        Ok(rows
            .into_iter()
            .map(|(id, username, role, first_name, last_name)| {
                let full_name =
                    format!("{} {}", first_name.trim(), last_name.trim()).trim().to_string();
                let display_name = if full_name.is_empty() {
                    username.clone()
                } else {
                    format!("{} ({})", full_name, username)
                };
                MessageAdminRecipient { id, username, role, first_name, last_name, display_name }
            })
            .collect())
    }

    /// List active porter recipients (porter id + user id)
    pub fn list_active_porter_recipients(
        conn: &mut DbConn,
    ) -> Result<Vec<MessagePorterRecipient>, diesel::result::Error> {
        let rows = porters::table
            .inner_join(users::table.on(users::id.eq(porters::user_id)))
            .filter(users::is_active.eq(true))
            .filter(users::role.eq("brancardier"))
            .order((porters::status.asc(), users::last_name.asc(), users::first_name.asc()))
            .select((
                porters::id,
                users::id,
                users::username,
                users::first_name,
                users::last_name,
                porters::status,
            ))
            .load::<(String, String, String, String, String, String)>(conn)?;

        Ok(rows
            .into_iter()
            .map(|(porter_id, user_id, username, first_name, last_name, status)| {
                let full_name =
                    format!("{} {}", first_name.trim(), last_name.trim()).trim().to_string();
                let display_name = if full_name.is_empty() {
                    format!("{} ({})", porter_id, username)
                } else {
                    format!("{} [{}]", full_name, porter_id)
                };
                MessagePorterRecipient {
                    porter_id,
                    user_id,
                    username,
                    first_name,
                    last_name,
                    status,
                    display_name,
                }
            })
            .collect())
    }

    /// List active demandeur recipients
    pub fn list_active_demandeur_recipients(
        conn: &mut DbConn,
    ) -> Result<Vec<MessageDemandeurRecipient>, diesel::result::Error> {
        let rows = users::table
            .filter(users::role.eq("demandeur"))
            .filter(users::is_active.eq(true))
            .order((users::last_name.asc(), users::first_name.asc(), users::username.asc()))
            .select((users::id, users::username, users::first_name, users::last_name))
            .load::<(String, String, String, String)>(conn)?;

        Ok(rows
            .into_iter()
            .map(|(id, username, first_name, last_name)| {
                let full_name =
                    format!("{} {}", first_name.trim(), last_name.trim()).trim().to_string();
                let display_name = if full_name.is_empty() {
                    username.clone()
                } else {
                    format!("{} ({})", full_name, username)
                };
                MessageDemandeurRecipient { id, username, first_name, last_name, display_name }
            })
            .collect())
    }

    /// Resolve porter id to active user id
    pub fn resolve_active_porter_user_id(
        conn: &mut DbConn,
        porter_id: &str,
    ) -> Result<Option<String>, diesel::result::Error> {
        porters::table
            .inner_join(users::table.on(users::id.eq(porters::user_id)))
            .filter(porters::id.eq(porter_id))
            .filter(users::is_active.eq(true))
            .filter(users::role.eq("brancardier"))
            .select(users::id)
            .first::<String>(conn)
            .optional()
    }

    /// Resolve active user id by role
    pub fn resolve_active_user_id_by_role(
        conn: &mut DbConn,
        user_id: &str,
        role: &str,
    ) -> Result<Option<String>, diesel::result::Error> {
        users::table
            .filter(users::id.eq(user_id))
            .filter(users::role.eq(role))
            .filter(users::is_active.eq(true))
            .select(users::id)
            .first::<String>(conn)
            .optional()
    }

    /// Resolve active user display name
    pub fn resolve_active_user_display(
        conn: &mut DbConn,
        user_id: &str,
    ) -> Result<Option<String>, diesel::result::Error> {
        users::table
            .filter(users::id.eq(user_id))
            .filter(users::is_active.eq(true))
            .select((users::username, users::first_name, users::last_name))
            .first::<(String, String, String)>(conn)
            .optional()
            .map(|row| {
                row.map(|(username, first_name, last_name)| {
                    let full_name =
                        format!("{} {}", first_name.trim(), last_name.trim()).trim().to_string();
                    if full_name.is_empty() {
                        username
                    } else {
                        format!("{} ({})", full_name, username)
                    }
                })
            })
    }

    /// Resolve active porter display name
    pub fn resolve_active_porter_display(
        conn: &mut DbConn,
        porter_id: &str,
    ) -> Result<Option<String>, diesel::result::Error> {
        porters::table
            .inner_join(users::table.on(users::id.eq(porters::user_id)))
            .filter(porters::id.eq(porter_id))
            .filter(users::is_active.eq(true))
            .filter(users::role.eq("brancardier"))
            .select((users::username, users::first_name, users::last_name))
            .first::<(String, String, String)>(conn)
            .optional()
            .map(|row| {
                row.map(|(username, first_name, last_name)| {
                    let full_name =
                        format!("{} {}", first_name.trim(), last_name.trim()).trim().to_string();
                    if full_name.is_empty() {
                        format!("{} ({})", porter_id, username)
                    } else {
                        format!("{} [{}]", full_name, porter_id)
                    }
                })
            })
    }

    /// Count unread notifications for a user
    pub fn count_unread(conn: &mut DbConn, user_id: &str) -> Result<i64, diesel::result::Error> {
        notifications::table
            .filter(notifications::user_id.eq(user_id))
            .filter(notifications::is_read.eq(false))
            .count()
            .get_result::<i64>(conn)
    }

    /// Mark specific notifications as read
    pub fn mark_as_read(
        conn: &mut DbConn,
        user_id: &str,
        notification_ids: &[String],
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(
            notifications::table
                .filter(notifications::user_id.eq(user_id))
                .filter(notifications::id.eq_any(notification_ids)),
        )
        .set(notifications::is_read.eq(true))
        .execute(conn)
    }

    /// Mark ALL notifications as read for a user
    pub fn mark_all_as_read(
        conn: &mut DbConn,
        user_id: &str,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(
            notifications::table
                .filter(notifications::user_id.eq(user_id))
                .filter(notifications::is_read.eq(false)),
        )
        .set(notifications::is_read.eq(true))
        .execute(conn)
    }

    /// Get or create notification preferences for a user
    pub fn get_or_create_preferences(
        conn: &mut DbConn,
        user_id: &str,
    ) -> Result<NotificationPreference, diesel::result::Error> {
        let existing = notification_preferences::table
            .filter(notification_preferences::user_id.eq(user_id))
            .first::<NotificationPreference>(conn);

        match existing {
            Ok(pref) => Ok(pref),
            Err(diesel::result::Error::NotFound) => {
                let new_pref = NewNotificationPreference {
                    id: format!("npref-{}", Uuid::new_v4()),
                    user_id: user_id.to_string(),
                    preferences: default_preferences(),
                    sound_enabled: true,
                };

                diesel::insert_into(notification_preferences::table)
                    .values(&new_pref)
                    .get_result::<NotificationPreference>(conn)
            }
            Err(e) => Err(e),
        }
    }

    /// Update notification preferences for a user
    pub fn update_preferences(
        conn: &mut DbConn,
        user_id: &str,
        new_preferences: Option<serde_json::Value>,
        new_sound_enabled: Option<bool>,
    ) -> Result<NotificationPreference, diesel::result::Error> {
        // Ensure preferences exist first
        let _existing = Self::get_or_create_preferences(conn, user_id)?;

        if let Some(prefs) = new_preferences {
            diesel::update(
                notification_preferences::table
                    .filter(notification_preferences::user_id.eq(user_id)),
            )
            .set(notification_preferences::preferences.eq(prefs))
            .execute(conn)?;
        }

        if let Some(sound) = new_sound_enabled {
            diesel::update(
                notification_preferences::table
                    .filter(notification_preferences::user_id.eq(user_id)),
            )
            .set(notification_preferences::sound_enabled.eq(sound))
            .execute(conn)?;
        }

        notification_preferences::table
            .filter(notification_preferences::user_id.eq(user_id))
            .first::<NotificationPreference>(conn)
    }

    /// Find user IDs by role (for broadcasting notifications)
    pub fn find_user_ids_by_role(
        conn: &mut DbConn,
        role: &str,
    ) -> Result<Vec<String>, diesel::result::Error> {
        users::table
            .filter(users::role.eq(role))
            .filter(users::is_active.eq(true))
            .select(users::id)
            .load::<String>(conn)
    }

    /// Find user IDs by multiple roles
    pub fn find_user_ids_by_roles(
        conn: &mut DbConn,
        roles: &[&str],
    ) -> Result<Vec<String>, diesel::result::Error> {
        users::table
            .filter(users::role.eq_any(roles))
            .filter(users::is_active.eq(true))
            .select(users::id)
            .load::<String>(conn)
    }

    /// Check if user has a specific notification type enabled
    pub fn is_notification_enabled(
        preferences: &serde_json::Value,
        notification_type: &str,
    ) -> bool {
        preferences
            .get(notification_type)
            .and_then(|v| v.get("enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// Persist notification for a user, respecting their preferences
    pub fn persist_if_enabled(
        conn: &mut DbConn,
        user_id: &str,
        notif: &Notification,
        ticket_id: Option<&str>,
    ) -> Result<Option<DbNotification>, diesel::result::Error> {
        let prefs = Self::get_or_create_preferences(conn, user_id)?;
        let notif_type = notif.notification_type.as_str();

        if Self::is_notification_enabled(&prefs.preferences, notif_type) {
            let result = Self::persist_for_user(conn, user_id, notif, ticket_id)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}
