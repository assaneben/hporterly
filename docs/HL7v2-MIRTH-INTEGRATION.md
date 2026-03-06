# HL7v2 and Mirth Integration Overview

This document describes the business and security model of the HL7v2 ingestion flow used by Hporterly.

It is intentionally limited to public-safe information.
It does not publish private routes, internal ports, secrets, or operational deployment details.

## Scope

Hporterly can consume patient and transport events originating from a hospital DPI.

The integration boundary is strict:

- the DPI emits HL7v2
- Mirth Connect receives the raw HL7v2 message
- Mirth transforms HL7v2 into normalized JSON
- Hporterly consumes only the normalized JSON through private integration channels

Hporterly does **not** parse raw HL7v2 directly.

## Supported business events

The current integration covers these operational event families:

- patient admission
- patient transfer between locations
- patient discharge
- transport order creation

Typical healthcare message families behind these events include:

- `ADT^A01` for admission
- `ADT^A02` for transfer
- `ADT^A03` for discharge
- `ORM^O01` for transport order creation

This public document describes their business meaning only.
It does not publish the private route contract used between Mirth and Hporterly.

## End-to-end flow

### 1. HL7v2 emission from the DPI

The hospital DPI remains the source system for patient identity and hospitalization context.
It emits HL7v2 messages containing the patient and transport-related operational context.

### 2. Reception and transformation by Mirth Connect

Mirth Connect is the mandatory middleware for HL7v2 ingestion.

Its responsibilities are:

- receive raw HL7v2
- apply mapping and normalization rules
- convert message content into a constrained JSON payload
- call Hporterly through a private authenticated integration channel

### 3. Reception by Hporterly

Hporterly receives the normalized JSON payload and applies application-level controls:

- payload parsing
- field normalization
- INS validation
- enum validation for status, transport type, and priority
- fail-secure rejection on malformed or unauthorized input

### 4. Operational persistence

Depending on the event type, Hporterly can:

- create or update a patient record
- update patient location
- mark a patient as discharged
- close active transports on discharge
- create a transport request from the normalized order payload

### 5. Audit

Every accepted business event is designed to produce an audit-oriented trace.
This supports operational supervision and post-event review without exposing raw clinical content in logs.

## Normalized patient payload

The patient-oriented normalized payload includes business fields such as:

- `ins`
- `nom`
- `prenom`
- `date_naissance`
- `sexe`
- `unite_actuelle`
- `chambre`
- `precautions`
- `statut_hospitalisation`

Expected identity rules:

- `ins` is mandatory
- `ins` must contain exactly 19 digits
- `ins` must pass Luhn validation before insertion or update
- French INS identifier system remains associated with `urn:oid:1.2.250.1.213.1.4.8`

Illustrative placeholder only:

```json
{
  "ins": "<INS_19_CHIFFRES>",
  "nom": "<NOM_SYNTHESE>",
  "prenom": "<PRENOM_SYNTHESE>",
  "date_naissance": "1980-01-01",
  "sexe": "F",
  "unite_actuelle": "CARDIO-1",
  "chambre": "101A",
  "precautions": ["contact"],
  "statut_hospitalisation": "hospitalise"
}
```

## Normalized transport order payload

The transport-order normalized payload includes business fields such as:

- `ins`
- `type_transport`
- `destination_code`
- `priorite`
- `prescripteur_id`
- `commentaire`
- `precautions_transport`

Allowed operational values currently include:

- `type_transport`: `marche`, `fauteuil`, `civiere`, `lit`
- `priorite`: `routine`, `urgent`, `asap`, `stat`

Illustrative placeholder only:

```json
{
  "ins": "<INS_19_CHIFFRES>",
  "type_transport": "fauteuil",
  "destination_code": "RADIO-2",
  "priorite": "urgent",
  "prescripteur_id": "USR_SYNTHETIC",
  "commentaire": "Transport operational note",
  "precautions_transport": ["oxygen"]
}
```

## Identity vigilance rules

INS remains the persistent patient identity anchor for this integration.

Operational expectations:

- malformed INS is rejected
- checksum-invalid INS is rejected
- patient identity is not inferred from name/date of birth alone
- downstream transport workflows must still enforce field confirmation rules where required

The dedicated user-facing confirmation workflow remains documented separately in:

- [INS Confirmation Flow](ins-confirmation-flow.md)

## Security model

This integration follows a defense-in-depth model.

Key controls:

- raw HL7v2 stays outside Hporterly
- only normalized JSON crosses into Hporterly
- private integration calls are authenticated
- source restriction is enforced for the private integration boundary
- unauthorized or malformed requests are rejected by default
- audit traces are generated for accepted operational events
- internal integration routes are excluded from public documentation

## Compliance boundary

Hporterly is positioned here as:

- logistics-only workflow software
- `MDR Class I`
- no automated clinical decision engine
- no clinical scoring derived from HL7 data

Precautions coming from upstream systems may be displayed for operations.
They are not transformed into clinical recommendations by Hporterly.

## Explicit exclusions from public documentation

The following are intentionally not published here:

- internal route paths
- internal ports
- signature headers or secret material
- infrastructure topology
- private payload signing procedure
- production deployment values

Those details belong to private deployment and operations documentation, not to the public repository contract.

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: the document explains the HL7v2 business flow without exposing private/internal routes.
  - OK SBD-09: all examples use synthetic placeholders only; no real INS or patient data is present.
  - OK SBD-21: fail-secure rejection and deny-by-default expectations are documented explicitly.
  - OK SBD-22: the integration boundary and trust assumptions are centralized in one auditable document.
- Not fully satisfiable in this file:
  - WARN SBD-08: this document cannot itself enforce authenticated transport, TLS, or encryption at rest.
    Alternative: keep those controls in runtime configuration, secret management, and deployment policy.
