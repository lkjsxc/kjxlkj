# Verification

## Goal

Run the same full-compose verification bundle used as the canonical acceptance path.

## Full Compose Bundle

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Outcomes

- `verify` exits `0` only when formatting, clippy, tests, release build, docs topology, docs term collection, and line-limit checks all pass.
- `visual-verify` exits `0` only when browser-rendered desktop and compact screenshots pass.
- Compose health checks stay enabled for both runtime and verification flows.
- Failure of any command blocks acceptance.

## Artifacts and State

- Browser screenshots are written under `tmp/visual-artifacts/`.
- Cargo verification caches use `kjxlkj-verify-cargo` and `kjxlkj-verify-target`.
- PostgreSQL state is disposable in this bundle because `down -v` removes the named volume.

## When to Use Deeper Docs

- Use [../operations/verification/compose-pipeline.md](../operations/verification/compose-pipeline.md) for the canonical pipeline contract.
- Use [../operations/verification/local-runbook.md](../operations/verification/local-runbook.md) for manual endpoint and UI checks.
- Use [../operations/quality/gates.md](../operations/quality/gates.md) for the full gate list.
