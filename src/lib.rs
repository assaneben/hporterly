pub mod auth;
pub mod cda;
pub mod config;
pub mod handlers;
pub mod hl7;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod schema;
pub mod services;
pub mod utils;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file: SBD-22, SBD-23 (explicitly declaring security-relevant modules).
- Not fully satisfiable in this file:
  - SBD-08 (TLS 1.3) cannot be enforced from a module export file.
    Alternative: enforce TLS policy at server/bootstrap and ingress layers.
*/
