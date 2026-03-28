# Automation Command Contracts

## CLI Commands

- `docs validate-topology`
- `docs validate-terms`
- `quality check-lines`
- `compose verify`

## Compose Command Bundle

- `docker compose build app verify visual-verify`
- `docker compose up -d postgres app`
- `docker compose --profile verify run --rm verify`
- `docker compose --profile verify run --rm visual-verify`
- `docker compose down -v`

## Output Rules

- Commands emit deterministic JSON lines.
- Final JSON line includes `command` and `status`.
- Non-zero process exit indicates contract failure.
- CI wrappers may add logs and artifact upload, but they may not weaken command failure behavior.
