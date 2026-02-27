#!/usr/bin/env python3
"""Generate synthetic demo users seed SQL (no real identities)."""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "examples" / "seed_demo_users.sql"

SQL = """-- DEMO DATA — SYNTHETIC / NOT REAL
INSERT INTO users (id, username, display_name, role, password_hash) VALUES
('11111111-1111-1111-1111-111111111111', 'dispatcher.alpha', 'Dispatcher Alpha', 'dispatcher', '$argon2id$v=19$m=19456,t=2,p=1$demo$demo'),
('22222222-2222-2222-2222-222222222222', 'porter-01', 'Porter 01', 'porter', '$argon2id$v=19$m=19456,t=2,p=1$demo$demo'),
('33333333-3333-3333-3333-333333333333', 'supervisor.demo', 'Supervisor Demo', 'supervisor', '$argon2id$v=19$m=19456,t=2,p=1$demo$demo')
ON CONFLICT (id) DO NOTHING;
"""


def main() -> int:
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(SQL, encoding="utf-8")
    print(f"Wrote {OUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
