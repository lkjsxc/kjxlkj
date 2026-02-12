#!/usr/bin/env bash
set -euo pipefail

PGDATA="${PGDATA:-/var/lib/postgresql/data}"
PGPORT="${PGPORT:-5432}"
POSTGRES_USER="${POSTGRES_USER:-kjxlkj}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-kjxlkj}"
POSTGRES_DB="${POSTGRES_DB:-kjxlkj}"
BIND_ADDR="${BIND_ADDR:-0.0.0.0:8080}"
PG_BIN_DIR="${PG_BIN_DIR:-$(dirname "$(find /usr/lib/postgresql -type f -name initdb | head -n1)")}"

if [ -z "$PG_BIN_DIR" ] || [ ! -x "$PG_BIN_DIR/initdb" ]; then
  echo "postgres binaries not found" >&2
  exit 1
fi

mkdir -p "$PGDATA"
chown -R postgres:postgres "$(dirname "$PGDATA")"

if [ ! -s "$PGDATA/PG_VERSION" ]; then
  su postgres -s /bin/bash -c "'$PG_BIN_DIR/initdb' -D '$PGDATA'"
fi

su postgres -s /bin/bash -c "'$PG_BIN_DIR/pg_ctl' -D '$PGDATA' -o \"-c listen_addresses='127.0.0.1' -c port=$PGPORT\" -w start"

su postgres -s /bin/bash -c \
  "psql --dbname postgres -tAc \"SELECT 1 FROM pg_roles WHERE rolname='${POSTGRES_USER}'\" | grep -q 1 \
   || psql --dbname postgres -c \"CREATE ROLE \\\"${POSTGRES_USER}\\\" LOGIN PASSWORD '${POSTGRES_PASSWORD}'\""

su postgres -s /bin/bash -c \
  "psql --dbname postgres -tAc \"SELECT 1 FROM pg_database WHERE datname='${POSTGRES_DB}'\" | grep -q 1 \
   || psql --dbname postgres -c \"CREATE DATABASE \\\"${POSTGRES_DB}\\\" OWNER \\\"${POSTGRES_USER}\\\"\""

export DATABASE_URL="${DATABASE_URL:-postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:${PGPORT}/${POSTGRES_DB}}"
export BIND_ADDR

term_handler() {
  if [ -n "${APP_PID:-}" ]; then
    kill -TERM "$APP_PID" 2>/dev/null || true
    wait "$APP_PID" || true
  fi
  su postgres -s /bin/bash -c "'$PG_BIN_DIR/pg_ctl' -D '$PGDATA' -m fast stop" || true
}
trap term_handler TERM INT EXIT

/usr/local/bin/kjxlkj-server &
APP_PID=$!
wait "$APP_PID"
