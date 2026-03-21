# Local Verification Runbook

## Steps

1. Ensure Docker and Compose are available.
2. From repository root, run `docker compose --profile verify run --rm verify`.
3. Inspect non-zero exit codes and failing step logs.

## Completion Criteria

- Verify command exits with code `0`.
- No quality gate reports a failure condition.
