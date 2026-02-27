import hashlib
import os
import zipfile
from collections import Counter
from datetime import datetime
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INVENTAIRES_DIR = ROOT / "DOSSIER_REPRISE_DEV" / "inventaires"
INVENTAIRES_DIR.mkdir(parents=True, exist_ok=True)
MANIFEST_PATH = INVENTAIRES_DIR / "PACK_PRET_A_PARTAGER_MANIFEST.txt"

EXCLUDED_FILE_NAMES = {"nul", "build_check.txt"}
EXCLUDED_DIR_NAMES_GLOBAL = {".git", "__pycache__", ".pytest_cache", ".mypy_cache", ".ruff_cache", ".cache"}
MAX_EXCLUSION_DETAILS = 200


def rel_posix(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def should_exclude_dir(path: Path):
    rel = rel_posix(path)
    name = path.name

    if name in EXCLUDED_DIR_NAMES_GLOBAL:
        return f"tool/cache dir: {name}"
    if rel == "archive 2026" or rel.startswith("archive 2026/"):
        return "archive folder"
    if rel == "backend/target" or rel.startswith("backend/target/"):
        return "build artifact: backend/target"
    if "/node_modules" in f"/{rel}" or rel.endswith("/node_modules") or rel == "node_modules":
        return "dependency cache: node_modules"
    return None


def should_exclude_file(path: Path):
    rel = rel_posix(path)
    name = path.name

    if name in EXCLUDED_FILE_NAMES:
        return f"excluded file name: {name}"
    if rel.endswith(".zip"):
        return "existing zip archive excluded"
    if name == ".env":
        return "secret file: .env"
    if name.startswith(".env.") and not name.endswith(".example"):
        return "secret/local env file"
    if rel.startswith("backend/target/"):
        return "build artifact: backend/target"
    if "/node_modules/" in f"/{rel}":
        return "dependency cache: node_modules"
    if "/__pycache__/" in f"/{rel}" or rel.endswith(".pyc") or rel.endswith(".pyo"):
        return "python cache artifact"
    return None


def walk_files():
    included = []
    exclusion_reasons = Counter()
    exclusion_details = []

    for current_root, dir_names, file_names in os.walk(ROOT):
        root_path = Path(current_root)

        kept_dirs = []
        for d in dir_names:
            d_path = root_path / d
            reason = should_exclude_dir(d_path)
            if reason:
                exclusion_reasons[reason] += 1
                if len(exclusion_details) < MAX_EXCLUSION_DETAILS:
                    exclusion_details.append(f"- {rel_posix(d_path)} :: {reason}")
            else:
                kept_dirs.append(d)
        dir_names[:] = kept_dirs

        for f in file_names:
            f_path = root_path / f
            reason = should_exclude_file(f_path)
            if reason:
                exclusion_reasons[reason] += 1
                if len(exclusion_details) < MAX_EXCLUSION_DETAILS:
                    exclusion_details.append(f"- {rel_posix(f_path)} :: {reason}")
                continue
            included.append(f_path)

    included.sort(key=lambda p: rel_posix(p))
    return included, exclusion_reasons, exclusion_details


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write_manifest(zip_path: Path, zip_sha256: str, included_files, exclusion_reasons, exclusion_details):
    top_levels = sorted({rel_posix(p).split("/")[0] for p in included_files})
    reason_lines = [f"- {reason}: {count}" for reason, count in sorted(exclusion_reasons.items())]

    lines = [
        "# Pack pret a partager - manifeste",
        "",
        f"Fichier ZIP: {zip_path.name}",
        f"Date generation: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
        f"Taille ZIP (octets): {zip_path.stat().st_size}",
        f"SHA256: {zip_sha256}",
        "",
        "## Inclus",
        f"- Fichiers inclus: {len(included_files)}",
        "",
        "## Exclusions (resume)",
    ]

    if reason_lines:
        lines.extend(reason_lines)
    else:
        lines.append("- Aucune exclusion")

    lines.extend(["", f"## Exclusions detaillees (premieres {MAX_EXCLUSION_DETAILS})"])
    if exclusion_details:
        lines.extend(exclusion_details)
    else:
        lines.append("- Aucune")

    lines.extend(["", "## Top-level inclus (deduit)"])
    lines.extend([f"- {name}" for name in top_levels])
    lines.append("")

    MANIFEST_PATH.write_text("\n".join(lines), encoding="utf-8")


def main():
    included_files, exclusion_reasons, exclusion_details = walk_files()
    ts = datetime.now().strftime("%Y%m%d_%H%M%S")
    zip_path = ROOT / f"HPorterly_pret_a_partager_dev_{ts}.zip"

    with zipfile.ZipFile(zip_path, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        for p in included_files:
            zf.write(p, arcname=rel_posix(p))

    zip_sha256 = sha256_file(zip_path)
    write_manifest(zip_path, zip_sha256, included_files, exclusion_reasons, exclusion_details)

    print(f"ZIP={zip_path}")
    print(f"SIZE={zip_path.stat().st_size}")
    print(f"SHA256={zip_sha256}")
    print(f"INCLUDED_FILES={len(included_files)}")


if __name__ == "__main__":
    main()
