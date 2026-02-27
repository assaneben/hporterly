# Commercial License Terms (Indicative)

**Indicative terms / non-contractual**

This document summarizes common topics for proprietary licensing of Hporterly.
It is provided for planning purposes only, is not legal advice, and does not
constitute a binding offer. Final terms are defined only in a signed agreement.

## Dual licensing model

- Open-source option: `GPL-3.0-or-later`
- Commercial option: proprietary license available upon request
- Typical use cases: OEM/white-label, proprietary redistribution, closed-source integrations
- Contact: `Couverture@ik.me` or the `Commercial License Request` GitHub issue template

## Typical commercial scope options

- Proprietary distribution rights
- OEM / white-label rights
- Closed-source integration rights
- Priority support and incident response
- Custom features / roadmap commitments
- Deployment architecture and hardening assistance

## Healthcare deployments

### 1) Hosting & compliance

- Deployment options may include customer-managed on-premises, private cloud, or managed cloud.
- Hosting providers can be selected to meet regional healthcare requirements, including the ability to use HDS-certified hosting in France (or equivalent qualified providers elsewhere).
- Environment separation (development, test, staging, production) should be defined with controlled promotion paths.
- Backup options may include encrypted backups, retention schedules, restore testing, and documented recovery objectives (RPO/RTO).

### 2) Security

- Encryption in transit (TLS) and encryption at rest (database, backups, object storage) can be specified.
- Secrets management options may include vault-based storage, short-lived credentials, and environment-specific scoping.
- Key rotation procedures and certificate renewal responsibilities should be documented.
- Hardening expectations may include OS baselines, least privilege, minimal services, and network segmentation.
- Vulnerability scanning (dependency, container, host) and security patching SLAs can be defined by severity.
- Logging policy should explicitly prohibit sensitive data in logs and define retention, access, and redaction controls.

### 3) Identity & access

- Optional SSO integration via OIDC and/or SAML can be scoped.
- RBAC can be configured to match operational roles and separation-of-duties needs.
- Optional MFA may be supported via the identity provider and/or access layer.
- Account lifecycle controls should cover provisioning, role changes, deprovisioning, periodic reviews, and break-glass access procedures.

### 4) Traceability

- Audit trail options may include immutable event logging for workflow actions, user identity, timestamps, and outcome changes.
- Timestamp handling should define timezone strategy and clock synchronization assumptions.
- Retention policies can be aligned with customer policy and regulatory obligations.
- Export capabilities for audit purposes may include CSV/JSON and documented event schemas.
- Audit requirements should clarify capture scope, access permissions, and tamper-evidence expectations.

### 5) GDPR/health data

- Contracting roles (controller/processor or equivalent) should be clearly defined in the commercial agreement.
- DPIA support may include architecture/data-flow documentation and control mapping inputs.
- A DPA may be provided where applicable.
- Incident notification procedures should define channels, timelines, severity criteria, and coordination responsibilities.
- Data minimization principles should guide field design, logs, exports, and retention configuration.

### 6) SLA/support

- Support tiers may include business-hours coverage and optional 24/7 support.
- Response and restoration targets can be defined by severity.
- On-call arrangements may be included for production incidents.
- Support channels may include email, ticket portal, chat, and scheduled review calls.
- Escalation paths (technical, security, management) should be documented.

### 7) Reversibility

- Data export options should specify supported formats (for example CSV, JSON, SQL dump) and schema documentation.
- Return/deletion procedures should define timelines, verification, and deletion attestations where applicable.
- Migration assistance may include extraction tooling, mapping workshops, and cutover support.
- Reversibility planning should address records, audit logs, and integration configuration artifacts.

### 8) HIS integration

- Optional integrations may include HL7 and/or FHIR connectors depending on scope and target systems.
- Connector work typically requires interface specifications, test payloads, and a non-production integration environment.
- Test environments and validation plans should be agreed before production cutover.
- Integration deliverables should define ownership for message mapping, retries, monitoring, and error handling.

## Typical commercial process

1. Initial scoping (use case, deployment model, constraints)
2. Technical/security questionnaire
3. Commercial proposal (license scope + support options)
4. Contracting and delivery planning

## Contact

- Email: `Couverture@ik.me`
- Alternative: GitHub issue template `Commercial License Request`
