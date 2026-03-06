# FHIR R4 Overview

This document describes the intended FHIR R4 interoperability model for Hporterly.

It is a public-safe architecture and contract overview.
It does not claim full FHIR conformance for the currently published branch.

## Current status

On the currently published branch:

- public `/api/*` routes are documented and exposed
- authentication middleware is already prepared to protect `/fhir/*`
- MFA is required by middleware for `/fhir/*` access
- no dedicated `src/fhir` module is currently published
- no public FHIR R4 route family is currently mounted in the published application router

As a result, this document should be read as a target interoperability contract and product direction, not as a statement that the full FHIR surface is already live on this branch.

## Scope

The target role of FHIR R4 in Hporterly is to expose a standards-oriented API for transport coordination workflows.

The primary resource families planned for this interoperability layer are:

- `Patient`
- `ServiceRequest`
- `Task`
- `Location`
- `Practitioner`

This scope remains logistics-oriented and excludes any automated clinical decision support.

## Intended base path

The intended public base path for this interoperability surface is:

- `/fhir/r4/*`

This path is public by design when enabled.
It is distinct from private integration channels and distinct from the current `/api/*` business API.

## Security posture

The existing middleware already establishes an important FHIR-specific rule:

- access to `/fhir/*` requires a valid JWT
- temporary MFA tokens must not grant business access
- `mfa_verified=true` is required for FHIR access

This means the published security posture already anticipates stricter access for FHIR traffic, even though the dedicated FHIR resources are not yet mounted on this branch.

## Target resource model

### Patient

Intended purpose:

- patient lookup for transport context
- patient identity linkage through INS
- display of non-decision operational precautions

Expected identity principles:

- INS remains the primary longitudinal identifier
- no unfiltered patient listing should be exposed
- patient access must stay role-scoped

### ServiceRequest

Intended purpose:

- represent a transport request created by a requester or by an upstream operational source
- carry operational attributes such as destination, priority, type, and notes

Expected status family:

- `draft`
- `active`
- `on-hold`
- `completed`
- `revoked`

### Task

Intended purpose:

- represent the executable mission assigned to a brancardier
- track operational lifecycle and field progression

Expected lifecycle:

- `requested`
- `accepted`
- `in-progress`
- `completed`
- `cancelled`

Expected safety behavior:

- the mission must not progress into active execution without required identity confirmation rules when those rules apply

### Location

Intended purpose:

- publish the operational referential of sites, rooms, and destinations used in transport workflows

### Practitioner

Intended purpose:

- expose regulated staff and transporter profiles relevant to dispatching and assignment

## Identity vigilance expectations

FHIR usage in Hporterly must remain aligned with INS-driven identity rules:

- INS validation before persistence
- deny-by-default on malformed or mismatched identity data
- no identity inference from weak matching
- dedicated transport confirmation workflows remain separate from pure lookup flows

The field confirmation workflow is documented separately in:

- [INS Confirmation Flow](ins-confirmation-flow.md)

## Relationship with the current `/api/*` surface

The currently published `/api/*` routes remain the active operational surface.

The FHIR layer is intended to:

- provide a standards-oriented interoperability facade
- map transport requests to `ServiceRequest`
- map executable missions to `Task`
- map patient identity and context to `Patient`
- map hospital places to `Location`
- map staff identities to `Practitioner`

This document does not replace the current public API documentation.

## Explicit non-claims

This document does not claim that the published branch already provides:

- a complete FHIR router
- a full FHIR CapabilityStatement
- complete profile validation
- formal conformance to a national implementation guide

Those items require implementation evidence and runtime exposure, which are not present in the currently published router.

## Explicit exclusions from public documentation

The following are intentionally excluded here:

- private hospital integration routes
- internal middleware secrets
- infrastructure topology
- hidden deployment paths
- internal-only operational endpoints

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: the document separates public intended FHIR usage from private/internal integrations.
  - OK SBD-09: no patient examples, INS values, or secrets are exposed.
  - OK SBD-21: MFA-gated and deny-by-default expectations for `/fhir/*` are documented explicitly.
  - OK SBD-22: the interoperability target and current-publication status are stated clearly and auditable.
- Not fully satisfiable in this file:
  - WARN SBD-08: this document cannot itself prove live FHIR transport security or runtime conformance.
    Alternative: keep runtime route enforcement, MFA checks, and deployment controls in code and infrastructure.
