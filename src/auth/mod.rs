pub mod mfa;
pub mod rbac;

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file: SBD-22, SBD-23 (security module inventory and explicit boundaries).
- Not fully satisfiable in this file:
  - SBD-08 (TLS 1.3 / at-rest policy) is deployment and persistence layer dependent.
    Alternative: enforce through ingress policy + database/volume encryption governance.
*/
