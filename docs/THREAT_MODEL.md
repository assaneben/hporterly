# Lightweight Threat Model (Generic)

## Assets

- Transfer workflow events and statuses
- User identities and roles
- Deployment credentials and secrets
- Audit and operational logs

## Primary threats

- Unauthorized access due to weak credentials or missing access controls
- Secret leakage via commits, logs, screenshots, or issues
- Injection and input-validation flaws on API endpoints
- Excessive request volume / abuse
- Insecure production defaults (TLS, backups, logging)

## Mitigations present in this repo

- Argon2 password hashing helpers
- JWT signing
- CORS + rate limiting middleware
- CI sanity checks for forbidden aliases and common secret-like patterns
- Synthetic-only demo-data policy and documentation guardrails

## Production gaps (intentional)

- Full RBAC and persistent audit trails
- SSO / MFA integration
- Secret vault integration and automated rotation
- Infrastructure hardening, monitoring, compliance controls
