## Auto-extraction (routes + roles)

_Genere automatiquement le 2026-02-25 00:52:18_

### Resume
- Endpoints detectes: **95**
- Publics (exact middleware bypass): **13**
- Compat v1 (publics partiels selon chemin): **7**
- Authentifies (`AuthMiddleware`): **75**
- Endpoints avec roles explicitement inferes: **35**

### Tableau endpoints (pre-remplissage automatique)

| Methode | Path | Auth | Roles infers | Source inference | Handler | Service principal | Notes |
|---|---|---|---|---|---|---|---|
| POST | /api/auth/login | Public | - | none (faible) | login (`backend/src/handlers/auth.rs:9`) | AuthService::login | - |
| POST | /api/auth/logout | Auth | - | none (faible) | logout (`backend/src/handlers/auth.rs:27`) | - | restriction fine a confirmer (service/logique metier) |
| GET | /api/auth/me | Auth | - | none (faible) | me (`backend/src/handlers/auth.rs:20`) | AuthService::me | restriction fine a confirmer (service/logique metier) |
| GET | /api/health | Public | - | none (faible) | api_health (`backend/src/handlers/health.rs:15`) | - | - |
| POST | /api/help-requests/{id}/respond | Auth | brancardier | service (moyenne) | respond_to_help (`backend/src/handlers/tickets/help.rs:45`) | TicketHelpService::respond_to_help | guard detecte: brancardier uniquement |
| GET | /api/notifications | Auth | - | none (faible) | list_notifications (`backend/src/handlers/notifications.rs:10`) | NotificationService::get_user_notifications_filtered | restriction fine a confirmer (service/logique metier) |
| POST | /api/notifications/mark-all-read | Auth | - | none (faible) | mark_all_notifications_read (`backend/src/handlers/notifications.rs:91`) | NotificationService::mark_all_as_read | restriction fine a confirmer (service/logique metier) |
| POST | /api/notifications/mark-read | Auth | - | none (faible) | mark_notifications_read (`backend/src/handlers/notifications.rs:72`) | NotificationService::mark_as_read | restriction fine a confirmer (service/logique metier) |
| GET | /api/notifications/message-recipients | Auth | - | none (faible) | list_message_recipients (`backend/src/handlers/notifications.rs:146`) | NotificationService::list_active_admin_recipients | restriction fine a confirmer (service/logique metier) |
| GET | /api/notifications/preferences | Auth | - | none (faible) | get_preferences (`backend/src/handlers/notifications.rs:108`) | NotificationService::get_or_create_preferences | restriction fine a confirmer (service/logique metier) |
| PATCH | /api/notifications/preferences | Auth | - | none (faible) | update_preferences (`backend/src/handlers/notifications.rs:124`) | NotificationService::update_preferences | restriction fine a confirmer (service/logique metier) |
| POST | /api/notifications/send-message | Auth | - | none (faible) | send_message_to_admins (`backend/src/handlers/notifications.rs:170`) | NotificationService::find_user_ids_by_roles | restriction fine a confirmer (service/logique metier) |
| GET | /api/notifications/unread-count | Auth | - | none (faible) | get_unread_count (`backend/src/handlers/notifications.rs:55`) | NotificationService::count_unread | restriction fine a confirmer (service/logique metier) |
| DELETE | /api/notifications/{notification_id} | Auth | - | none (faible) | delete_notification (`backend/src/handlers/notifications.rs:409`) | NotificationService::delete_user_notification | restriction fine a confirmer (service/logique metier) |
| GET | /api/patients | Auth | - | none (faible) | search_patients (`backend/src/handlers/patients.rs:9`) | PatientQueryService::search | restriction fine a confirmer (service/logique metier) |
| GET | /api/porters | Auth | - | none (faible) | list_porters (`backend/src/handlers/porters.rs:8`) | PorterManagementService::list_porters | restriction fine a confirmer (service/logique metier) |
| POST | /api/porters | Auth | administrateur | service (moyenne) | create_porter (`backend/src/handlers/porters.rs:56`) | PorterManagementService::create_porter | guard detecte: administrateur uniquement |
| GET | /api/porters/available | Auth | - | none (faible) | list_available_porters (`backend/src/handlers/porters.rs:18`) | PorterManagementService::list_available_porters | restriction fine a confirmer (service/logique metier) |
| GET | /api/porters/me/help-requests | Auth | brancardier | service (moyenne) | get_my_help_requests (`backend/src/handlers/tickets/help.rs:76`) | TicketHelpService::get_my_pending_help_requests | guard detecte: brancardier uniquement |
| GET | /api/porters/{id} | Auth | - | none (faible) | get_porter (`backend/src/handlers/porters.rs:28`) | PorterManagementService::get_porter | restriction fine a confirmer (service/logique metier) |
| PATCH | /api/porters/{id}/skills | Auth | administrateur | service (moyenne) | update_porter_skills (`backend/src/handlers/porters.rs:67`) | PorterManagementService::update_porter_skills | guard detecte: administrateur uniquement |
| PATCH | /api/porters/{id}/status | Auth | - | none (faible) | update_porter_status (`backend/src/handlers/porters.rs:39`) | PorterManagementService::update_porter_status | guard detecte: demandeur interdit; restriction fine a confirmer (service/logique metier) |
| GET | /api/priority-rules | Auth | admin, administrateur, moderateur, regulateur | handler_helper (elevee) | get_priority_rules (`backend/src/handlers/priority_rules.rs:147`) | PriorityRulesService::ensure_rules_config | check role via helper handler |
| PUT | /api/priority-rules | Auth | admin, administrateur, moderateur, regulateur | handler_helper (elevee) | update_priority_rules (`backend/src/handlers/priority_rules.rs:197`) | PriorityRulesService::ensure_rules_config | check role via helper handler |
| GET | /api/priority-rules/default | Auth | admin, administrateur, moderateur, regulateur | handler_helper (elevee) | get_priority_rules_default (`backend/src/handlers/priority_rules.rs:165`) | PriorityRulesService::default_rules_json | check role via helper handler |
| POST | /api/priority-rules/restore-default | Auth | admin, administrateur, moderateur, regulateur | handler_helper (elevee) | restore_priority_rules_default (`backend/src/handlers/priority_rules.rs:258`) | PriorityRulesService::ensure_rules_config | check role via helper handler |
| GET | /api/priority-rules/runtime | Auth | - | none (faible) | get_priority_rules_runtime (`backend/src/handlers/priority_rules.rs:176`) | PriorityRulesService::ensure_rules_config | restriction fine a confirmer (service/logique metier) |
| GET | /api/ready | Public | - | none (faible) | api_ready (`backend/src/handlers/health.rs:50`) | - | - |
| GET | /api/referentials/equipment | Auth | administrateur | service (moyenne) | list_equipment_admin (`backend/src/handlers/referentials.rs:80`) | ReferentialCatalogService::list_equipment_admin | require_role/require_admin detecte |
| POST | /api/referentials/equipment | Auth | administrateur | service (moyenne) | create_equipment (`backend/src/handlers/referentials.rs:98`) | ReferentialCatalogService::create_equipment | require_role/require_admin detecte |
| GET | /api/referentials/equipment/active | Auth | - | none (faible) | list_equipment_active (`backend/src/handlers/referentials.rs:89`) | ReferentialCatalogService::list_equipment_active | restriction fine a confirmer (service/logique metier) |
| DELETE | /api/referentials/equipment/{id} | Auth | administrateur | service (moyenne) | deactivate_equipment (`backend/src/handlers/referentials.rs:124`) | ReferentialCatalogService::deactivate_equipment | require_role/require_admin detecte |
| PUT | /api/referentials/equipment/{id} | Auth | administrateur | service (moyenne) | update_equipment (`backend/src/handlers/referentials.rs:108`) | ReferentialCatalogService::update_equipment | require_role/require_admin detecte |
| DELETE | /api/referentials/equipment/{id}/hard | Auth | administrateur | service (moyenne) | hard_delete_equipment (`backend/src/handlers/referentials.rs:135`) | ReferentialCatalogService::hard_delete_equipment | require_role/require_admin detecte |
| GET | /api/referentials/services | Auth | administrateur | service (moyenne) | list_services_admin (`backend/src/handlers/referentials.rs:14`) | ReferentialCatalogService::list_services_admin | require_role/require_admin detecte |
| POST | /api/referentials/services | Auth | administrateur | service (moyenne) | create_service (`backend/src/handlers/referentials.rs:32`) | ReferentialCatalogService::create_service | require_role/require_admin detecte |
| GET | /api/referentials/services/active | Auth | - | none (faible) | list_services_active (`backend/src/handlers/referentials.rs:23`) | ReferentialCatalogService::list_services_active | restriction fine a confirmer (service/logique metier) |
| DELETE | /api/referentials/services/{id} | Auth | administrateur | service (moyenne) | deactivate_service (`backend/src/handlers/referentials.rs:58`) | ReferentialCatalogService::deactivate_service | require_role/require_admin detecte |
| PUT | /api/referentials/services/{id} | Auth | administrateur | service (moyenne) | update_service (`backend/src/handlers/referentials.rs:42`) | ReferentialCatalogService::update_service | require_role/require_admin detecte |
| DELETE | /api/referentials/services/{id}/hard | Auth | administrateur | service (moyenne) | hard_delete_service (`backend/src/handlers/referentials.rs:68`) | ReferentialCatalogService::hard_delete_service | require_role/require_admin detecte |
| GET | /api/referentials/specimens | Auth | administrateur | service (moyenne) | list_specimens_admin (`backend/src/handlers/referentials.rs:217`) | ReferentialCatalogService::list_specimens_admin | require_role/require_admin detecte |
| POST | /api/referentials/specimens | Auth | administrateur | service (moyenne) | create_specimen (`backend/src/handlers/referentials.rs:235`) | ReferentialCatalogService::create_specimen | require_role/require_admin detecte |
| GET | /api/referentials/specimens/active | Auth | - | none (faible) | list_specimens_active (`backend/src/handlers/referentials.rs:226`) | ReferentialCatalogService::list_specimens_active | restriction fine a confirmer (service/logique metier) |
| DELETE | /api/referentials/specimens/{id} | Auth | administrateur | service (moyenne) | deactivate_specimen (`backend/src/handlers/referentials.rs:261`) | ReferentialCatalogService::deactivate_specimen | require_role/require_admin detecte |
| PUT | /api/referentials/specimens/{id} | Auth | administrateur | service (moyenne) | update_specimen (`backend/src/handlers/referentials.rs:245`) | ReferentialCatalogService::update_specimen | require_role/require_admin detecte |
| DELETE | /api/referentials/specimens/{id}/hard | Auth | administrateur | service (moyenne) | hard_delete_specimen (`backend/src/handlers/referentials.rs:272`) | ReferentialCatalogService::hard_delete_specimen | require_role/require_admin detecte |
| GET | /api/referentials/transport-modes | Auth | administrateur | service (moyenne) | list_transport_modes_admin (`backend/src/handlers/referentials.rs:148`) | ReferentialCatalogService::list_transport_modes_admin | require_role/require_admin detecte |
| POST | /api/referentials/transport-modes | Auth | administrateur | service (moyenne) | create_transport_mode (`backend/src/handlers/referentials.rs:166`) | ReferentialCatalogService::create_transport_mode | require_role/require_admin detecte |
| GET | /api/referentials/transport-modes/active | Auth | - | none (faible) | list_transport_modes_active (`backend/src/handlers/referentials.rs:157`) | ReferentialCatalogService::list_transport_modes_active | restriction fine a confirmer (service/logique metier) |
| DELETE | /api/referentials/transport-modes/{id} | Auth | administrateur | service (moyenne) | deactivate_transport_mode (`backend/src/handlers/referentials.rs:193`) | ReferentialCatalogService::deactivate_transport_mode | require_role/require_admin detecte |
| PUT | /api/referentials/transport-modes/{id} | Auth | administrateur | service (moyenne) | update_transport_mode (`backend/src/handlers/referentials.rs:177`) | ReferentialCatalogService::update_transport_mode | require_role/require_admin detecte |
| DELETE | /api/referentials/transport-modes/{id}/hard | Auth | administrateur | service (moyenne) | hard_delete_transport_mode (`backend/src/handlers/referentials.rs:204`) | ReferentialCatalogService::hard_delete_transport_mode | require_role/require_admin detecte |
| GET | /api/services | Auth | - | none (faible) | list_services (`backend/src/handlers/services.rs:8`) | HospitalServiceQueryService::list_all | restriction fine a confirmer (service/logique metier) |
| GET | /api/tickets | Auth | - | none (faible) | list_tickets (`backend/src/handlers/tickets/core.rs:17`) | TicketCoreService::list_payloads_from_pool | restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets | Auth | - | none (faible) | create_ticket (`backend/src/handlers/tickets/core.rs:4`) | TicketCoreService::create_from_pool | restriction fine a confirmer (service/logique metier) |
| GET | /api/tickets/{id} | Auth | - | none (faible) | get_ticket (`backend/src/handlers/tickets/core.rs:30`) | TicketCoreService::get_payload_from_pool | restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets/{id}/assign | Auth | - | none (faible) | assign_ticket (`backend/src/handlers/tickets/assignment.rs:16`) | - | restriction fine a confirmer (service/logique metier) |
| GET | /api/tickets/{id}/available-porters-for-help | Auth | - | none (faible) | get_available_porters_for_help (`backend/src/handlers/tickets/help.rs:92`) | TicketHelpService::get_available_porters_for_help | restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets/{id}/cancel | Auth | - | none (faible) | cancel_ticket (`backend/src/handlers/tickets/assignment.rs:156`) | TicketAssignmentActionService::cancel_from_pool | restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets/{id}/co-partners | Auth | - | none (faible) | add_co_partner (`backend/src/handlers/tickets/assignment.rs:37`) | TicketAssignmentActionService::add_co_partner_from_pool | restriction fine a confirmer (service/logique metier) |
| DELETE | /api/tickets/{id}/co-partners/{porter_id} | Auth | - | none (faible) | remove_co_partner (`backend/src/handlers/tickets/assignment.rs:56`) | TicketAssignmentActionService::remove_co_partner_from_pool | restriction fine a confirmer (service/logique metier) |
| PATCH | /api/tickets/{id}/equipment-status | Auth | - | none (faible) | update_equipment_status (`backend/src/handlers/tickets/updates.rs:4`) | TicketUpdateService::update_equipment_status | guard detecte: demandeur interdit; restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets/{id}/hard-delete | Auth | - | none (faible) | hard_delete_ticket (`backend/src/handlers/tickets/assignment.rs:168`) | TicketAssignmentActionService::hard_delete_from_pool | restriction fine a confirmer (service/logique metier) |
| PATCH | /api/tickets/{id}/notes | Auth | administrateur | service (moyenne) | update_ticket_notes (`backend/src/handlers/tickets/updates.rs:50`) | TicketUpdateService::update_notes | require_role/require_admin detecte |
| POST | /api/tickets/{id}/pause | Auth | - | none (faible) | pause_ticket (`backend/src/handlers/tickets/assignment.rs:105`) | TicketAssignmentActionService::pause_from_pool | restriction fine a confirmer (service/logique metier) |
| PATCH | /api/tickets/{id}/priority | Auth | admin, administrateur, moderateur, regulateur | service (moyenne) | update_ticket_priority (`backend/src/handlers/tickets/updates.rs:98`) | TicketUpdateService::override_priority | require_role/require_admin detecte |
| POST | /api/tickets/{id}/reassign | Auth | - | none (faible) | reassign_ticket (`backend/src/handlers/tickets/assignment.rs:118`) | TicketAssignmentActionService::reassign_from_pool | restriction fine a confirmer (service/logique metier) |
| GET | /api/tickets/{id}/recommendations | Auth | - | none (faible) | get_recommendations (`backend/src/handlers/tickets/assignment.rs:93`) | TicketAssignmentActionService::recommendations_from_pool | restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets/{id}/request-help | Auth | brancardier | service (moyenne) | request_help (`backend/src/handlers/tickets/help.rs:4`) | TicketHelpService::request_help | guard detecte: brancardier uniquement |
| PATCH | /api/tickets/{id}/status | Auth | - | none (faible) | update_ticket_status (`backend/src/handlers/tickets/assignment.rs:74`) | TicketAssignmentActionService::update_status_from_pool | restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets/{id}/take | Auth | - | none (faible) | take_ticket (`backend/src/handlers/tickets/assignment.rs:26`) | - | restriction fine a confirmer (service/logique metier) |
| POST | /api/tickets/{id}/unassign | Auth | - | none (faible) | unassign_ticket (`backend/src/handlers/tickets/assignment.rs:137`) | TicketAssignmentActionService::unassign_from_pool | restriction fine a confirmer (service/logique metier) |
| GET | /api/users | Auth | administrateur | service (moyenne) | list_users (`backend/src/handlers/users.rs:7`) | AdminUserService::list_users | require_role/require_admin detecte |
| POST | /api/users | Auth | administrateur | service (moyenne) | create_user (`backend/src/handlers/users.rs:13`) | AdminUserService::create_user | require_role/require_admin detecte |
| DELETE | /api/users/{id} | Auth | administrateur | service (moyenne) | deactivate_user (`backend/src/handlers/users.rs:34`) | AdminUserService::deactivate_user | require_role/require_admin detecte |
| PUT | /api/users/{id} | Auth | administrateur | service (moyenne) | update_user (`backend/src/handlers/users.rs:23`) | AdminUserService::update_user | require_role/require_admin detecte |
| DELETE | /api/users/{id}/gdpr | Auth | self+admin | service (moyenne) | delete_user_gdpr (`backend/src/handlers/gdpr.rs:11`) | GdprService::delete_user_data | guard detecte: administrateur OU proprietaire (self) |
| GET | /api/users/{id}/gdpr/export | Auth | self+admin | service (moyenne) | export_user_data (`backend/src/handlers/gdpr.rs:22`) | GdprService::export_user_data | guard detecte: administrateur OU proprietaire (self) |
| DELETE | /api/v1 | Public | - | none (faible) | compat_redirect_root (`backend/src/handlers/api_compat.rs:17`) | - | - |
| GET | /api/v1 | Public | - | none (faible) | compat_redirect_root (`backend/src/handlers/api_compat.rs:17`) | - | - |
| HEAD | /api/v1 | Public | - | none (faible) | compat_redirect_root (`backend/src/handlers/api_compat.rs:17`) | - | - |
| OPTIONS | /api/v1 | Public | - | none (faible) | compat_redirect_root (`backend/src/handlers/api_compat.rs:17`) | - | - |
| PATCH | /api/v1 | Public | - | none (faible) | compat_redirect_root (`backend/src/handlers/api_compat.rs:17`) | - | - |
| POST | /api/v1 | Public | - | none (faible) | compat_redirect_root (`backend/src/handlers/api_compat.rs:17`) | - | - |
| PUT | /api/v1 | Public | - | none (faible) | compat_redirect_root (`backend/src/handlers/api_compat.rs:17`) | - | - |
| DELETE | /api/v1/{tail:.*} | Partiel (v1) | - | none (faible) | compat_redirect (`backend/src/handlers/api_compat.rs:32`) | - | redirect compat /api/v1 -> /api |
| GET | /api/v1/{tail:.*} | Partiel (v1) | - | none (faible) | compat_redirect (`backend/src/handlers/api_compat.rs:32`) | - | redirect compat /api/v1 -> /api |
| HEAD | /api/v1/{tail:.*} | Partiel (v1) | - | none (faible) | compat_redirect (`backend/src/handlers/api_compat.rs:32`) | - | redirect compat /api/v1 -> /api |
| OPTIONS | /api/v1/{tail:.*} | Partiel (v1) | - | none (faible) | compat_redirect (`backend/src/handlers/api_compat.rs:32`) | - | redirect compat /api/v1 -> /api |
| PATCH | /api/v1/{tail:.*} | Partiel (v1) | - | none (faible) | compat_redirect (`backend/src/handlers/api_compat.rs:32`) | - | redirect compat /api/v1 -> /api |
| POST | /api/v1/{tail:.*} | Partiel (v1) | - | none (faible) | compat_redirect (`backend/src/handlers/api_compat.rs:32`) | - | redirect compat /api/v1 -> /api |
| PUT | /api/v1/{tail:.*} | Partiel (v1) | - | none (faible) | compat_redirect (`backend/src/handlers/api_compat.rs:32`) | - | redirect compat /api/v1 -> /api |
| GET | /api/version | Public | - | none (faible) | api_version (`backend/src/handlers/health.rs:77`) | - | - |
| GET | /health | Public | - | none (faible) | health (`backend/src/handlers/health.rs:7`) | - | - |
| GET | /ready | Public | - | none (faible) | ready (`backend/src/handlers/health.rs:23`) | - | - |

### Notes
- `Public` = chemin explicitement exempté par `backend/src/middleware/auth.rs`.
- `Partiel (v1)` = route de compatibilite `/api/v1/...`; le middleware bypass seulement certains chemins v1 (login/health/ready/version/root).
- `Roles infers` = pre-remplissage automatique, a valider manuellement pour les cas metier complexes.
