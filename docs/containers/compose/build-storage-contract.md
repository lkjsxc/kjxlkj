# Compose Build and Storage Contract

## Scope

- This file is the canonical contract for compose app-image build strategy, compose file location, and host mount layout.
- If other docs differ, this file is authoritative for compose build and mount rules.
- Repository root prerequisites are governed by [../../repository/structure/root-layout.md](../../repository/structure/root-layout.md).

## Rule 1: Compose Runtime Definition Location

- Compose runtime definitions MUST live in repository-root `docker-compose.yml`.
- `app` build definition MUST reference repository-root `Dockerfile`.

### Required Example

```yaml
services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
```

## Rule 2: App Image Build Strategy

- `app` MUST be built from repository `Dockerfile`.
- The canonical local strategy is prebuild first, then start services.
- `app` MUST NOT depend on a repository-root source mount for runtime execution.

### Required Example

```bash
docker compose build app
docker compose up
```

### Disallowed Example

```yaml
services:
  app:
    image: rust:bookworm
    volumes:
      - ./:/work
```

## Rule 3: Host Mount Root

- All host bind mounts MUST use paths under `./data/`.
- Host bind mounts from repository root (for example `./:/work`) are forbidden.

### Required Example

```yaml
services:
  app:
    volumes:
      - ./data/app:/app/data
```

## Rule 4: PostgreSQL Mount Path

- PostgreSQL durable data MUST mount from `./data/postgres`.
- The canonical container target path is `/var/lib/postgresql/data`.

### Required Example

```yaml
services:
  postgres:
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
```

## Rule 5: Verify Service Startup

- `verify` MUST be profile-gated and opt-in.
- `docker compose up` (default profile) MUST start `app` and `postgres` only.
- `verify` runs only through explicit verify commands.

### Required Example

```bash
docker compose --profile verify run --rm verify
```

## Rule 6: Migration Asset Delivery (Preferred)

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
