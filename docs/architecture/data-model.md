# Data Model

## PostgreSQL Tables

### `admin_users`

- `id BIGSERIAL PRIMARY KEY`
- `username TEXT UNIQUE NOT NULL`
- `password_hash TEXT NOT NULL`
- `created_at TIMESTAMPTZ NOT NULL DEFAULT now()`

### `sessions`

- `id UUID PRIMARY KEY`
- `admin_id BIGINT NOT NULL REFERENCES admin_users(id) ON DELETE CASCADE`
- `expires_at TIMESTAMPTZ NOT NULL`
- `created_at TIMESTAMPTZ NOT NULL DEFAULT now()`

## Markdown Frontmatter Contract

Frontmatter is optional. When present it is YAML between `---` delimiters.

Supported fields:

- `title: <string>`
- `private: <bool>`

Example:

```yaml
---
title: Welcome
private: false
---
# Hello
```

## Visibility Rules

- Missing `private` is treated as `false`.
- Public site excludes `private: true` pages.
- Admin UI can read/write both public and private pages.
