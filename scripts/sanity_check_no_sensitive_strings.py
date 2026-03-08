#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKIP_DIRS = {"target", "node_modules", ".venv", "venv", "dist", "build", "coverage", "dev-handoff"}
EXAMPLE_SUFFIXES = (".example", ".sample", ".template")
PLACEHOLDER_MARKERS = (
    "change_me",
    "changeme",
    "replace_with",
    "placeholder",
    "example",
    "dummy",
    "sample",
    "long_random",
    "random_secret",
    "base64_32_byte_key",
    "at_least_32_random_characters",
    "256_bit_random_secret",
)

forbidden_parts = [
    ("Beau", " Soleil"),
    ("A", "ESIO"),
    ("Languedoc", " Mutualit" + "Ã©"),
    ("ES", "PIC"),
    ("BEGIN", " PRIVATE KEY"),
    ("AWS", "_SECRET"),
]
EXACT_PATTERNS = [a + b for a, b in forbidden_parts]
REGEX_PATTERNS = [
    re.compile(r"AKIA[0-9A-Z]{16}"),
    re.compile(r"-----BEGIN [A-Z ]+PRIVATE KEY-----"),
    re.compile(r"(?i)bearer\s+[A-Za-z0-9\-_=]{20,}"),
    re.compile(r"(?i)(api[_-]?key|secret[_-]?key)\s*[:=]\s*['\"][A-Za-z0-9_\-+/=]{16,}"),
]
SENSITIVE_ASSIGNMENT_RE = re.compile(
    r"^\s*(?:export\s+)?(?P<key>[A-Z][A-Z0-9_]*)\s*=\s*(?P<value>.+?)\s*$"
)
SENSITIVE_KEY_RE = re.compile(
    r"(?i)(?:password|passwd|secret|token|api[_-]?key|private[_-]?key)"
)
URL_WITH_CREDENTIALS_RE = re.compile(
    r"(?i)^[A-Za-z][A-Za-z0-9+.-]*://(?P<user>[^:/@\s]+):(?P<password>[^@\s]+)@"
)
VARIABLE_REFERENCE_RE = re.compile(r"^\$(?:\{[^}]+\}|\([^)]+\)|[A-Za-z_][A-Za-z0-9_]*)$")


def iter_files(root: Path):
    for path in root.rglob("*"):
        if not path.is_file():
            continue
        if any(part in SKIP_DIRS for part in path.parts):
            continue
        yield path


def normalize_value(value: str) -> str:
    stripped = value.strip()
    if len(stripped) >= 2 and stripped[0] == stripped[-1] and stripped[0] in {"'", '"'}:
        stripped = stripped[1:-1].strip()
    return stripped


def is_placeholder_value(value: str) -> bool:
    lowered = normalize_value(value).lower()
    return any(marker in lowered for marker in PLACEHOLDER_MARKERS)


def is_variable_reference(value: str) -> bool:
    return bool(VARIABLE_REFERENCE_RE.fullmatch(normalize_value(value)))


def is_example_file(path: Path) -> bool:
    name = path.name.lower()
    return any(name.endswith(suffix) for suffix in EXAMPLE_SUFFIXES)


def find_sensitive_assignments(path: Path, content: str) -> list[str]:
    failures: list[str] = []
    for lineno, line in enumerate(content.splitlines(), start=1):
        match = SENSITIVE_ASSIGNMENT_RE.match(line)
        if not match:
            continue

        key = match.group("key")
        if not SENSITIVE_KEY_RE.search(key):
            continue

        value = normalize_value(match.group("value"))
        if not value:
            continue
        if is_variable_reference(value) or is_placeholder_value(value):
            continue
        if is_example_file(path) and value.isupper():
            continue

        failures.append(f"{path.relative_to(ROOT)}:{lineno} :: suspicious assignment: {key}")

    return failures


def find_embedded_credentials(path: Path, content: str) -> list[str]:
    failures: list[str] = []
    for lineno, line in enumerate(content.splitlines(), start=1):
        match = SENSITIVE_ASSIGNMENT_RE.match(line)
        if not match:
            continue

        key = match.group("key")
        value = normalize_value(match.group("value"))
        if key.upper() not in {"DATABASE_URL", "REDIS_URL"} and not key.upper().endswith("_URL"):
            continue

        url_match = URL_WITH_CREDENTIALS_RE.match(value)
        if not url_match:
            continue

        password = url_match.group("password")
        if is_variable_reference(password) or is_placeholder_value(password):
            continue

        failures.append(f"{path.relative_to(ROOT)}:{lineno} :: embedded credentials in: {key}")

    return failures


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
        failures.extend(find_sensitive_assignments(path, content))
        failures.extend(find_embedded_credentials(path, content))
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
