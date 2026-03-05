pub mod handlers;
pub mod models;

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file: SBD-22 (module boundary clarity), SBD-23 (explicit asset/module inventory).
- Not fully satisfiable in this file:
  - SBD-08 (TLS 1.3) cannot be enforced in a module declaration.
    Alternative: enforce TLS 1.3 at ingress/reverse-proxy and keep internal HL7 routes on loopback only.
*/
