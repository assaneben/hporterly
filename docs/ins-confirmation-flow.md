# INS Confirmation Flow (Brancardier Mobile)

## Scope
This document defines the mandatory identitovigilance flow for transporter missions.
Classification: MDR Class I (logistics only, no clinical decision automation).

## Preconditions
- Mission is received by the brancardier client through WebSocket push.
- Mission detail screen displays at minimum:
  - patient `nom`
  - patient `prenom`
  - patient `date_naissance`
  - `chambre_depart`
  - `destination`
  - `precautions`

## Mandatory INS Confirmation Step
1. Before mission start, brancardier must scan or manually enter patient INS from bracelet.
2. Mobile client calls:
   - `POST /fhir/r4/Task/{id}/confirm-ins`
   - Body: `{ "ins_scanne": "<INS_19_CHIFFRES>" }`
3. Backend performs constant-time comparison between scanned INS and expected INS.

## Expected Outcomes
- Match:
  - API returns success (`200 OK`).
  - Transport start is authorized.
  - UI state is green (`INS_CONFIRME`).
- Mismatch:
  - API returns `409 Conflict`.
  - Transport start remains blocked.
  - UI state is red (`INS_MISMATCH`).
  - Regulator receives real-time alert through WebSocket broadcast.

## Security + Audit Requirements
- Every confirmation attempt must be logged in `audit_logs`.
- Successful match action: `INS_CONFIRME`.
- Mismatch action: `INS_MISMATCH` with task id + source metadata.
- Rate limit must apply to confirmation attempts (account-scoped and IP-scoped).

## API Contract Snapshot
Request:
```http
POST /fhir/r4/Task/{id}/confirm-ins
Authorization: Bearer <jwt>
Content-Type: application/json

{ "ins_scanne": "<INS_19_CHIFFRES>" }
```

Success response:
```json
{
  "status": "ok",
  "ins_confirme": true,
  "task_status": "accepted"
}
```

Mismatch response:
```json
{
  "error": "INS_MISMATCH",
  "message": "INS patient mismatch; transport blocked"
}
```

SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-05: explicit access gate before mission start (`confirm-ins` mandatory).
  - SBD-08: constant-time INS comparison requirement documented.
  - SBD-10: audit actions mandated for match/mismatch.
  - SBD-11: rate limiting requirement documented.
  - SBD-21: fail-secure behavior documented (block on mismatch).
- Not fully satisfiable in this file:
  - Runtime enforcement cannot be guaranteed by documentation alone.
  - Alternative: enforce checks in handlers/middleware and add integration tests in CI.
