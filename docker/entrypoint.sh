#!/bin/bash
set -e

# Per /docs/spec/architecture/deployment.md — process supervision contract:
# 1. init PG data dir if missing
# 2. start PG and wait for readiness
# 3. run SQL migrations (handled by app on startup)
# 4. start application server
# 5. forward signals and stop both cleanly

PGDATA="${POSTGRES_DATA_DIR:-/var/lib/postgresql/data}"

# Step 1: Init PostgreSQL data directory if missing
if [ ! -f "$PGDATA/PG_VERSION" ]; then
    echo "[entrypoint] Initializing PostgreSQL data directory..."
    chown -R postgres:postgres "$PGDATA"
    su - postgres -c "initdb -D $PGDATA"
    # Configure pg_hba for local trust + localhost password
    echo "local all all trust" > "$PGDATA/pg_hba.conf"
    echo "host all all 127.0.0.1/32 trust" >> "$PGDATA/pg_hba.conf"
    echo "host all all ::1/128 trust" >> "$PGDATA/pg_hba.conf"
fi

# Ensure ownership
chown -R postgres:postgres "$PGDATA"

# Step 2: Start PostgreSQL
echo "[entrypoint] Starting PostgreSQL..."
su - postgres -c "pg_ctl -D $PGDATA -o '-k /tmp' start -w"

# Wait for PG readiness
for i in $(seq 1 30); do
    if su - postgres -c "pg_isready -h 127.0.0.1" > /dev/null 2>&1; then
        break
    fi
    sleep 1
done

# Create database and user if not exists
su - postgres -c "psql -h 127.0.0.1 -c \"SELECT 1 FROM pg_roles WHERE rolname='kjxlkj'\"" | grep -q 1 || \
    su - postgres -c "psql -h 127.0.0.1 -c \"CREATE USER kjxlkj WITH PASSWORD 'kjxlkj'\""
su - postgres -c "psql -h 127.0.0.1 -lqt" | grep -q kjxlkj || \
    su - postgres -c "psql -h 127.0.0.1 -c \"CREATE DATABASE kjxlkj OWNER kjxlkj\""

echo "[entrypoint] PostgreSQL ready"

# Step 5: Signal handling — stop both processes on SIGTERM/SIGINT
cleanup() {
    echo "[entrypoint] Shutting down..."
    kill "$APP_PID" 2>/dev/null || true
    wait "$APP_PID" 2>/dev/null || true
    su - postgres -c "pg_ctl -D $PGDATA stop -m fast" || true
    echo "[entrypoint] Stopped"
    exit 0
}
trap cleanup SIGTERM SIGINT

# Steps 3+4: Start application (migrations run on startup via kjxlkj_db::migrate)
echo "[entrypoint] Starting kjxlkj server..."
/app/kjxlkj &
APP_PID=$!

# Wait for app process
wait "$APP_PID"
