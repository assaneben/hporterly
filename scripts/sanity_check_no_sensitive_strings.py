#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKIP_DIRS = {"target", "node_modules", ".venv", "venv", "dist", "build", "coverage"}

forbidden_parts = [
    ("Beau", " Soleil"),
    ("A", "ESIO"),
    ("Languedoc", " Mutualit" + "é"),
    ("ES", "PIC"),
]
secret_parts = [
    ("BEGIN", " PRIVATE KEY"),
    ("AWS", "_SECRET"),
    ("TOK", "EN="),
    ("PASS", "WORD="),
]
EXACT_PATTERNS = [a + b for a, b in forbidden_parts + secret_parts]
REGEX_PATTERNS = [
    re.compile(r"AKIA[0-9A-Z]{16}"),
    re.compile(r"-----BEGIN [A-Z ]+PRIVATE KEY-----"),
    re.compile(r"(?i)bearer\s+[A-Za-z0-9\-_=]{20,}"),
    re.compile(r"(?i)(api[_-]?key|secret[_-]?key)\s*[:=]\s*['\"][A-Za-z0-9_\-+/=]{16,}"),
]


def iter_files(root: Path):
    for path in root.rglob("*"):
        if not path.is_file():
            continue
        if any(part in SKIP_DIRS for part in path.parts):
            continue
        yield path


def main() -> int:
    failures: list[str] = []
    for path in iter_files(ROOT):
        try:
            content = path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        lower = content.lower()
        for token in EXACT_PATTERNS:
            if token.lower() in lower:
                failures.append(f"{path.relative_to(ROOT)} :: exact pattern: {token}")
        for pattern in REGEX_PATTERNS:
            if pattern.search(content):
                failures.append(f"{path.relative_to(ROOT)} :: regex pattern: {pattern.pattern}")

    if failures:
        print("Sanity check failed. Forbidden strings or likely secrets detected:")
        for item in failures:
            print(f" - {item}")
        return 1

    print("Sanity check passed: no forbidden strings or obvious secrets detected.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
