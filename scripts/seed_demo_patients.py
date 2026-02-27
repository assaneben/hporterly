#!/usr/bin/env python3
"""Generate fake patient-linked demo records for testing only."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "examples" / "seed_demo_patients.json"

PAYLOAD = {
    "demo_data_notice": "DEMO DATA — SYNTHETIC / NOT REAL",
    "dataset": "seed_demo_patients",
    "records": [
        {
            "patient_name": "Nora Valen",
            "patient_id": "PT-FAKE-1001",
            "dob": "1982-04-13",
            "room": "R-101",
            "notes": "Synthetic demo record for integration testing only.",
        },
        {
            "patient_name": "Ilan Mercer",
            "patient_id": "PT-FAKE-1002",
            "dob": "1975-11-02",
            "room": "R-202",
            "notes": "Synthetic demo record for integration testing only.",
        },
    ],
}


def main() -> int:
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(PAYLOAD, indent=2) + "\n", encoding="utf-8")
    print(f"Wrote {OUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
