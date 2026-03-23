# Runtime Environment Contract

## Required Variables

- `BIND_HOST` default: `0.0.0.0`
- `BIND_PORT` default: `8080`
- `DATA_ROOT` default: `/app/data`
- `ADMIN_TOKEN` required for write operations

## Boot Rule

App process fails fast when required configuration is invalid.
