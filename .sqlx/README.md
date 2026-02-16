# SQLx Offline Cache

This directory enables `SQLX_OFFLINE=true` for compile-time query checking
without a live database connection.

## How It Works

SQLx can validate queries at compile time using cached metadata from a
previous database connection. This directory stores that metadata.

## Generating the Cache

When a PostgreSQL database is available:

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx database create
sqlx migrate run

# Generate offline cache
cargo sqlx prepare --workspace
```

This writes `.sqlx/query-*.json` files for each `sqlx::query!()` macro call.

## Flags

Set in the build environment:

```bash
export SQLX_OFFLINE=true
```

Or in `.env`:

```
SQLX_OFFLINE=true
```

## Current Status

The in-memory repository implementations do not use `sqlx::query!()` macros,
so no query cache files are needed yet. When PostgreSQL repository
implementations are added (IMP-PG-01), this directory will be populated
with the actual query metadata.

## Related

- Spec: /docs/spec/technical/migrations.md
- Improvement: IMP-ARC-01 in /docs/reference/IMPROVEMENT_BACKLOG.md
