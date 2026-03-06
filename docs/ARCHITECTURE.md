# Backend Architecture

This document summarizes the currently published backend architecture of Hporterly.

It focuses on runtime structure, trust boundaries, and operational responsibilities.
It intentionally avoids private endpoint paths, secrets, and deployment-sensitive details.

## Runtime shape

The published backend is a Rust `Actix-web` application with a PostgreSQL persistence layer.

At runtime, the application starts two distinct HTTP surfaces:

- a public application server for `/api/*`, health, readiness, and static frontend delivery
- a private HL7 ingestion server bound separately for internal hospital integration traffic

This separation is part of the security boundary, not just an implementation detail.

## Main backend components

Key published components include:

- `src/main.rs`
  - bootstraps both HTTP servers
  - loads and validates configuration
  - configures CORS, logging, and global rate limiting
- `src/config.rs`
  - loads environment variables
  - validates security-sensitive settings
  - enforces separation between public and internal ports
- `src/middleware/auth.rs`
  - enforces JWT authentication on protected routes
  - blocks temporary MFA tokens from business access
  - requires `mfa_verified=true` for `/fhir/*`
- `src/handlers/*`
  - public business routes for auth, tickets, notifications, referentials, users, and related operations
- `src/hl7/*`
  - internal hospital event ingestion from normalized JSON
- `src/cda/mod.rs`
  - CDA R2 generation and downstream dispatch with persistence-backed retry

## Public application surface

The public server currently exposes:

- health and readiness endpoints
- `/api/*` business routes
- static frontend assets

Documented public route families are described in:

- [Public API Overview](API.md)

The public server also applies:

- CORS with explicit allowed origins
- global rate limiting
- JWT-based authentication middleware
- MFA-aware access rules

## Private hospital integration surface

The backend also runs a distinct private integration surface for HL7 ingestion.

Its responsibilities are:

- receive normalized JSON produced upstream from HL7v2
- validate identity and operational fields
- create or update transport-operational data
- write audit-oriented traces for accepted events

The business overview of this flow is documented separately in:

- [HL7v2 + Mirth Overview](HL7v2-MIRTH-INTEGRATION.md)

This document does not publish private paths, ports, or secret material.

## Security boundaries

### Public business boundary

The public business boundary is designed around:

- JWT authentication
- MFA strengthening on sensitive flows
- role-based authorization
- rate limiting
- explicit CORS origin control

### Private interoperability boundary

The private interoperability boundary is designed around:

- separate server binding
- private/internal-only exposure
- authenticated upstream integration
- source restriction and fail-secure rejection

### Reporting boundary

Completed transport reporting is handled by the CDA module.

That flow:

- generates a CDA R2 document
- sends it to a private downstream integration channel
- persists unsent documents for later retry when downstream delivery fails

The business overview is documented separately in:

- [CDA R2 Overview](CDA-R2.md)

## Persistence model

The application uses PostgreSQL as the operational system of record.

Persistence is used for:

- business entities such as users, tickets, patients, porters, and referentials
- authentication and MFA state
- audit-oriented event traces
- pending CDA delivery retry state

The published branch is not documented here as using an in-memory queue as the primary runtime model.
The architecture should be understood as database-backed.

## Interoperability direction

The current published branch distinguishes between:

- the active `/api/*` business API
- the private HL7 and CDA integration flows
- the intended future `/fhir/r4/*` interoperability surface

FHIR is documented as a target interoperability layer, not as a fully mounted public surface on this branch:

- [FHIR R4 Overview](FHIR-R4.md)

## Operational constraints

This backend remains positioned as:

- logistics-only workflow software
- `MDR Class I`
- no automated clinical decision support
- no clinical scoring based on patient data

Upstream patient context may be displayed for operational execution.
It is not transformed into a clinical recommendation engine.

## Explicit exclusions

This public architecture document intentionally excludes:

- private endpoint paths
- internal ports
- secret names and secret values
- production topology details
- reverse proxy and load balancer configuration
- network-level trust lists

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: public and private trust boundaries are described without exposing internal route details.
  - OK SBD-09: no sensitive runtime values, patient data, or secrets are included.
  - OK SBD-21: deny-by-default and boundary separation are reflected in the architecture description.
  - OK SBD-22: runtime structure and interoperability boundaries are centralized in one auditable document.
- Not fully satisfiable in this file:
  - WARN SBD-08: this architecture note cannot itself prove TLS, downstream auth, or encryption-at-rest enforcement.
    Alternative: keep those controls in deployment policy, runtime configuration, and infrastructure validation.
