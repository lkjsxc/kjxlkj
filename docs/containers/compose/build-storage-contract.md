# Compose Build and Storage Contract

## Scope

- This file is the canonical contract for app-image build strategy and container mount layout.
- If other docs differ, this file is authoritative for build and mount rules.

## Rule 1: App Image Build Strategy

- `app` MUST be built from a repository `Dockerfile`.
- The canonical local strategy is prebuild first, then start services.
- `app` MUST NOT depend on a repository-root source mount for runtime execution.

### Required Example

```bash
docker compose build app
docker compose up
```

```yaml
services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
```

### Disallowed Example

```yaml
services:
  app:
    image: rust:bookworm
    volumes:
      - ./:/work
```

## Rule 2: Host Mount Root

- All host bind mounts MUST use paths under `./data/`.
- Host bind mounts from repository root (for example `./:/work`) are forbidden.

### Required Example

```yaml
services:
  app:
    volumes:
      - ./data/app:/app/data
```

## Rule 3: PostgreSQL Mount Path

- PostgreSQL durable data MUST mount from `./data/postgres`.
- The canonical container target path is `/var/lib/postgresql/data`.

### Required Example

```yaml
services:
  postgres:
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
```

## Rule 4: Verify Service Startup

- `verify` MUST be profile-gated and opt-in.
- `docker compose up` (default profile) MUST start `app` and `postgres` only.
- `verify` runs only through explicit verify commands.

### Required Example

```bash
docker compose --profile verify run --rm verify
```

## Rule 5: Migration Asset Delivery

- Migration SQL SHOULD be baked into the app image at build time.
- The canonical strategy is `COPY migrations/ ...` in `Dockerfile`.
- Compose migration mounts are non-canonical and SHOULD NOT be used.

### Required Example

```dockerfile
COPY migrations/ /app/migrations/
```

### Disallowed Example

```yaml
services:
  postgres:
    volumes:
      - ./migrations:/docker-entrypoint-initdb.d:ro
```
