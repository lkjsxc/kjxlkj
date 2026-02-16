#!/usr/bin/env bash
set -euo pipefail

# kjxlkj single-container entrypoint
# 1. Init DB directory if missing
# 2. Start PostgreSQL and wait for readiness
# 3. Create DB and user if needed
# 4. Run migrations (handled by app)
# 5. Start app server
# 6. Forward shutdown signals

PGDATA="${PGDATA:-/var/lib/postgresql/16/main}"
PG_USER="kjxlkj"
PG_DB="kjxlkj"
PG_PASS="${POSTGRES_PASSWORD:-kjxlkj}"

cleanup() {
    echo "[entrypoint] shutting down..."
    if [ -f /var/run/postgresql/16-main.pid ]; then
        gosu postgres pg_ctlcluster 16 main stop -m fast 2>/dev/null || true
    fi
    kill -TERM "$APP_PID" 2>/dev/null || true
    wait "$APP_PID" 2>/dev/null || true
    echo "[entrypoint] shutdown complete"
    exit 0
}

trap cleanup SIGTERM SIGINT SIGQUIT

# 1. Init DB directory if missing
if [ ! -f "$PGDATA/PG_VERSION" ]; then
    echo "[entrypoint] initializing PostgreSQL data directory..."
    mkdir -p "$PGDATA"
    chown -R postgres:postgres "$PGDATA"
    gosu postgres /usr/lib/postgresql/16/bin/initdb -D "$PGDATA" --auth-local=trust --auth-host=md5
fi

# Ensure ownership
chown -R postgres:postgres "$PGDATA"

# 2. Start PostgreSQL
echo "[entrypoint] starting PostgreSQL..."
gosu postgres pg_ctlcluster 16 main start || gosu postgres /usr/lib/postgresql/16/bin/pg_ctl -D "$PGDATA" start -w -o "-c listen_addresses=127.0.0.1 -c port=5432"

# Wait for PostgreSQL readiness
for i in $(seq 1 30); do
    if gosu postgres psql -h 127.0.0.1 -p 5432 -U postgres -c "SELECT 1" >/dev/null 2>&1; then
        echo "[entrypoint] PostgreSQL is ready"
        break
    fi
    if [ "$i" -eq 30 ]; then
        echo "[entrypoint] ERROR: PostgreSQL did not become ready in 30s"
        exit 1
    fi
    sleep 1
done

# 3. Create user and database if needed
gosu postgres psql -h 127.0.0.1 -p 5432 -U postgres -tc "SELECT 1 FROM pg_roles WHERE rolname='$PG_USER'" | grep -q 1 || \
    gosu postgres psql -h 127.0.0.1 -p 5432 -U postgres -c "CREATE ROLE $PG_USER LOGIN PASSWORD '$PG_PASS'"

gosu postgres psql -h 127.0.0.1 -p 5432 -U postgres -tc "SELECT 1 FROM pg_database WHERE datname='$PG_DB'" | grep -q 1 || \
    gosu postgres psql -h 127.0.0.1 -p 5432 -U postgres -c "CREATE DATABASE $PG_DB OWNER $PG_USER"

# 4. Start app server (migrations run inside the app)
echo "[entrypoint] starting kjxlkj-server..."
export DATABASE_URL="postgres://${PG_USER}:${PG_PASS}@127.0.0.1:5432/${PG_DB}"

/usr/local/bin/kjxlkj-server &
APP_PID=$!

echo "[entrypoint] kjxlkj-server started (PID $APP_PID)"

# 5. Wait for app process
wait "$APP_PID"
