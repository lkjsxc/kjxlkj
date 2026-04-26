# CI Triage Contract

## First Read

- Identify the exact failing workflow step before changing code.
- Treat missing service health, `verify` failure, and `visual-verify` failure as different failure classes.
- Read compose status and logs before assuming the failing service is the direct cause.
- Include `postgres`, `seaweedfs`, and `app` in service status and log collection.

## Local Reproduction

Use the same compose service set as CI:

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres seaweedfs app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Fix Rule

- Update docs canon before changing workflow or implementation behavior.
- Keep CI commands aligned with [../verification/compose-pipeline.md](../verification/compose-pipeline.md).
- Prefer explicit compose service names over relying on implicit dependency startup.
- Preserve visual artifacts and compose logs on failure.
