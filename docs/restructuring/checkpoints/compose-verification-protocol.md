# Compose Verification Protocol

## Required Command

```bash
docker compose --profile verify run --rm verify
```

## Contingent Rule

- If compose assets exist, this command is mandatory.
- If compose assets are missing, record `blocked` with root cause and recovery owner.

## Recovery Requirement for Blocked State

1. Restore compose assets required by [../../containers/compose/commands.md](../../containers/compose/commands.md).
2. Re-run the command.
3. Update [../evidence/final.md](../evidence/final.md) with final status.
