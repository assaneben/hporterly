# Changelog

This file tracks notable published changes for Hporterly.

It is based on actual commits present in the published repository history.
Private/internal integration endpoints remain intentionally excluded from public documentation.

## [Unreleased]

- No unpublished public documentation changes are currently pending.

## [2026-03-06]

### Documentation

- Refreshed public user documentation to align `README.md`, `docs/API.md`, and `docs/USER-WORKFLOWS.md` with the current public API surface, MFA flow, user roles, and operational scope.
- Added a dedicated changelog to make branch evolution and published updates easier to track.

### CI and governance

- Enforced locked Cargo validation in CI with:
  - `cargo check --locked --all-targets --all-features`
  - `cargo clippy --locked --all-targets --all-features -- -D warnings`
  - `cargo test --locked --all-targets --all-features`
- Adjusted CI/readiness behavior to tolerate local Windows host-policy execution blocks while keeping Linux CI authoritative.
- Tightened PR readiness governance and evidence expectations for future deliveries.

### Dependency management

- Refreshed `Cargo.lock` to align dependency resolution with the manifest used by the published backend.

## [2026-03-05]

### Build and dependency stability

- Synced the published Rust backend with the dev backend to resolve compilation and dependency mismatches.
- Added the required Diesel `64-column-tables` feature for the project's join/table surface.
- Reformatted the published codebase with the stable Rust toolchain.
- Brought `cargo clippy -- -D warnings` back to green on the published branch.

### Governance and repository hygiene

- Added the PR delivery protocol, PR readiness workflow, and local readiness script.
- Removed test artifacts from the published repository and hardened ignore rules.

## [2026-03-02]

### Business logic

- Published the `v1.1` full business logic baseline:
  - models
  - services
  - handlers
  - migrations

### CI fixes

- Applied formatting fixes and removed conflicting legacy files to restore CI consistency.

## [2026-02-28]

### Product positioning and status alignment

- Updated repository descriptions in French and English.
- Repositioned the repository as a SaaS coordination solution.
- Aligned transfer statuses with business rules.

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-05: the changelog documents only public, publish-safe evolution and excludes private/internal routes.
  - OK SBD-09: no real patient, INS, facility, or secret data is introduced.
  - OK SBD-22: published evolution is centralized in an auditable, dated history.
- Not fully satisfiable in this file:
  - WARN SBD-08: a changelog cannot itself enforce transport encryption or encryption at rest.
    Alternative: keep those controls in runtime configuration, CI, and deployment policy.
