# Local Verification Runbook

## Steps

1. Ensure Docker and Compose are available.
2. From repository root, run `docker compose build app`.
3. Start default services with `docker compose up`.
4. Run verify checks with `docker compose --profile verify run --rm verify`.
5. Inspect non-zero exit codes and failing step logs.

## Completion Criteria

- Verify command exits with code `0`.
- No quality gate reports a failure condition.
