# Hporterly

<p align="center">
  <img src="assets/logo.png" alt="HPorterly Logo" width="400" />
</p>

**Solution SaaS de coordination du brancardage et des flux hospitaliers internes**
<br />
**Hospital Patient Transfer Coordination & Operations System**

![CI](https://img.shields.io/github/actions/workflow/status/assaneben/hporterly/ci.yml?branch=main&label=CI)
![License: GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)

Hporterly is a Rust/Actix backend for hospital transport operations:
patient transfers, equipment logistics, specimen routing, mission supervision,
notifications, and audit-friendly workflow tracking.

- Version: `1.0.0`
- Author: Assan ABDOU-OUSSENI
- License: GPL-3.0-or-later
- Device classification: `MDR Class I` (logistics only, no automated clinical decision)

> Strict rule: **Never commit real patient or facility data.**

## Business scope

Hporterly is an operational workflow platform for internal hospital transport.

- No automated clinical scoring or recommendation engine
- Manual operational priority management
- Role-based mission handling
- Patient identity vigilance support, including INS-related workflows
- Private hospital integration channels for inbound updates and outbound reporting

## User roles

- `Demandeur`: creates and follows transport requests from a workstation
- `Brancardier`: receives and executes missions, updates field status, requests help
- `Regulateur` / `Administrateur`: supervises queue, assignments, priorities, referentials, and users

## Core capabilities

- Ticket lifecycle with controlled statuses:
  `pending -> assigned -> in_progress -> arrived -> completed`
- Allowed exception flows for `suspended` and `canceled`
- Priority handling from `1` (highest urgency) to `4` (scheduled/programmed)
- JWT authentication with MFA enrollment, activation, and verification flow
- Historical reporting for supervision with day/week/month/year views and archived data consultation
- Year-based reporting filters with bounded historical dataset export for operational dashboards
- Notification center, unread counters, message recipients, and targeted messages
- Porter availability and skill management
- Patient and service lookup endpoints
- Referential management for services, equipment, transport modes, and specimens
- Cursor-ready ticket listing for large queues (`cursor_created_at`, `cursor_id`)
- Structured JSON logging and Prometheus metrics endpoints for observability
- Production deployment bundle with Traefik TLS termination and PostgreSQL backup automation
- Append-only audit-oriented service layer
- CDA generation on completed transport and private hospital reporting pipeline

## Documentation map

- [User Workflows](docs/USER-WORKFLOWS.md)
- [Public API Overview](docs/API.md)
- [INS Confirmation Flow](docs/ins-confirmation-flow.md)
- [Interoperability Index](docs/INTEROPERABILITY.md)
- [FHIR R4 Overview](docs/FHIR-R4.md)
- [HL7v2 + Mirth Overview](docs/HL7v2-MIRTH-INTEGRATION.md)
- [CDA R2 Overview](docs/CDA-R2.md)
- [Changelog](CHANGELOG.md)
- [PR Delivery Protocol](docs/pr-delivery-protocol.md)
- [Threat Model](docs/THREAT_MODEL.md)
- [Architecture](docs/ARCHITECTURE.md)

## Recent updates

Recent branch evolution is tracked in [CHANGELOG.md](CHANGELOG.md).

Highlights for the current published baseline:

- `2026-03-08`: production deployment bundle hardened with Traefik TLS termination, placeholder-only `.env.production.example`, JSON logs, metrics exposure, and backup automation helpers
- `2026-03-08`: historical supervision reports added with archived ticket consultation, year selection, and bounded reporting dataset endpoint
- `2026-03-08`: MFA persistence finalized with dedicated `mfa_secrets` migration and restored login compatibility for MFA-enabled accounts
- `2026-03-08`: health/readiness aliases (`/healthz`, `/readyz`) and Prometheus metrics (`/metrics`, `/api/metrics`) published for operations tooling
- `2026-03-06`: interoperability index added to centralize FHIR R4, HL7v2, and CDA documentation in one public-safe entry point
- `2026-03-06`: threat model harmonized with the current public API, private HL7 ingestion boundary, and CDA reporting boundary
- `2026-03-06`: FHIR R4 interoperability target documented with explicit non-publication status on the current branch
- `2026-03-06`: CDA R2 reporting flow documented from completed transport to private downstream delivery
- `2026-03-06`: HL7v2 integration overview documented for the `DPI -> Mirth -> normalized JSON -> HPorterly` flow
- `2026-03-06`: public documentation refreshed to match the current public API, user roles, and MFA flow
- `2026-03-06`: CI now enforces locked Cargo validation to prevent silent dependency drift
- `2026-03-06`: `Cargo.lock` refreshed and republished to align dependency resolution with the manifest
- `2026-03-05`: PR governance layer tightened with readiness checks, evidence requirements, and security-oriented delivery protocol
- `2026-03-05`: build stability restored by syncing the full Rust backend surface, fixing Diesel `64-column-tables`, and eliminating Clippy warnings on the published branch

## Quickstart

### Docker / production-like compose

The published `docker-compose.yml` expects the companion frontend repository to be available as a
sibling checkout named `../Hporterly-frontend`.

```bash
git clone https://github.com/assaneben/hporterly.git
git clone https://github.com/assaneben/Hporterly-frontend.git ../Hporterly-frontend
cd hporterly
cp .env.production.example .env.production
docker compose --env-file .env.production up --build
```

Services:

- Public HTTPS entrypoint: `https://<PUBLIC_DOMAIN>` through Traefik
- Health: `/health`, `/healthz`, `/api/health`, `/api/healthz`
- Readiness: `/ready`, `/readyz`, `/api/ready`, `/api/readyz`
- Metrics: `/metrics`, `/api/metrics` (internal scraping only, do not expose publicly)
- Backup job on demand: `docker compose --env-file .env.production run --rm db-backup`

### Local development

Requirements:

- Rust `1.93.0` recommended for CI parity
- PostgreSQL `15+`
- Companion frontend repository if you want the full PWA locally
- Optional: `diesel_cli` for local migration workflows

```bash
cp backend/.env.example .env
cargo run
```

Recommended validation commands before publishing changes:

```bash
cargo fmt --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

Optional Diesel CLI:

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run
```

## Configuration

Key variables in `backend/.env.example` and `.env.production.example`:

- `HOST`, `PORT`
- `DATABASE_URL`
- `JWT_SECRET`, `JWT_EXPIRATION`
- `MFA_ISSUER`, `MFA_ENCRYPTION_KEY`
- `MIRTH_WEBHOOK_SECRET`, `MIRTH_ALLOWED_IP`, `HL7_INTERNAL_PORT`
- `CORS_ALLOWED_ORIGINS`
- `LOG_FORMAT`, `RUST_LOG`
- `RATE_LIMIT_LOGIN_PER_IP`
- `RATE_LIMIT_MFA_PER_ACCOUNT`
- `RATE_LIMIT_API_PER_USER`
- `AUDIT_RETENTION_DAYS`
- `CDA_MIRTH_ENDPOINT`
- `PUBLIC_DOMAIN`, `ACME_EMAIL`
- `BACKUP_DIR`, `BACKUP_RETENTION_DAYS`, `S3_BACKUP_URI`

All values in `.env.example` and `.env.production.example` are placeholders only.

## Public HTTP surface

Canonical public routes are documented in [docs/API.md](docs/API.md).

Public route families currently include:

- authentication and MFA: `/api/auth/*`
- tickets and mission handling: `/api/tickets*`
- porter operations: `/api/porters*`
- patient search: `/api/patients`
- service lookup: `/api/services`
- notifications and user messaging: `/api/notifications*`
- user administration and GDPR export/delete flows: `/api/users*`
- operational historical reporting: `/api/reports/operations`
- referentials and priority rules: `/api/referentials/*`, `/api/priority-rules*`

Legacy `/api/v1/*` paths currently redirect to `/api/*`. New clients should use `/api/*`.

## Private integrations

This public documentation intentionally omits private hospital integration routes.

- Inbound hospital updates are handled through private integration channels
- Outbound CDA reporting is handled through private reporting channels
- Internal integration endpoints are not part of the public API contract

## Security and data handling

- Demo and seed content must stay synthetic only
- No production secrets are committed in this repository
- MFA is available on the public auth flow
- Production compose is designed for reverse-proxy TLS termination, not direct Internet exposure of Actix
- Metrics endpoints should be scraped from trusted infrastructure only
- Reporting archives are restricted to supervisory roles through authenticated API access
- Backup artifacts must be stored on isolated storage and restore-tested regularly
- Locked Cargo validation is enforced in CI to prevent silent lockfile drift
- Public docs must not expose private/internal integration endpoints

## Roadmap

- SSO federation (OIDC/SAML)
- Restore testing and disaster-recovery drills for PostgreSQL backups
- Expanded public API stabilization and companion frontend release alignment
- PWA/mobile workflow hardening and stronger automated E2E coverage

## Contributing

See `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, and `SECURITY.md`.

## License

GPL-3.0-or-later. See `LICENSE`.

## Commercial licensing

Proprietary/commercial licensing is available for OEM and closed-source deployments.

- Email: `Couverture@ik.me`
- Alternative: open a **Commercial License Request** issue using `.github/ISSUE_TEMPLATE/commercial_license_request.yml`

## Disclaimer

Hporterly is an operational transport workflow system. Real deployments must be
validated against applicable security, privacy, safety, and organizational
requirements. This repository is not medical, legal, or regulatory advice.

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: public documentation stays limited to public routes and role-visible workflows.
  - OK SBD-09: no real patient or facility data is introduced.
  - OK SBD-22: public scope, roles, and operational constraints are documented in one auditable entry point.
- Not fully satisfiable in this file:
  - WARN SBD-08: this document cannot itself enforce TLS or encryption at rest.
    Alternative: keep transport/storage controls in runtime configuration and deployment policy.
