# Interoperability Index

This document centralizes the interoperability-related documentation published for Hporterly.

It provides a public-safe map of the standards and integration flows referenced in this repository.
It does not expose private endpoints, ports, secrets, or deployment-specific topology.

## Why this index exists

Hporterly currently spans three distinct interoperability concerns:

- a standards-oriented future FHIR R4 surface
- a private upstream HL7v2 ingestion flow mediated by Mirth
- a private downstream CDA R2 reporting flow

These concerns have different publication status and different security boundaries.
This index keeps them readable from one place without mixing public API commitments with private integration details.

## Current status summary

| Topic | Purpose | Current published status | Public route status |
| --- | --- | --- | --- |
| FHIR R4 | Standards-oriented interoperability facade for transport workflows | Documented target | Not currently mounted on the published branch |
| HL7v2 + Mirth | Upstream hospital ingestion flow | Documented private integration model | Not public |
| CDA R2 | Downstream completed-transport reporting | Documented implemented reporting flow | Not public |

## FHIR R4

Reference:

- [FHIR R4 Overview](FHIR-R4.md)

What it covers:

- intended `/fhir/r4/*` interoperability direction
- target resource families such as `Patient`, `ServiceRequest`, `Task`, `Location`, and `Practitioner`
- security expectations for JWT, MFA, and deny-by-default access
- explicit statement that the full FHIR router is not currently published on this branch

Use this document when you need to understand:

- where FHIR fits in the product roadmap
- how the intended FHIR resource model maps to transport workflows
- what is and is not currently true on the published branch

## HL7v2 + Mirth

Reference:

- [HL7v2 + Mirth Overview](HL7v2-MIRTH-INTEGRATION.md)

What it covers:

- the `DPI -> Mirth -> normalized JSON -> Hporterly` ingestion model
- supported business events such as admission, transfer, discharge, and transport order creation
- the rule that Hporterly does not parse raw HL7v2 directly
- identity and validation expectations around INS and normalized payload handling

Use this document when you need to understand:

- how patient and transport context enters Hporterly from hospital systems
- where Mirth sits in the trust boundary
- why private ingestion details are intentionally absent from the public repository contract

## CDA R2

Reference:

- [CDA R2 Overview](CDA-R2.md)

What it covers:

- report generation when a transport is completed
- the operational content model of the generated document
- downstream delivery through a private reporting channel
- persistence-backed retry behavior to avoid report loss on transient failure

Use this document when you need to understand:

- how completed transport execution is turned into an interoperable document
- what reliability guarantees are expected from the reporting flow
- why the downstream delivery channel is intentionally not public

## Relationship between the three documents

These documents describe different points in the same interoperability chain:

1. HL7v2 and Mirth describe how upstream hospital data enters the platform.
2. FHIR R4 describes the standards-oriented API target for future public interoperability.
3. CDA R2 describes how completed transport execution is exported downstream.

They should not be read as a single active public API surface.
Only the currently documented `/api/*` routes in [Public API Overview](API.md) are part of the active public contract on this branch.

## Security boundary reminder

The interoperability documents are intentionally split because they belong to different trust zones:

- public application API documentation
- private upstream integration documentation
- private downstream reporting documentation

This separation reduces the risk of:

- accidentally publishing internal integration details
- overstating FHIR availability on the current branch
- confusing private operational channels with public product commitments

## Related documents

- [Public API Overview](API.md)
- [Backend Architecture](ARCHITECTURE.md)
- [Threat Model](THREAT_MODEL.md)
- [User Workflows](USER-WORKFLOWS.md)
- [INS Confirmation Flow](ins-confirmation-flow.md)

## Explicit exclusions

This index intentionally excludes:

- internal route paths
- private ports
- signature headers or secret names
- infrastructure topology
- deployment-specific authentication details

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: the interoperability documentation is centralized without exposing private/internal routes.
  - OK SBD-09: no real patient, INS, facility, or secret values are included.
  - OK SBD-21: publication status and trust-zone separation are documented explicitly to avoid unsafe assumptions.
  - OK SBD-22: FHIR, HL7v2, and CDA documentation is centralized in one auditable index.
- Not fully satisfiable in this file:
  - WARN SBD-08: this document cannot itself enforce transport security, MFA, or encryption-at-rest behavior.
    Alternative: keep those controls in runtime code, deployment policy, and infrastructure validation.
