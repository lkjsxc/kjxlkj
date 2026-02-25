# Docker Deployment Guide

**Back:** [Quick Start](/README.md) | [Architecture](/docs/spec/architecture/README.md)

---

## Overview

kjxlkj provides Docker containerization for consistent deployment across development and production environments.

---

## Quick Start

### Build and Run with Docker Compose

```bash
# Build and start all services
docker compose up --build

# Access the application
# Server: http://localhost:8080
# Health check: http://localhost:8080/api/healthz
```

### Run with Docker (Single Container)

```bash
# Build the image
docker build -t kjxlkj-server .

# Run the container
docker run -d \
  -p 8080:8080 \
  -v $(pwd)/data:/app/data:ro \
  -v $(pwd)/static:/app/static:ro \
  --name kjxlkj \
  kjxlkj-server
```

---

## Services

### PostgreSQL (with pgvector)

- **Image:** `pgvector/pgvector:pg16`
- **Port:** 5432
- **Database:** kjxlkj
- **User:** kjxlkj
- **Password:** kjxlkj_dev_password

### kjxlkj Server

- **Port:** 8080
- **Health Check:** `/api/healthz`
- **Readiness:** `/api/readyz`

### LMStudio (Optional - Embeddings)

- **Profile:** `with-embeddings`
- **Port:** 1234
- **Purpose:** Local embedding generation for semantic search

### Frontend Dev (Optional)

- **Profile:** `development`
- **Port:** 3000
- **Purpose:** Hot-reload development server

---

## Docker Compose Profiles

### Default (Server + Database)

```bash
docker compose up
```

### With Embeddings (Server + Database + LMStudio)

```bash
docker compose --profile with-embeddings up
```

### Development (Server + Database + Frontend Dev)

```bash
docker compose --profile development up
```

---

## Environment Variables

### Server Configuration

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | No | — | PostgreSQL connection string |
| `RUST_LOG` | No | `info` | Logging level |

### Database Configuration

| Variable | Required | Default |
|----------|----------|---------|
| `POSTGRES_USER` | Yes | `kjxlkj` |
| `POSTGRES_PASSWORD` | Yes | `kjxlkj_dev_password` |
| `POSTGRES_DB` | Yes | `kjxlkj` |

---

## Volumes

| Volume | Purpose |
|--------|---------|
| `postgres_data` | PostgreSQL data persistence |
| `lmstudio_models` | LMStudio model cache |
| `./data:/app/data:ro` | Configuration files (read-only) |
| `./static:/app/static:ro` | Static assets (read-only) |
| `./migrations:/docker-entrypoint-initdb.d:ro` | Database migrations |

---

## Health Checks

### Server Health

```bash
curl http://localhost:8080/api/healthz
```

**Response:**
```json
{ "status": "ok" }
```

### Database Health

```bash
docker exec kjxlkj-postgres pg_isready -U kjxlkj -d kjxlkj
```

---

## Logs

### View All Logs

```bash
docker compose logs -f
```

### View Specific Service Logs

```bash
docker compose logs -f server
docker compose logs -f postgres
```

---

## Database Migrations

Migrations are automatically applied on first startup via `/docker-entrypoint-initdb.d/`.

**Manual Migration:**

```bash
docker exec -i kjxlkj-postgres psql -U kjxlkj -d kjxlkj < migrations/001_users_sessions.sql
```

---

## Configuration

### Update `data/config.json`

Edit configuration before starting:

```bash
# Edit configuration
vim data/config.json

# Restart services
docker compose restart server
```

### Environment-Specific Config

Create environment-specific config:

```bash
# Production
cp data/config.json data/config.prod.json

# Mount in compose
volumes:
  - ./data/config.prod.json:/app/data/config.json:ro
```

---

## Production Deployment

### Build Optimized Image

```dockerfile
# Use specific Rust version for reproducibility
FROM rust:1.75-slim-bookworm AS builder

# ... (see Dockerfile)
```

### Security Hardening

1. **Non-root user** — Container runs as `kjxlkj` (UID 1000)
2. **Read-only volumes** — Config and static files mounted read-only
3. **Health checks** — Automatic container restart on failure
4. **Network isolation** — Dedicated bridge network

### Scaling

```bash
# Run multiple replicas (requires external DB)
docker compose up --scale server=3
```

---

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker compose logs server

# Verify configuration
docker compose config
```

### Database Connection Failed

```bash
# Check database is healthy
docker compose ps postgres

# Test connection
docker exec kjxlkj-postgres psql -U kjxlkj -d kjxlkj -c "SELECT 1"
```

### Port Already in Use

```bash
# Change port in docker-compose.yml
ports:
  - "8081:8080"  # Use 8081 instead of 8080
```

### Rebuild from Scratch

```bash
# Stop and remove all containers/volumes
docker compose down -v

# Rebuild and start
docker compose up --build
```

---

## Related

- **Architecture:** [Runtime Model](/docs/spec/architecture/runtime.md)
- **Configuration:** [Config Contract](/docs/spec/architecture/configuration.md)
- **Deployment:** [Deployment Spec](/docs/spec/architecture/deployment.md)

---

## Commands Reference

| Command | Description |
|---------|-------------|
| `docker compose up` | Start all services |
| `docker compose up --build` | Rebuild and start |
| `docker compose down` | Stop and remove containers |
| `docker compose down -v` | Stop and remove volumes |
| `docker compose logs -f` | Follow logs |
| `docker compose ps` | Show running containers |
| `docker compose exec server sh` | Shell in server container |
| `docker compose restart server` | Restart server |

---

**Guide End**
