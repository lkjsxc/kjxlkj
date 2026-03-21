# PostgreSQL Schema Contract

## `admin_users`

- `id BIGSERIAL PRIMARY KEY`
- `username TEXT UNIQUE NOT NULL`
- `password_hash TEXT NOT NULL`
- `created_at TIMESTAMPTZ NOT NULL DEFAULT now()`

## `sessions`

- `id UUID PRIMARY KEY`
- `admin_id BIGINT NOT NULL REFERENCES admin_users(id) ON DELETE CASCADE`
- `expires_at TIMESTAMPTZ NOT NULL`
- `created_at TIMESTAMPTZ NOT NULL DEFAULT now()`

## Lifecycle Rule

- Session rows are bound to admin lifecycle through cascade delete.
