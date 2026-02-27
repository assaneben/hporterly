# API Overview (Generic / Demo-safe)

Base path: `/api/v1`

## Endpoints

- `GET /healthz` - liveness/health summary
- `POST /api/v1/auth/login` - demo login, returns JWT
- `GET /api/v1/transfers` - list transfer requests (in-memory demo store)
- `POST /api/v1/transfers` - create a transfer request
- `GET /ws` - WebSocket endpoint (heartbeat + echo)

## Notes

- Core API models are generic and intentionally omit patient fields.
- Fake patient fields exist only in demo datasets under `examples/` and demo scripts.
- JWT is included for integration prototyping; production deployments require hardened identity and key management.
