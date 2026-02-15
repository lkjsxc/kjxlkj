#!/usr/bin/env bash
# Backup and restore drill automation per /docs/spec/technical/operations.md.
# Usage: scripts/backup-restore-drill.sh [db_name] [backup_dir]
# Requires: psql, pg_dump, pg_restore (or psql for SQL format).
set -euo pipefail

DB_NAME="${1:-kjxlkj}"
BACKUP_DIR="${2:-/tmp/kjxlkj-drill}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="${BACKUP_DIR}/drill_${TIMESTAMP}.sql"
RESTORE_DB="${DB_NAME}_drill_restore"

echo "=== Backup/Restore Drill ==="
echo "Source DB:   ${DB_NAME}"
echo "Backup dir:  ${BACKUP_DIR}"
echo "Restore DB:  ${RESTORE_DB}"
echo ""

mkdir -p "${BACKUP_DIR}"

# Step 1: Create SQL backup
echo "[1/5] Creating SQL backup..."
pg_dump --format=custom --no-owner "${DB_NAME}" > "${BACKUP_FILE}"
echo "  Backup: ${BACKUP_FILE} ($(du -h "${BACKUP_FILE}" | cut -f1))"

# Step 2: Drop restore DB if exists
echo "[2/5] Preparing restore target..."
psql -c "DROP DATABASE IF EXISTS ${RESTORE_DB};" postgres 2>/dev/null || true
psql -c "CREATE DATABASE ${RESTORE_DB};" postgres

# Step 3: Restore backup into clean DB
echo "[3/5] Restoring backup..."
pg_restore --no-owner --dbname="${RESTORE_DB}" "${BACKUP_FILE}"

# Step 4: Verify parity (row counts)
echo "[4/5] Verifying parity..."
TABLES=$(psql -At -c "SELECT tablename FROM pg_tables WHERE schemaname='public'" "${DB_NAME}")
PARITY_OK=true
for tbl in ${TABLES}; do
  ORIG=$(psql -At -c "SELECT count(*) FROM ${tbl}" "${DB_NAME}")
  REST=$(psql -At -c "SELECT count(*) FROM ${tbl}" "${RESTORE_DB}")
  if [ "${ORIG}" != "${REST}" ]; then
    echo "  MISMATCH: ${tbl} (${ORIG} vs ${REST})"
    PARITY_OK=false
  fi
done

# Step 5: Cleanup
echo "[5/5] Cleanup..."
psql -c "DROP DATABASE IF EXISTS ${RESTORE_DB};" postgres 2>/dev/null || true

if [ "${PARITY_OK}" = true ]; then
  echo ""
  echo "=== DRILL PASS: backup/restore parity verified ==="
  exit 0
else
  echo ""
  echo "=== DRILL FAIL: row count mismatch detected ==="
  exit 1
fi
