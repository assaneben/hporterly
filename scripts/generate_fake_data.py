#!/usr/bin/env python3
"""Generate synthetic demo datasets.

Output files include the required notice: DEMO DATA — SYNTHETIC / NOT REAL.
"""
from __future__ import annotations

import json
import random
import uuid
from datetime import datetime, timedelta, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = ROOT / "examples"
random.seed(42)

LOCATIONS = ["Unit A", "Unit B", "Unit C", "Imaging", "Lab", "OR-1", "Recovery"]
PRIORITIES = ["routine", "high", "urgent"]
STATUSES = ["queued", "assigned", "in_progress", "completed"]
DISPATCHERS = ["dispatcher.alpha", "dispatcher.bravo", "dispatcher.charlie"]
PORTERS = ["porter-01", "porter-02", "porter-03"]
FAKE_NAMES = ["Nora Valen", "Ilan Mercer", "Sara Quill", "Mina Hart", "Theo Larkin"]


def ts(offset_min: int) -> str:
    base = datetime(2026, 2, 24, 8, 0, tzinfo=timezone.utc)
    return (base + timedelta(minutes=offset_min)).isoformat().replace("+00:00", "Z")


def minimal_row(i: int) -> dict:
    origin = random.choice(LOCATIONS)
    destination = random.choice([x for x in LOCATIONS if x != origin])
    requested = random.randint(0, 180)
    updated = requested + random.randint(0, 25)
    return {
        "request_id": str(uuid.uuid4()),
        "origin_location": origin,
        "destination_location": destination,
        "priority": random.choice(PRIORITIES),
        "status": random.choice(STATUSES),
        "requested_at": ts(requested),
        "updated_at": ts(updated),
        "requested_by": random.choice(DISPATCHERS),
        "assigned_to": random.choice(PORTERS),
    }


def full_row(i: int) -> dict:
    row = minimal_row(i)
    row.update(
        {
            "patient_name": random.choice(FAKE_NAMES),
            "patient_id": f"PT-FAKE-{1000 + i}",
            "dob": f"19{random.randint(70, 99)}-{random.randint(1, 12):02d}-{random.randint(1, 28):02d}",
            "room": f"R-{random.choice([101, 102, 202, 315, 410])}",
            "notes": "Synthetic demo note generated for testing only.",
        }
    )
    return row


def write_payload(name: str, transfers: list[dict]) -> None:
    payload = {
        "demo_data_notice": "DEMO DATA — SYNTHETIC / NOT REAL",
        "dataset": name,
        "version": "1.0.0",
        "transfers": transfers,
    }
    (EXAMPLES / f"{name}.generated.json").write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    EXAMPLES.mkdir(parents=True, exist_ok=True)
    write_payload("minimal_demo", [minimal_row(i) for i in range(5)])
    write_payload("full_demo_with_fake_patient_fields", [full_row(i) for i in range(5)])
    print("Generated synthetic demo datasets in examples/ (generated variants).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
