pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod schema;
pub mod services;
pub mod utils;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
