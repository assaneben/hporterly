# User Workflows

This document summarizes the user-facing business workflows currently supported by Hporterly.

## Product scope

Hporterly supports internal hospital transport operations for:

- patient transport
- equipment logistics
- specimen routing

Operational constraints:

- `MDR Class I` logistics only
- no automated clinical decision
- no public documentation of private integration routes

Related interoperability references:

- [FHIR R4 Overview](FHIR-R4.md)
- [HL7v2 + Mirth Overview](HL7v2-MIRTH-INTEGRATION.md)
- [CDA R2 Overview](CDA-R2.md)

## User roles

### Demandeur

Typical profile:

- care staff
- secretariat
- operational requester

Main responsibilities:

- create a request
- consult own requests
- follow progress
- communicate with operations

### Brancardier

Typical profile:

- field transporter
- mobile or tablet operator

Main responsibilities:

- receive assigned missions
- take or update missions
- request help for co-portage
- update operational status

### Regulateur / Administrateur

Typical profile:

- supervisor
- dispatcher
- operations administrator

Main responsibilities:

- supervise queue and workload
- assign, reassign, pause, cancel
- manage priorities
- manage referentials, users, and porter capabilities

## Shared operational rules

- Every public action is role-protected
- Authentication uses JWT; MFA can add a second verification step
- Mission statuses follow controlled transitions
- Sensitive operational actions are designed to be auditable
- Demo and test content must stay synthetic only

## Workflow 1: Demandeur

### 1. Authenticate

The user logs in with `username/password` or `email/password`.

If MFA is active:

- login returns a temporary session token
- the user verifies TOTP or a backup code
- a full access token is issued only after MFA verification

### 2. Create a request

The demandeur creates a transport request through `POST /api/tickets`.

Supported business scopes include:

- patient transport
- equipment movement
- specimen transport

The request carries operational data such as:

- origin
- destination
- priority
- mode or transport subtype
- notes
- planned time if applicable

### 3. Follow the request

The demandeur can:

- list own tickets
- read ticket detail
- receive notification updates
- send operational messages through the notifications API

### 4. Cancel when allowed

A demandeur can only cancel within the rules enforced by the workflow service.
Completed requests are not reopened from the demandeur workflow.

## Workflow 2: Brancardier

### 1. Authenticate

The brancardier logs in through the same auth flow, with MFA when enabled.

### 2. Receive or take a mission

Missions can be:

- assigned by a regulateur
- self-taken when the workflow allows it

Helpful endpoints:

- `GET /api/tickets`
- `GET /api/tickets/{id}`
- `POST /api/tickets/{id}/take`

### 3. Execute the mission

Typical mission flow:

`assigned -> in_progress -> arrived -> completed`

Available operational controls include:

- start or update status
- pause when allowed
- cancel when allowed
- update mission notes
- update equipment-delivery status

### 4. Request help

If a mission requires assistance or co-portage, the brancardier can:

- request help
- see available co-porters
- respond to help requests

### 5. Manage availability

The brancardier can expose availability through the porter status endpoints and receive notification updates.

## Workflow 3: Regulateur / Administrateur

### 1. Supervise queue

The regulateur monitors:

- pending tickets
- assigned tickets
- mission bottlenecks
- porter availability

### 2. Dispatch work

The regulateur or administrator can:

- assign
- reassign
- unassign
- pause
- cancel
- override priority

### 3. Manage operational catalogs

Administrative users manage:

- services
- equipment
- transport modes
- specimen referentials
- priority rules
- users
- porter profiles and skills

### 4. Communicate with teams

The notifications module supports:

- unread counts
- notification preferences
- recipient discovery
- targeted operational messages

## Mission status reference

Normalized statuses visible to clients:

- `pending`
- `assigned`
- `in_progress`
- `arrived`
- `suspended`
- `completed`
- `canceled`

The workflow service enforces allowed transitions. Clients should not assume arbitrary status jumps are accepted.

## Priority reference

Operational priorities are normalized from `1` to `4`.

- `1`: highest urgency
- `4`: lowest urgency or scheduled/programmed case

Priority rules and manual overrides remain operational decisions, not clinical automation.

## Identity vigilance

INS-related field workflow is documented separately:

- [INS Confirmation Flow](ins-confirmation-flow.md)

This dedicated document should be used for bracelet verification, mismatch handling, and transport blocking rules.

## Notifications and audit

User-facing actions may trigger:

- notification entries
- unread counters
- targeted messages
- audit-oriented backend actions

Public user documentation does not list internal audit schemas or private reporting channels.

## Interoperability touchpoints

User workflows can intersect with interoperability features without exposing those private channels directly:

- upstream patient and transport context can originate from hospital integration flows documented in [HL7v2 + Mirth Overview](HL7v2-MIRTH-INTEGRATION.md)
- completed transport execution can feed private downstream reporting documented in [CDA R2 Overview](CDA-R2.md)
- a standards-oriented public interoperability target is described separately in [FHIR R4 Overview](FHIR-R4.md)

## Private integrations

The published product also contains private hospital integration capabilities, but they are intentionally outside the public user contract.

Examples:

- inbound patient-update channels
- private reporting/export channels

These routes and ports are intentionally omitted from user-facing documentation.

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: role-visible workflows are documented without exposing private/internal capabilities.
  - OK SBD-09: no real patient, INS, or facility data is included.
  - OK SBD-21: fail-secure expectations are reflected in controlled workflow descriptions.
  - OK SBD-22: workflow documentation is now explicitly connected to the separate interoperability documents.
- Not fully satisfiable in this file:
  - WARN SBD-10: audit behavior is described functionally but enforced by runtime services, not by documentation.
    Alternative: keep runtime audit tests and append-only database controls.
