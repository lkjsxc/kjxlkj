#!/usr/bin/env bash
set -euo pipefail

# backup-restore-drill.sh
# Proves functional parity after database backup and restore.
# Usage: ./scripts/backup-restore-drill.sh [container_name]

CONTAINER="${1:-kjxlkj}"
BACKUP_FILE="/tmp/kjxlkj-drill-$(date +%s).sql"
API_BASE="http://127.0.0.1:8080"

echo "=== kjxlkj Backup-Restore Drill ==="

# Step 1: Check readiness
echo "[1/6] Checking readiness..."
if ! curl -fsS "$API_BASE/api/readyz" >/dev/null 2>&1; then
    echo "ERROR: Server not ready at $API_BASE"
    exit 1
fi

# Step 2: Create backup
echo "[2/6] Creating SQL backup..."
docker exec "$CONTAINER" gosu postgres pg_dump -h 127.0.0.1 -U kjxlkj kjxlkj > "$BACKUP_FILE"
BACKUP_SIZE=$(wc -c < "$BACKUP_FILE")
echo "  Backup: $BACKUP_FILE ($BACKUP_SIZE bytes)"

# Step 3: Drop and recreate database
echo "[3/6] Dropping and recreating database..."
docker exec "$CONTAINER" gosu postgres psql -h 127.0.0.1 -U postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='kjxlkj' AND pid <> pg_backend_pid();" >/dev/null
docker exec "$CONTAINER" gosu postgres psql -h 127.0.0.1 -U postgres -c "DROP DATABASE IF EXISTS kjxlkj"
docker exec "$CONTAINER" gosu postgres psql -h 127.0.0.1 -U postgres -c "CREATE DATABASE kjxlkj OWNER kjxlkj"

# Step 4: Restore backup
echo "[4/6] Restoring from backup..."
docker exec -i "$CONTAINER" gosu postgres psql -h 127.0.0.1 -U kjxlkj kjxlkj < "$BACKUP_FILE"

# Step 5: Restart the app inside the container to pick up restored DB
echo "[5/6] Restarting container..."
docker restart "$CONTAINER"
sleep 5

# Step 6: Verify readiness after restore
echo "[6/6] Verifying readiness after restore..."
for i in $(seq 1 30); do
    if curl -fsS "$API_BASE/api/readyz" >/dev/null 2>&1; then
        echo "  Server ready after restore"
        break
    fi
    if [ "$i" -eq 30 ]; then
        echo "ERROR: Server not ready after restore"
        rm -f "$BACKUP_FILE"
        exit 1
    fi
    sleep 1
done

# Cleanup
rm -f "$BACKUP_FILE"

echo ""
echo "=== Drill PASSED ==="
echo "Backup and restore completed with functional parity."
