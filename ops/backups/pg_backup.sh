#!/bin/sh
set -eu

: "${POSTGRES_DB:?POSTGRES_DB is required}"
: "${POSTGRES_USER:?POSTGRES_USER is required}"
: "${POSTGRES_PASSWORD:?POSTGRES_PASSWORD is required}"

BACKUP_DIR="${BACKUP_DIR:-/backups}"
PGHOST="${PGHOST:-db}"
PGPORT="${PGPORT:-5432}"
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-14}"
TIMESTAMP="$(date -u +"%Y%m%dT%H%M%SZ")"
OUTPUT_FILE="${BACKUP_DIR}/${POSTGRES_DB}_${TIMESTAMP}.dump.gz"
TMP_FILE="$(mktemp)"

mkdir -p "${BACKUP_DIR}"
export PGPASSWORD="${POSTGRES_PASSWORD}"

pg_dump \
  --format=custom \
  --clean \
  --if-exists \
  --no-owner \
  --host="${PGHOST}" \
  --port="${PGPORT}" \
  --username="${POSTGRES_USER}" \
  "${POSTGRES_DB}" > "${TMP_FILE}"

gzip -c "${TMP_FILE}" > "${OUTPUT_FILE}"
rm -f "${TMP_FILE}"

find "${BACKUP_DIR}" -type f -name '*.dump.gz' -mtime "+${RETENTION_DAYS}" -delete

if [ -n "${S3_BACKUP_URI:-}" ]; then
  if command -v aws >/dev/null 2>&1; then
    aws s3 cp "${OUTPUT_FILE}" "${S3_BACKUP_URI}/$(basename "${OUTPUT_FILE}")" --only-show-errors
  else
    echo "S3_BACKUP_URI is set but aws CLI is not available in the backup container" >&2
    exit 1
  fi
fi

echo "Backup created: ${OUTPUT_FILE}"
