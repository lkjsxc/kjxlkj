# Compose Profiles

## Default Profile

- Starts `app` and `postgres` services only.
- Intended for local functional development.
- MUST NOT auto-start `verify`.

## Verify Profile

- Runs the repository verification pipeline.
- Designed for deterministic CI-equivalent execution.

## Selection Rule

- Use default profile for interactive development.
- Use verify profile for explicit pass/fail quality validation.
