# Backend Architecture

## Components

- `src/main.rs`: server bootstrap (Actix, CORS, rate limiting, routes)
- `src/config.rs`: environment configuration
- `src/auth.rs`: JWT + Argon2 helpers
- `src/db.rs`: Diesel pool + embedded migrations
- `src/handlers/*`: HTTP and WebSocket handlers
- `src/models.rs`: generic workflow DTOs and enums
- `migrations/`: Diesel SQL migrations (15 folders)
- `scripts/`: synthetic demo seed/data generation and safety checks

## Runtime behavior

- Starts with an in-memory queue for demo usability
- Attempts DB migration execution when `DATABASE_URL` is reachable
- Exposes WebSocket heartbeats for client realtime wiring
- Enforces IP-based rate limiting at 500 req/s with burst 1000
