# CDA R2 Overview

This document describes the CDA R2 transport-reporting flow implemented in Hporterly.

It is limited to public-safe information.
It does not expose private reporting endpoints, internal ports, or deployment secrets.

## Scope

Hporterly can generate a CDA R2 document when a transport is completed.

The purpose of this document is to:

- preserve an interoperable transport summary
- support downstream document ingestion by hospital integration middleware
- avoid losing completed transport reports when downstream delivery is temporarily unavailable

This reporting flow is logistics-oriented and does not introduce automated clinical decisions.

## Trigger condition

The CDA generation flow is triggered when a transport reaches the completed state.

In the currently published implementation:

- a completed transport triggers report generation
- the report is built from transport, patient, and operational context
- the generated XML is sent to a private downstream reporting channel
- if downstream delivery fails, the document is persisted for retry

## Report content model

The implemented report model includes operational fields such as:

- transport identifier
- patient INS
- patient last name and first name
- patient date of birth
- patient sex
- origin name
- destination name
- brancardier name
- prescriber name when available
- transport start time
- transport end time
- precautions
- INS confirmation status

These fields are used to populate the CDA document body and header structure.

## CDA structure

The generated XML follows a CDA R2-style `ClinicalDocument` structure with elements such as:

- document identifier
- `effectiveTime`
- `recordTarget`
- patient identity block
- `author`
- structured transport summary section

Notable implemented semantics:

- the document `id` is tied to the transport identifier
- the patient identity block includes the INS under the French INS OID `1.2.250.1.213.1.4.8`
- the author is the HPorterly system, not a human operator
- transport details are emitted in a structured summary section

## Operational data sources

The generated report is assembled from:

- the completed transport record
- patient identity context already present in Hporterly
- origin and destination transport data
- porter identity when available
- requester identity when available
- operational precautions and INS confirmation information

If identity data needed for a compliant report is missing, the flow is expected to fail safely rather than emit an incomplete report silently.

## Delivery model

The generated CDA document is sent through a private downstream integration channel.

This public document intentionally does not publish:

- the private endpoint path
- the port
- the delivery secret model
- the private middleware topology

## Reliability model

The implementation includes a persistence-based retry model.

If downstream delivery fails:

- the CDA document is not discarded
- it is queued in a persistence layer dedicated to pending CDA delivery
- retries are attempted later

This behavior is designed to prevent report loss on transient downstream failure.

## Identity and safety rules

The CDA flow stays aligned with transport identity controls:

- INS is the patient identity anchor used in the report
- INS confirmation state is carried into the document content
- the report remains an operational account of transport execution
- the report is not a clinical recommendation artifact

## Compliance boundary

Within Hporterly, CDA generation remains:

- operational and logistics-oriented
- tied to transport execution traceability
- outside automated clinical decision-making
- compatible with the `MDR Class I` positioning of the application

## Explicit exclusions from public documentation

The following are intentionally excluded:

- private reporting routes
- private receiving middleware details
- secret headers or credentials
- internal retry scheduling values
- infrastructure-specific transport security configuration

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: the document explains the CDA reporting flow without exposing private downstream routes.
  - OK SBD-09: no real patient, INS, or secret values are included.
  - OK SBD-21: never-drop and fail-safe persistence behavior is documented explicitly.
  - OK SBD-22: the report trigger, content model, and delivery reliability assumptions are centralized in one auditable document.
- Not fully satisfiable in this file:
  - WARN SBD-08: this document cannot itself enforce authenticated downstream transport, TLS, or encryption at rest.
    Alternative: keep transport security, secret handling, and storage controls in runtime configuration and deployment policy.
