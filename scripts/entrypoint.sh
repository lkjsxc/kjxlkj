#!/usr/bin/env bash
# Per /docs/spec/architecture/deployment.md §Process Supervision Contract:
# 1. Initialize PostgreSQL data directory if missing
# 2. Start PostgreSQL and wait for readiness
# 3. Run SQL migrations
# 4. Start application server
# 5. Forward termination signals and stop both processes cleanly
set -euo pipefail

PGDATA="${POSTGRES_DATA_DIR:-/var/lib/postgresql/data}"
MIGRATIONS_DIR="${KJXLKJ_MIGRATIONS_DIR:-/app/migrations}"
POSTGRES_BIN_DIR="${POSTGRES_BIN_DIR:-/usr/lib/postgresql/15/bin}"
PG_INITDB="$POSTGRES_BIN_DIR/initdb"
PG_CTL="$POSTGRES_BIN_DIR/pg_ctl"
PG_ISREADY="$POSTGRES_BIN_DIR/pg_isready"

# Compose volume mounts can override image-time ownership; enforce runtime ownership.
mkdir -p "$PGDATA"
chown -R postgres:postgres "$PGDATA"

# --- 1. Initialize PostgreSQL data directory if missing ---
if [ ! -f "$PGDATA/PG_VERSION" ]; then
  echo "[entrypoint] Initializing PostgreSQL data directory..."
  su postgres -c "$PG_INITDB -D \"$PGDATA\" --auth=trust --no-locale -E UTF8"
  # Allow local connections
  echo "host all all 127.0.0.1/32 trust" >> "$PGDATA/pg_hba.conf"
  echo "listen_addresses = '127.0.0.1'" >> "$PGDATA/postgresql.conf"
fi

# --- 2. Start PostgreSQL and wait for readiness ---
echo "[entrypoint] Starting PostgreSQL..."
su postgres -c "$PG_CTL -D \"$PGDATA\" -l /tmp/pg.log start"

echo "[entrypoint] Waiting for PostgreSQL readiness..."
for i in $(seq 1 30); do
  if su postgres -c "$PG_ISREADY -q"; then
    break
  fi
  sleep 1
done

# Create database if not exists
su postgres -c "psql -tc \"SELECT 1 FROM pg_database WHERE datname='kjxlkj'\" \
  | grep -q 1 || psql -c 'CREATE DATABASE kjxlkj'"

# --- 3. Run SQL migrations ---
# The app startup runs sqlx-managed migrations deterministically.
echo "[entrypoint] Migrations are managed by app startup."

# Export DATABASE_URL if not set
export DATABASE_URL="${DATABASE_URL:-postgres://postgres@127.0.0.1/kjxlkj}"

# --- 4. Start application server ---
echo "[entrypoint] Starting kjxlkj server..."
/app/kjxlkj-server &
APP_PID=$!

# --- 5. Forward termination signals ---
cleanup() {
  echo "[entrypoint] Shutting down..."
  kill "$APP_PID" 2>/dev/null || true
  wait "$APP_PID" 2>/dev/null || true
  su postgres -c "pg_ctl -D $PGDATA stop -m fast" 2>/dev/null || true
  echo "[entrypoint] Shutdown complete."
}

trap cleanup SIGTERM SIGINT

set +e
wait "$APP_PID"
APP_STATUS=$?
set -e

cleanup
exit "$APP_STATUS"
