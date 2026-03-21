#!/bin/sh
set -eu

if [ -n "${DATABASE_URL:-}" ]; then
    until psql "$DATABASE_URL" -c "SELECT 1" >/dev/null 2>&1; do
        sleep 1
    done

    for migration in /app/migrations/*.sql; do
        [ -f "$migration" ] || continue
        psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$migration"
    done
fi

exec "$@"
