#!/usr/bin/env bash
# scripts/migrate.sh — Run SQL migrations against DATABASE_URL.
#
# Spec: /docs/spec/technical/migrations.md
# Requires: psql on PATH and DATABASE_URL in .env or environment.
set -euo pipefail

if [ -f .env ]; then
  set -a; source .env; set +a
fi

if [ -z "${DATABASE_URL:-}" ]; then
  echo "ERROR: DATABASE_URL not set"
  exit 1
fi

echo "Running migrations against ${DATABASE_URL%%@*}@..."

for f in migrations/*.sql; do
  echo "  applying $(basename "$f") ..."
  psql "$DATABASE_URL" -f "$f" -v ON_ERROR_STOP=1
done

echo "Migrations complete."
