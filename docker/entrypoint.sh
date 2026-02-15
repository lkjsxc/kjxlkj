#!/bin/bash
set -euo pipefail

# Container entrypoint per /docs/spec/architecture/deployment.md.
# Process supervision contract:
# 1. Initialize PostgreSQL data directory if missing
# 2. Start PostgreSQL and wait for readiness
# 3. Run SQL migrations (handled by app on startup)
# 4. Start application server
# 5. Forward termination signals and stop both processes cleanly

PGDATA="${POSTGRES_DATA_DIR:-/var/lib/postgresql/data}"

# --- Step 1: Initialize PostgreSQL if needed ---
if [ ! -f "$PGDATA/PG_VERSION" ]; then
  echo "[entrypoint] Initializing PostgreSQL data directory..."
  gosu postgres /usr/lib/postgresql/15/bin/initdb -D "$PGDATA" --auth-local=trust --auth-host=trust
fi

# --- Step 2: Start PostgreSQL ---
echo "[entrypoint] Starting PostgreSQL..."
gosu postgres /usr/lib/postgresql/15/bin/pg_ctl -D "$PGDATA" -l /tmp/pg.log start

# Wait for PG to be ready
for i in $(seq 1 30); do
  if gosu postgres /usr/lib/postgresql/15/bin/pg_isready -q; then
    break
  fi
  sleep 1
done

if ! gosu postgres /usr/lib/postgresql/15/bin/pg_isready -q; then
  echo "[entrypoint] ERROR: PostgreSQL failed to start"
  cat /tmp/pg.log
  exit 1
fi

# Ensure the database exists
gosu postgres /usr/lib/postgresql/15/bin/psql -c "SELECT 1 FROM pg_database WHERE datname='kjxlkj'" | grep -q 1 || \
  gosu postgres /usr/lib/postgresql/15/bin/createdb kjxlkj

echo "[entrypoint] PostgreSQL is ready."

# --- Step 3+4: Start application server (migrations run inside app) ---
# Set DATABASE_URL if not already set
export DATABASE_URL="${DATABASE_URL:-postgresql://postgres@localhost/kjxlkj}"

echo "[entrypoint] Starting kjxlkj application server..."
/app/kjxlkj-server &
APP_PID=$!

# --- Step 5: Forward signals ---
trap_handler() {
  echo "[entrypoint] Received shutdown signal, stopping services..."
  kill "$APP_PID" 2>/dev/null || true
  wait "$APP_PID" 2>/dev/null || true
  gosu postgres /usr/lib/postgresql/15/bin/pg_ctl -D "$PGDATA" stop -m fast
  echo "[entrypoint] All processes stopped."
  exit 0
}

trap trap_handler SIGTERM SIGINT SIGQUIT

# Wait for app process
wait "$APP_PID"
EXIT_CODE=$?

# If app exits, stop postgres too
gosu postgres /usr/lib/postgresql/15/bin/pg_ctl -D "$PGDATA" stop -m fast 2>/dev/null || true
exit "$EXIT_CODE"
