# Verification

## Goal

Run the same full-compose verification bundle used as the canonical acceptance path.

## Full Compose Bundle

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres seaweedfs app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Outcomes

- `verify` exits `0` only when formatting, clippy, tests, release build, docs topology, docs links, docs term collection, and line-limit checks all pass.
- `visual-verify` exits `0` only when browser-rendered desktop and compact screenshots pass.
- Compose health checks stay enabled for both runtime and verification flows.
- No `.env` file is required for the canonical compose verification path.
