# Security Policy

## Supported versions

The latest `1.x` release line is the primary target for security fixes.

## Reporting a vulnerability

- Contact: `Couverture@ik.me`
- Subject suggestion: `Security report - Hporterly`
- Do not include real patient/facility data in reports.
- Provide a minimal synthetic proof-of-concept if possible.

Please include:

- Affected component/version
- Reproduction steps
- Impact assessment
- Suggested mitigations (if known)

Acknowledgement target: within 3 business days (best effort).

## Contributor security expectations

- Never commit real data or secrets.
- Sanitize logs before sharing.
- Run the sanity check script before opening a pull request.
- Keep dependencies reasonably up to date.

## Scope note

This repository is a generic operational workflow reference implementation with
synthetic demo data only. Production deployments handling health-related data
require a full security and compliance program.
