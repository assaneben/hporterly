# Public API Overview

This document describes the public HTTP surface currently exposed by Hporterly.
It intentionally excludes private hospital integration routes and internal-only channels.

## Canonical base paths

- Public application API: `/api`
- Legacy compatibility redirect: `/api/v1/* -> /api/*`
- Health endpoints:
  - `/health`
  - `/ready`
  - `/api/health`
  - `/api/ready`
  - `/api/version`

New clients should use `/api/*`.

## Standards and interoperability notes

This document covers the currently published public `/api/*` surface.

Related interoperability documentation is split intentionally:

- [FHIR R4 Overview](FHIR-R4.md)
  - documents the intended standards-oriented interoperability target
  - does not claim that a full FHIR router is currently published on this branch
- [HL7v2 + Mirth Overview](HL7v2-MIRTH-INTEGRATION.md)
  - documents upstream hospital ingestion as a private integration flow
- [CDA R2 Overview](CDA-R2.md)
  - documents downstream transport reporting as a private reporting flow

This separation prevents private/internal integrations from being mistaken for public API commitments.

## Authentication and session

### Public endpoints

- `POST /api/auth/login`
- `GET /api/auth/me`
- `POST /api/auth/logout`
- `POST /api/auth/mfa/setup`
- `POST /api/auth/mfa/activate`
- `POST /api/auth/mfa/verify`

### Login flow

`POST /api/auth/login` accepts:

- `username` + `password`
- or `email` + `password`

Possible outcomes:

- direct access token:
  - `token`
  - `user`
  - `mfa_required = false`
  - `mfa_verified = true`
- MFA challenge:
  - `token = null`
  - `user = null`
  - `mfa_required = true`
  - `session_token_partiel`
  - `mfa_verified = false`

`POST /api/auth/mfa/verify` exchanges the temporary token for a full access token after TOTP or backup-code verification.

## Operational API families

### Tickets and mission lifecycle

Primary route family:

- `POST /api/tickets`
- `GET /api/tickets`
- `GET /api/tickets/{id}`
- `POST /api/tickets/{id}/assign`
- `POST /api/tickets/{id}/take`
- `PATCH /api/tickets/{id}/status`
- `POST /api/tickets/{id}/pause`
- `POST /api/tickets/{id}/reassign`
- `POST /api/tickets/{id}/unassign`
- `POST /api/tickets/{id}/cancel`
- `POST /api/tickets/{id}/hard-delete`
- `PATCH /api/tickets/{id}/notes`
- `PATCH /api/tickets/{id}/priority`
- `PATCH /api/tickets/{id}/equipment-status`

Help and co-portage support:

- `POST /api/tickets/{id}/request-help`
- `POST /api/help-requests/{id}/respond`
- `GET /api/porters/me/help-requests`
- `GET /api/tickets/{id}/available-porters-for-help`
- `POST /api/tickets/{id}/co-partners`
- `DELETE /api/tickets/{id}/co-partners/{porter_id}`

Recommendations:

- `GET /api/tickets/{id}/recommendations`

### Porter operations

- `GET /api/porters`
- `GET /api/porters/available`
- `GET /api/porters/{id}`
- `PATCH /api/porters/{id}/status`
- `POST /api/porters`
- `PATCH /api/porters/{id}/skills`

### Patient and service lookup

- `GET /api/patients`
- `GET /api/services`

### Notifications and user messaging

- `GET /api/notifications`
- `GET /api/notifications/unread-count`
- `POST /api/notifications/mark-read`
- `POST /api/notifications/mark-all-read`
- `GET /api/notifications/preferences`
- `PATCH /api/notifications/preferences`
- `GET /api/notifications/message-recipients`
- `POST /api/notifications/send-message`
- `DELETE /api/notifications/{notification_id}`

### User administration and GDPR

- `GET /api/users`
- `POST /api/users`
- `PUT /api/users/{id}`
- `DELETE /api/users/{id}`
- `GET /api/users/{id}/gdpr/export`
- `DELETE /api/users/{id}/gdpr`

### Operational referentials

- services:
  - `GET /api/referentials/services`
  - `GET /api/referentials/services/active`
  - `POST /api/referentials/services`
  - `PUT /api/referentials/services/{id}`
  - `DELETE /api/referentials/services/{id}`
  - `DELETE /api/referentials/services/{id}/hard`
- equipment:
  - `GET /api/referentials/equipment`
  - `GET /api/referentials/equipment/active`
  - `POST /api/referentials/equipment`
  - `PUT /api/referentials/equipment/{id}`
  - `DELETE /api/referentials/equipment/{id}`
  - `DELETE /api/referentials/equipment/{id}/hard`
- transport modes:
  - `GET /api/referentials/transport-modes`
  - `GET /api/referentials/transport-modes/active`
  - `POST /api/referentials/transport-modes`
  - `PUT /api/referentials/transport-modes/{id}`
  - `DELETE /api/referentials/transport-modes/{id}`
  - `DELETE /api/referentials/transport-modes/{id}/hard`
- specimens:
  - `GET /api/referentials/specimens`
  - `GET /api/referentials/specimens/active`
  - `POST /api/referentials/specimens`
  - `PUT /api/referentials/specimens/{id}`
  - `DELETE /api/referentials/specimens/{id}`
  - `DELETE /api/referentials/specimens/{id}/hard`

### Priority rules

- `GET /api/priority-rules`
- `GET /api/priority-rules/default`
- `GET /api/priority-rules/runtime`
- `PUT /api/priority-rules`
- `POST /api/priority-rules/restore-default`

## Role-oriented summary

- `Demandeur`
  - create requests
  - list and view own requests
  - consult patient and service lookup
  - receive notifications and send operational messages
- `Brancardier`
  - receive or take missions
  - update mission status
  - request help or respond to help requests
  - manage own porter status and read notifications
- `Regulateur` / `Administrateur`
  - assign, reassign, pause, cancel, and supervise missions
  - override priorities
  - manage referentials, users, porters, and targeted messaging

## Status model

Normalized ticket statuses:

- `pending`
- `assigned`
- `in_progress`
- `arrived`
- `suspended`
- `completed`
- `canceled`

Typical mission flow:

`pending -> assigned -> in_progress -> arrived -> completed`

## Identity vigilance

Patient identity and INS-related workflows are documented separately in:

- [INS Confirmation Flow](ins-confirmation-flow.md)

This public document does not expose private integration routes or internal patient-update channels.

## Explicit exclusions

The following are intentionally not documented here:

- private hospital ingestion channels
- private reporting/export channels
- internal-only integration endpoints
- unpublished FHIR resource routes

If a route or channel is not listed here, it should not be treated as part of the public API contract.

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: only public route families are documented; private/internal channels are explicitly excluded.
  - OK SBD-09: no patient data examples or sensitive values are included.
  - OK SBD-22: public API contract is centralized and auditable, with explicit separation from interoperability documents.
- Not fully satisfiable in this file:
  - WARN SBD-11: rate limiting is described indirectly through auth semantics but enforced in runtime code, not documentation.
    Alternative: keep runtime tests and CI checks for throttling behavior.
