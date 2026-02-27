<!-- AUTO-GENERATED:START -->
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
<!-- AUTO-GENERATED:END -->

---

﻿# 08 - Contrats API et WebSocket [A COMPLETER]


<!-- AUTO-PAYLOADS:START -->
## Pre-remplissage automatique des payloads (endpoints principaux)

Source de verite utilisee pour ce pre-remplissage (structs/handlers/services Rust):
- `backend/src/models/user.rs` (`LoginRequest`, `LoginResponse`, `UserInfo`)
- `backend/src/models/porter.rs` (`Porter`, `CreatePorterRequest`, `UpdatePorterStatus`, `UpdatePorterSkills`)
- `backend/src/models/patient.rs` (`SearchPatientsQuery`, `PatientSearchResult`)
- `backend/src/models/service.rs` (`Service`)
- `backend/src/models/ticket.rs` (`CreateTicketRequest`, `Ticket`, `UpdateTicketStatus`, `AssignTicketRequest`)
- `backend/src/models/ticket_assignment.rs` (`TicketAssignment`, `AddCoPartnerRequest`, `DesignateSuccessorRequest`)
- `backend/src/models/help_request.rs` (`HelpRequest`, `CreateHelpRequest`, `RespondToHelpRequest`)
- `backend/src/services/admin_users.rs` (`CreateUserRequest`, `UpdateUserRequest`, `AdminUserResponse`)
- `backend/src/handlers/tickets/updates.rs` (payloads JSON inline pour `equipment-status`, `hard-delete`, etc.)
- `backend/src/handlers/health.rs` (payloads `health`, `ready`, `version`)
- `backend/src/utils/error.rs` (format d'erreur standard)

### Conventions API observees
- Auth API: JWT Bearer via header `Authorization: Bearer <token>` (sauf endpoints publics)
- Reponse erreur standard (`ApiError`):

```json
{
  "error": "BAD_REQUEST",
  "message": "Bad Request: ..."
}
```

- Les endpoints `tickets` peuvent renvoyer soit:
  - un `Ticket` serialise (`backend/src/models/ticket.rs`)
  - un payload enrichi JSON (`Ticket` + champs calcules / role-aware) via `TicketReadService`

### Endpoints principaux (request/response pre-remplis)

#### `GET /api/health` (public)
Reponse `200`:
```json
{
  "status": "ok",
  "service": "hporterly-api"
}
```

#### `GET /api/ready` (public)
Reponse `200` (DB OK):
```json
{
  "status": "ready",
  "service": "hporterly-api"
}
```
Reponse `503` (DB indisponible):
```json
{
  "status": "error",
  "service": "hporterly-api"
}
```

#### `GET /api/version` (public)
Reponse `200`:
```json
{
  "service": "hporterly-api",
  "default_version": "v1",
  "supported_versions": ["v1"],
  "base_paths": {
    "versioned": "/api/v1",
    "legacy_compat": "/api"
  },
  "compatibility_mode": true
}
```

#### `POST /api/auth/login`
Struct request: `LoginRequest`
```json
{
  "username": "marie.durand",
  "password": "password123"
}
```

Struct response: `LoginResponse` + `UserInfo`
```json
{
  "token": "<jwt>",
  "user": {
    "id": "usr-...",
    "username": "marie.durand",
    "role": "demandeur",
    "first_name": "Marie",
    "last_name": "Durand",
    "email": null,
    "service": "Unit A",
    "porter_id": null
  }
}
```

Erreurs typiques:
- `401` utilisateur/mot de passe invalide
- `401` compte desactive

#### `GET /api/auth/me`
Reponse `200` (`UserInfo`):
```json
{
  "id": "usr-...",
  "username": "jean.martin",
  "role": "brancardier",
  "first_name": "Jean",
  "last_name": "Martin",
  "email": null,
  "service": "Transport",
  "porter_id": "porter-01"
}
```

#### `POST /api/auth/logout`
Reponse `200`:
```json
{
  "message": "Deconnexion reussie"
}
```

#### `GET /api/services`
Reponse `200` (`Vec<Service>`):
```json
[
  {
    "id": "svc-unit-a",
    "name": "Unit A",
    "building": "BAT-A",
    "floor": "Niveau 1",
    "full_name": "Unit A (BAT-A - Niveau 1 - R-101)",
    "created_at": "2026-02-24T10:15:00"
  }
]
```

#### `GET /api/patients?search=<term>&limit=<n>`
Query (`SearchPatientsQuery`):
- `search` (obligatoire)
- `limit` (optionnel, defaut `10`)

Reponse `200` (`Vec<PatientSearchResult>`):
```json
[
  {
    "id": "IPP-100001",
    "first_name": "Alice",
    "last_name": "Martin",
    "age": 42,
    "gender": "F",
    "service": "Unit B",
    "room": "R-204",
    "building": "BAT-B",
    "floor": "Niveau 2",
    "date_of_birth": "1983-05-12",
    "sex": "F"
  }
]
```

#### `GET /api/porters`
Reponse `200` (`Vec<Porter>`)

#### `GET /api/porters/available`
Reponse `200` (`Vec<Porter>`) - meme shape que `Porter`, filtree cote service

#### `GET /api/porters/{id}`
Reponse `200` (`Porter`):
```json
{
  "id": "porter-01",
  "user_id": "usr-...",
  "status": "available",
  "skills": ["O2", "LIT"],
  "current_location": "Unit A",
  "completed_missions_today": 3,
  "total_missions": 128,
  "rating": 4.7,
  "created_at": "2026-02-20T09:00:00",
  "updated_at": "2026-02-25T08:45:00"
}
```

#### `PATCH /api/porters/{id}/status`
Struct request: `UpdatePorterStatus`
```json
{
  "status": "busy",
  "location": "Unit A"
}
```
Reponse `200`: `Porter`

#### `POST /api/porters`
Struct request: `CreatePorterRequest`
```json
{
  "username": "porter.demo",
  "password": "password123",
  "first_name": "Alex",
  "last_name": "Rossi",
  "skills": ["O2", "LIT"]
}
```
Reponse `201`: `Porter`

#### `PATCH /api/porters/{id}/skills`
Struct request: `UpdatePorterSkills`
```json
{
  "skills": ["O2", "LIT", "URG"]
}
```
Reponse `200`: `Porter`

#### `GET /api/tickets`
Query params (derive de `TicketListParams::from_query`):
- `include_archived` (`true/false` ou `1/0`)
- `status`
- `priority`
- `porter_id`
- `transport_type`
- `limit` (defaut `200`, clamp `1..500`)
- `offset` (defaut `0`)

Reponse `200`: `Vec<Value>` (payload enrichi)
- Base = `Ticket` serialise
- Enrichissements detectes (`TicketReadService::enrich_ticket_payload_for_role`):
  - `supervisor_porter_id`
  - `co_partner_ids`
  - `co_partners` (alias du tableau)
  - `requester_display` / `requester_username` / `requester_full_name` (selon role)

Exemple (shape simplifiee):
```json
[
  {
    "id": "BR-HPly-001-2026",
    "patient_id": "IPP-100001",
    "patient_name": "Alice Martin",
    "origin": "Unit A",
    "destination": "Imaging",
    "priority": 2,
    "mode": "brancard",
    "status": "assigned",
    "porter_id": "porter-01",
    "requester_id": "usr-123",
    "notes": "Transport standard fictif",
    "transport_type": "PATIENT",
    "transport_subtype": "TP-BRANC",
    "created_at": "2026-02-25T09:10:00",
    "updated_at": "2026-02-25T09:12:00",
    "supervisor_porter_id": "porter-01",
    "co_partner_ids": [],
    "co_partners": []
  }
]
```

#### `GET /api/tickets/{id}`
Reponse `200`: meme shape que `GET /api/tickets` (payload enrichi single object)

#### `POST /api/tickets`
Struct request principal: `CreateTicketRequest` (tres riche). Utiliser un minimum valide + extensions optionnelles.

Exemple minimal (transport patient):
```json
{
  "patient_id": "IPP-100001",
  "patient_name": "Alice Martin",
  "origin": "Unit A",
  "destination": "Imaging",
  "priority": 3,
  "mode": "brancard",
  "notes": "DEMO DATA - SYNTHETIC / NOT REAL",
  "transport_type": "PATIENT",
  "transport_subtype": "TP-BRANC"
}
```

Champs optionnels importants supportes (selection):
- Vigilances / contraintes: `needs_o2`, `needs_perfusion`, `isolation`, `needs_two_porters`, `patient_monitoring`, `patient_agitated`
- Programmation: `scheduled_time`, `activation_minutes_before`
- Identite patient detaillee: `patient_first_name`, `patient_last_name`, `patient_dob` (`YYYY-MM-DD`), `patient_sex`, `patient_ipp`, `motif`
- Transport materiel: `equipment_recipient_patient_id`, `equipment_recipient_patient_name`, `equipment_size`, `equipment_return_service`
- Specimens: `laboratory_name`, `specimen_types`, `notes_for_reception`
- Vigilances supplementaires: `patient_contentious`, `patient_confused`, `patient_over_120kg`, `patient_bariatric`, `patient_psychiatry`, `patient_dialysis`, `patient_icu`, `other_precautions`

Reponse `201`: `Ticket` (non enrichi)
```json
{
  "id": "BR-HPly-001-2026",
  "patient_id": "IPP-100001",
  "patient_name": "Alice Martin",
  "origin": "Unit A",
  "destination": "Imaging",
  "priority": 3,
  "mode": "brancard",
  "status": "pending",
  "porter_id": null,
  "requester_id": "usr-123",
  "needs_o2": false,
  "needs_perfusion": false,
  "isolation": false,
  "patient_weight": null,
  "patient_agitated": false,
  "patient_monitoring": false,
  "needs_two_porters": false,
  "notes": "DEMO DATA - SYNTHETIC / NOT REAL",
  "scheduled_time": null,
  "created_at": "2026-02-25T09:10:00",
  "updated_at": "2026-02-25T09:10:00",
  "completed_at": null,
  "transport_type": "PATIENT",
  "transport_subtype": "TP-BRANC"
}
```

#### `POST /api/tickets/{id}/assign` et `POST /api/tickets/{id}/take`
Struct request: `AssignTicketRequest`
```json
{
  "porter_id": "porter-01"
}
```
Notes:
- `porter_id` peut etre omis pour self-assignment selon le workflow backend (cas `take` / porteur authentifie)

Reponse `200`: `Ticket`

#### `PATCH /api/tickets/{id}/status`
Struct request: `UpdateTicketStatus`
```json
{
  "status": "in_progress",
  "comment": "Prise en charge demarree"
}
```
Reponse `200`: `Ticket`

#### `GET /api/tickets/{id}/recommendations`
Reponse `200`: tableau de recommandations (`porter` + `score`)
```json
[
  {
    "porter": {
      "id": "porter-01",
      "user_id": "usr-...",
      "status": "available",
      "skills": ["O2"],
      "current_location": "Unit A",
      "completed_missions_today": 1,
      "total_missions": 50,
      "rating": 4.8,
      "created_at": "2026-02-20T09:00:00",
      "updated_at": "2026-02-25T09:00:00"
    },
    "score": 92
  }
]
```

#### `POST /api/tickets/{id}/co-partners`
Struct request: `AddCoPartnerRequest`
```json
{
  "porter_id": "porter-02"
}
```
Reponse `200`: `TicketAssignment`

#### `DELETE /api/tickets/{id}/co-partners/{porter_id}`
Body: aucun
Reponse `200`: `TicketAssignment`

#### `POST /api/tickets/{id}/request-help`
Struct request: `CreateHelpRequest`
```json
{
  "requested_porter_id": "porter-02"
}
```
Reponse `200` (JSON inline handler):
```json
{
  "success": true,
  "help_request_id": "HELP-ab12cd34",
  "message": "Demande d'aide envoyee"
}
```

#### `POST /api/help-requests/{id}/respond`
Struct request: `RespondToHelpRequest`
```json
{
  "accepted": true
}
```
Reponse `200`:
```json
{
  "success": true,
  "accepted": true,
  "status": "accepted"
}
```

#### `GET /api/porters/me/help-requests`
Reponse `200`: `Vec<HelpRequest>`
```json
[
  {
    "id": "HELP-ab12cd34",
    "ticket_id": "BR-HPly-001-2026",
    "requesting_porter_id": "porter-01",
    "requested_porter_id": "porter-02",
    "status": "pending",
    "created_at": "2026-02-25T09:30:00",
    "responded_at": null
  }
]
```

#### `GET /api/tickets/{id}/available-porters-for-help`
Reponse `200`: `Vec<Porter>`

#### `PATCH /api/tickets/{id}/equipment-status`
Struct request (handler local):
```json
{
  "delivered": true,
  "label_returned": false
}
```
Reponse `200` (JSON inline handler):
```json
{
  "success": true,
  "delivered": true,
  "label_returned": false,
  "completed": false
}
```

#### `PATCH /api/tickets/{id}/notes`
Struct request (handler local):
```json
{
  "notes": "Note operationnelle fictive"
}
```
Reponse `200`: `Ticket`

#### `PATCH /api/tickets/{id}/priority`
Struct request (handler local):
```json
{
  "priority": 1,
  "reason": "Escalade operationnelle fictive"
}
```
Reponse `200`: `Ticket`

#### `POST /api/tickets/{id}/reassign`
Struct request: `AssignTicketRequest`
```json
{
  "porter_id": "porter-03"
}
```
Reponse `200`: `Ticket`

#### `POST /api/tickets/{id}/unassign`
Struct request optionnelle: `DesignateSuccessorRequest`
```json
{
  "next_supervisor_porter_id": "porter-02",
  "comment": "Transmission planifiee"
}
```
Reponse `200`: `Ticket`

#### `POST /api/tickets/{id}/pause`
Body: aucun
Reponse `200`: `Ticket`

#### `POST /api/tickets/{id}/cancel`
Body: aucun
Reponse `200`: `Ticket`

#### `POST /api/tickets/{id}/hard-delete`
Struct request local (handler):
```json
{
  "reason": "Suppression de donnee de demo"
}
```
Reponse `200`:
```json
{
  "id": "BR-HPly-001-2026",
  "deleted": true
}
```

#### `GET /api/users`
Reponse `200`: `Vec<AdminUserResponse>`

#### `POST /api/users`
Struct request: `CreateUserRequest`
```json
{
  "username": "demo.requester",
  "password": "password123",
  "role": "demandeur",
  "first_name": "Nora",
  "last_name": "Lefevre",
  "email": null,
  "service": "Unit A"
}
```
Reponse `201`: `AdminUserResponse`
```json
{
  "id": "usr-...",
  "username": "demo.requester",
  "role": "demandeur",
  "first_name": "Nora",
  "last_name": "Lefevre",
  "email": null,
  "service": "Unit A",
  "porter_id": null,
  "is_active": true,
  "created_at": "2026-02-25T09:00:00",
  "updated_at": "2026-02-25T09:00:00"
}
```

#### `PUT /api/users/{id}`
Struct request: `UpdateUserRequest` (partiel)
```json
{
  "first_name": "Nora",
  "last_name": "Martin",
  "service": "Unit B",
  "is_active": true
}
```
Reponse `200`: `AdminUserResponse`

#### `DELETE /api/users/{id}`
Reponse `200` (JSON inline service):
```json
{
  "success": true
}
```

<!-- AUTO-PAYLOADS-EXTRA:START -->
### Complements automatiques - payloads `notifications` et `referentials/*`

Source de verite utilisee (scan code):
- `backend/src/handlers/notifications.rs`
- `backend/src/models/notification.rs`
- `backend/src/services/notification.rs`
- `backend/src/handlers/referentials.rs`
- `backend/src/models/referential.rs`
- `backend/src/services/referentials.rs`

#### Notifications - DTOs et payloads (pre-remplis)

##### `GET /api/notifications`
Query params (`NotificationQuery`):
- `limit` (optionnel, borne `1..200`, defaut handler `50`)
- `notification_type` (optionnel, ex: `ticket_created`, `ticket_assigned`, `user_message`)

Reponse `200`: `Vec<DbNotification>`
```json
[
  {
    "id": "notif-...",
    "user_id": "usr-...",
    "notification_type": "ticket_assigned",
    "title": "Mission assignee",
    "message": "Assignee a Alex Rossi",
    "priority": "normal",
    "data": {
      "ticket_id": "BR-HPly-001-2026",
      "porter_name": "Alex Rossi"
    },
    "is_read": false,
    "related_ticket_id": "BR-HPly-001-2026",
    "created_at": "2026-02-25T10:00:00"
  }
]
```

##### `GET /api/notifications/unread-count`
Reponse `200` (`UnreadCountResponse`):
```json
{ "count": 4 }
```

##### `POST /api/notifications/mark-read`
Request (`MarkReadRequest`):
```json
{
  "notification_ids": ["notif-123", "notif-456"]
}
```
Reponse `200`:
```json
{ "updated": 2 }
```

##### `POST /api/notifications/mark-all-read`
Body: aucun
Reponse `200`:
```json
{ "updated": 12 }
```

##### `GET /api/notifications/preferences`
Reponse `200` (`NotificationPreference`):
```json
{
  "id": "npref-...",
  "user_id": "usr-...",
  "preferences": {
    "ticket_created": { "enabled": true, "sound": true },
    "ticket_assigned": { "enabled": true, "sound": true },
    "ticket_updated": { "enabled": true, "sound": false },
    "user_message": { "enabled": true, "sound": true }
  },
  "sound_enabled": true,
  "created_at": "2026-02-25T09:00:00",
  "updated_at": "2026-02-25T10:00:00"
}
```

##### `PATCH /api/notifications/preferences`
Request (`UpdatePreferencesRequest`):
```json
{
  "preferences": {
    "ticket_created": { "enabled": true, "sound": true },
    "ticket_assigned": { "enabled": true, "sound": true },
    "ticket_updated": { "enabled": true, "sound": false },
    "ticket_completed": { "enabled": true, "sound": false },
    "user_message": { "enabled": true, "sound": true }
  },
  "sound_enabled": true
}
```
Reponse `200`: `NotificationPreference`

##### `GET /api/notifications/message-recipients`
Reponse `200` (`MessageRecipientsResponse`):
```json
{
  "admins": [
    {
      "id": "usr-admin-1",
      "username": "admin",
      "role": "administrateur",
      "first_name": "Admin",
      "last_name": "Demo",
      "display_name": "Admin Demo (admin)"
    }
  ],
  "porters": [
    {
      "porter_id": "porter-01",
      "user_id": "usr-porter-1",
      "username": "jean.martin",
      "first_name": "Jean",
      "last_name": "Martin",
      "status": "available",
      "display_name": "Jean Martin [porter-01]"
    }
  ],
  "demandeurs": [
    {
      "id": "usr-req-1",
      "username": "marie.durand",
      "first_name": "Marie",
      "last_name": "Durand",
      "display_name": "Marie Durand (marie.durand)"
    }
  ]
}
```

##### `POST /api/notifications/send-message`
Request (`SendUserMessageRequest`) - exemple envoi aux administrateurs:
```json
{
  "message": "Message de test synthetique",
  "channel": "general",
  "target_type": "admins"
}
```

Variantes `target_type` supportees (handler):
- `admins` / `administrateurs` / `administration`
- `admin` / `administrateur` (necessite `target_user_id`)
- `porter` / `brancardier` (necessite `target_porter_id`)
- `all_porters` / `tous_brancardiers`
- `demandeur` (necessite `target_user_id`)
- `all_demandeurs` / `tous_demandeurs`
- `all_admins` / `tous_admins`

Exemple ciblage brancardier:
```json
{
  "message": "Besoin de rappel sur mission en attente",
  "channel": "operations",
  "target_type": "porter",
  "target_porter_id": "porter-01"
}
```

Reponse `200`:
```json
{
  "sent_to_admins": 1,
  "sent_to_porters": 0,
  "sent_to_total": 1,
  "target_type": "admins",
  "target_label": "Administration"
}
```

##### `DELETE /api/notifications/{notification_id}`
Reponse `200`:
```json
{ "deleted": 1 }
```

Erreurs typiques notifications:
- `400` / `VALIDATION_ERROR` (message trop court/long, `target_type` invalide, cible manquante)
- `404` (notification ou destinataire introuvable)

#### Referentials - DTOs et payloads (pre-remplis)

Pattern observe:
- Endpoints `GET .../active`: auth requise, liste active (memes DTOs que les listes admin)
- Endpoints `GET/POST/PUT/DELETE` (admin): controles par `require_admin`
- Endpoints `DELETE .../hard`: reponse JSON standard:

```json
{ "success": true, "deleted": true, "id": "<ID>" }
```

##### `referentials/services`
DTO response: `ReferentialService`
```json
{
  "id": "SVC-...",
  "name": "Unit A",
  "building": "BAT-A",
  "floor": "Niveau 1",
  "is_active": true,
  "created_at": "2026-02-25T10:00:00",
  "site_id": "SITE-DEMO",
  "site_name": "Site Demo",
  "building_id": "BAT-BAT-A",
  "building_name": "BAT-A",
  "level_id": "NIV-NIVEAU-1",
  "level_name": "Niveau 1",
  "zone_id": "ZONE-UNIT-A",
  "zone_name": "Unit A",
  "subzone_id": "SUB-R-101",
  "subzone_name": "R-101"
}
```

Create request (`CreateReferentialServiceRequest`) - minimum utile:
```json
{
  "site_name": "Site Demo",
  "building_name": "BAT-A",
  "level_name": "Niveau 1",
  "zone_name": "Unit A",
  "subzone_name": "R-101"
}
```

Update request (`UpdateReferentialServiceRequest`) - partiel:
```json
{
  "zone_name": "Unit A Updated",
  "subzone_name": "R-102",
  "is_active": true
}
```

Validation observee (service):
- `site_name`, `building_name`, `level_name`, `zone_name` requis (ou derives via alias `name/building/floor`)

##### `referentials/equipment`
DTO response: `ReferentialEquipment`
```json
{
  "id": "EQUIP-...",
  "label": "Stretcher Standard",
  "sizes": ["S", "M", "L"],
  "required_fields": {
    "recipient": false,
    "priority": true,
    "origin": true,
    "destination": true,
    "note_free": false,
    "scheduled": false
  },
  "is_active": true,
  "created_at": "2026-02-25T10:00:00"
}
```

Create request (`CreateReferentialEquipmentRequest`):
```json
{
  "label": "Stretcher Standard",
  "sizes": ["S", "M", "L"],
  "required_fields": {
    "recipient": false,
    "priority": true,
    "origin": true,
    "destination": true,
    "note_free": false,
    "scheduled": false
  }
}
```

Note: si `required_fields` est absent, le backend applique un default interne avec ces memes cles.

Update request (`UpdateReferentialEquipmentRequest`) - partiel:
```json
{
  "label": "Stretcher Standard Updated",
  "sizes": ["M", "L"],
  "is_active": true
}
```

##### `referentials/transport-modes`
DTO response: `ReferentialTransportMode`
```json
{
  "id": "TM-...",
  "label": "Mode Demo",
  "is_active": true,
  "sort_order": 99
}
```

Create request (`CreateReferentialTransportModeRequest`):
```json
{
  "label": "Mode Demo",
  "sort_order": 99
}
```

Update request (`UpdateReferentialTransportModeRequest`) - partiel:
```json
{
  "label": "Mode Demo Updated",
  "sort_order": 100,
  "is_active": true
}
```

##### `referentials/specimens`
DTO response: `ReferentialSpecimen`
```json
{
  "id": "SPEC-...",
  "label": "Specimen Demo",
  "is_active": true,
  "created_at": "2026-02-25T10:00:00"
}
```

Create request (`CreateReferentialSpecimenRequest`):
```json
{ "label": "Specimen Demo" }
```

Update request (`UpdateReferentialSpecimenRequest`) - partiel:
```json
{
  "label": "Specimen Demo Updated",
  "is_active": true
}
```

##### Listes `referentials/*`
Reponses `200`:
- `GET /api/referentials/services` -> `Vec<ReferentialService>`
- `GET /api/referentials/services/active` -> `Vec<ReferentialService>`
- `GET /api/referentials/equipment` -> `Vec<ReferentialEquipment>`
- `GET /api/referentials/equipment/active` -> `Vec<ReferentialEquipment>`
- `GET /api/referentials/transport-modes` -> `Vec<ReferentialTransportMode>`
- `GET /api/referentials/transport-modes/active` -> `Vec<ReferentialTransportMode>`
- `GET /api/referentials/specimens` -> `Vec<ReferentialSpecimen>`
- `GET /api/referentials/specimens/active` -> `Vec<ReferentialSpecimen>`
<!-- AUTO-PAYLOADS-EXTRA:END -->

### WebSocket (etat observe du code)
- Aucune route WebSocket explicite n'a ete detectee dans `backend/src/handlers` lors du scan automatique de cette revision.
- La dependance `actix-web-actors` est presente, mais la doc de contrat WebSocket reste a completer si une route WS existe hors handlers actuels / branche de reference.

### Points a valider manuellement (avant partage final)
- Payloads exacts des endpoints `notifications` (query params + responses)
- Payloads exacts des endpoints `referentials/*` et `priority-rules/*`
- Shape complete de `Ticket` (liste exhaustive des champs) selon la version cible de l'app
- Codes HTTP/erreurs specifiques par endpoint (409/conflits de workflow notamment)
<!-- AUTO-PAYLOADS:END -->

## 1. Authentification
- Mecanisme: JWT [a confirmer details]
- Endpoint login:
- Payload request:
- Payload response:
- Erreurs possibles:
- Expiration / refresh:

## 2. Endpoints REST (completer endpoint par endpoint)
### Exemple de fiche endpoint
- Methode:
- URL:
- Auth requise:
- Roles autorises:
- Query params:
- Body request (JSON):
- Response 200 (JSON):
- Erreurs (400/401/403/404/409/500):
- Effets de bord (audit, notifications, WS):

## 3. Endpoints a documenter (liste initiale observee)
- `/api/health`
- `/api/auth/login`
- `/api/auth/me`
- `/api/auth/logout`
- `/api/tickets`
- `/api/tickets/:id`
- `/api/tickets/:id/assign`
- `/api/tickets/:id/status`
- `/api/porters`
- `/api/porters/available`
- `/api/services`

## 4. WebSocket (si actif)
- URL / route:
- Authentification:
- Evenements emis serveur -> client:
- Evenements client -> serveur:
- Schema payloads:
- Retry/reconnect:
- Heartbeat:

## 5. Mock / fixtures pour reprise
- Fichiers JSON de reference:
- Jeux de reponses d erreurs:
- Scenarios offline:
