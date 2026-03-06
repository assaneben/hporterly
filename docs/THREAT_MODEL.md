# Threat Model

This document summarizes the current published threat model for Hporterly.

It is a public-safe threat model for a regulated logistics application in a healthcare context.
It documents trust boundaries, exposed surfaces, likely threats, and existing mitigations without disclosing private routes, secrets, or deployment-sensitive values.

## SecureByDesign session context

- Version: `SecureByDesign v1.1.0`
- Tier: `REGULATED`
- Language: `FR user / EN documentation`
- Deployment context available in this repository:
  - public `/api/*` application surface
  - private HL7 ingestion surface separated from the public server
  - private CDA downstream reporting flow with persistence-backed retry
  - PostgreSQL operational persistence
  - CI and governance controls for locked dependency validation and secret scanning

## System scope

This threat model covers the currently published branch and the following runtime concerns:

- public business API and authentication surface
- mission and ticket workflow handling
- MFA-protected user access
- private upstream hospital event ingestion from normalized JSON produced by Mirth
- private downstream CDA report dispatch with retry persistence
- documentation and CI controls that affect supply-chain and publication trust

This document does not claim to replace a deployment-specific threat model, a penetration test, or a certified audit.

## Assets to protect

Primary assets include:

- patient-linked operational identifiers such as INS
- transport workflow state and assignment data
- user identities, roles, and MFA state
- audit-oriented traces and security-relevant metadata
- CDA transport reports awaiting or completing delivery
- configuration secrets and signing or encryption material
- integrity of published dependency resolution and CI execution

## Trust boundaries

### 1. Public application boundary

The public application boundary includes:

- `/api/*` business routes
- authentication and MFA flows
- notifications and user messaging
- health and readiness endpoints

Primary trust assumptions:

- requests originate from untrusted clients
- every protected route must require valid authentication
- authorization must deny by default on missing permission
- temporary MFA tokens must not grant business access

### 2. Private upstream hospital boundary

The upstream hospital boundary covers the ingestion path for patient and transport events.

Primary trust assumptions:

- raw HL7v2 does not enter Hporterly directly
- upstream normalization is performed by Mirth
- Hporterly consumes only normalized JSON through a private integration surface
- malformed or unauthorized upstream requests must be rejected fail-secure

### 3. Private downstream reporting boundary

The downstream reporting boundary covers CDA document delivery after completed transport execution.

Primary trust assumptions:

- generated CDA documents contain sensitive operational identity context
- downstream delivery can fail transiently
- report loss is unacceptable for traceability
- failed delivery must result in persistence for retry, not silent discard

### 4. Persistence boundary

The persistence boundary covers PostgreSQL-backed operational state.

Primary trust assumptions:

- database data is durable and authoritative for workflow state
- sensitive identity-linked records require strict application-side access control
- audit-oriented data must remain tamper-resistant by design and operations policy

### 5. CI and publication boundary

The CI and publication boundary covers:

- the published repository state
- `Cargo.lock` integrity
- secret scanning and sanity checks
- readiness checks used before publication

Primary trust assumptions:

- dependency resolution drift must be visible and controlled
- documentation must not expose private integration details
- synthetic-only test and seed data rules must remain enforced

## Threat actors and likely abuse sources

Relevant threat actors include:

- unauthenticated internet clients probing the public API
- authenticated users attempting privilege escalation or data overreach
- compromised mobile or workstation sessions using stolen JWTs
- misconfigured or rogue upstream systems sending malformed or unauthorized payloads
- operators or developers leaking secrets or sensitive data through commits or logs
- supply-chain or dependency drift introduced during publication or CI execution

## Attack surfaces and primary threats

### Public authentication and session flows

Primary threats:

- credential stuffing or brute-force login attempts
- MFA bypass through temporary-token misuse
- JWT misuse or replay from compromised clients
- excessive error detail leaking account or security-state information

Relevant mitigations already present in the published branch:

- Argon2 password handling
- MFA enrollment, activation, and verification flow
- rate limiting on login and MFA verification paths
- middleware that blocks temporary MFA tokens from business access
- locked CI validation to reduce dependency drift risk

### Public business API

Primary threats:

- broken access control across tickets, users, and operational data
- privilege escalation across `Demandeur`, `Brancardier`, and `Regulateur`
- injection through malformed request data
- abuse through excessive request volume
- sensitive data leakage in logs or documentation

Relevant mitigations already present in the published branch:

- JWT authentication middleware
- role-based authorization helpers with fail-secure default denial
- explicit CORS origin configuration
- request validation and operational guardrails in service handlers
- documentation guardrails that exclude private/internal endpoints

### Private HL7-derived ingestion flow

Primary threats:

- malformed normalized payloads attempting state corruption
- spoofed upstream calls across the private integration boundary
- patient identity corruption through invalid INS values
- over-trust in upstream systems causing unsafe automatic acceptance

Relevant mitigations already present in the published branch:

- dedicated private ingestion surface separated from the public server
- INS validation helpers including length, numeric-only, and Luhn checks
- constant-time INS comparison helper available for identity-sensitive flows
- public documentation that keeps internal route details out of the repository contract

### CDA downstream reporting flow

Primary threats:

- report loss during downstream outage
- incomplete or inconsistent report generation from missing runtime context
- unintended exposure of private reporting routes or transport contents
- replay or duplicate-delivery handling gaps in downstream processes

Relevant mitigations already present in the published branch:

- report generation tied to completed transport state
- persistence-backed retry model for failed downstream delivery
- public-safe documentation that omits private delivery routes and ports

### Database and audit persistence

Primary threats:

- unauthorized reads of identity-linked operational data
- unauthorized mutation of audit-oriented history
- insecure operational backups or database access outside the application

Mitigations visible from the published application and documentation layer:

- strong application-side authentication and role checks
- append-only audit intent documented and reflected in the data model direction
- explicit no-secret-in-repository policy

Manual-review items still required outside the published code snapshot:

- database role separation
- encryption-at-rest enforcement details
- backup protection and retention operations
- infrastructure-level tamper protections

## Security properties the published branch is designed to provide

The currently published branch is designed to provide:

- deny-by-default authorization behavior
- MFA-aware access control for sensitive routes
- separation between public business traffic and private hospital ingestion
- synthetic-only test and demo data discipline
- no public exposure of private interoperability routes
- no silent loss of completed-transport CDA reports on transient downstream failure

## Residual risks and explicit non-claims

This public threat model does not claim verified production conformance for:

- TLS termination policy in the deployment environment
- encryption at rest for all operational data stores
- reverse-proxy, WAF, or network ACL enforcement
- database administration hardening
- mobile device posture management
- formal FHIR conformance or a live published FHIR router on this branch
- penetration-testing coverage

These items require deployment-specific evidence and operational validation.

## Priority manual review checklist

The highest-priority manual review points for a real deployment are:

1. Verify TLS 1.3 and certificate handling at the reverse proxy or load balancer.
2. Verify secret management and rotation for JWT, MFA, and private integration credentials.
3. Verify private-boundary source restrictions and authenticated transport in production.
4. Verify database backup, retention, and tamper-resistance controls for audit-oriented data.
5. Verify downstream CDA retry operations and alerting on persistent delivery failure.
6. Verify role-isolation and data-access tests across all user-facing endpoints.

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: public and private trust boundaries are documented without exposing private endpoints or deployment secrets.
  - OK SBD-09: no real patient, INS, facility, or secret values are included.
  - OK SBD-21: fail-secure and deny-by-default expectations are documented across trust boundaries.
  - OK SBD-22: the regulated threat model is centralized in one auditable document aligned with the published branch.
- Not fully satisfiable in this file:
  - WARN SBD-08: this document cannot itself prove TLS, transport authentication, or encryption-at-rest enforcement.
    Alternative: verify those controls in deployment configuration, secret management, and infrastructure review.
  - WARN SBD-24: this document cannot validate real production availability and failover behavior.
    Alternative: verify downstream retry operations, monitoring, and disaster-recovery procedures separately.
