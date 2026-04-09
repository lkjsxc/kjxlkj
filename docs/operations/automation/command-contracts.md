# Automation Command Contracts

## CLI Commands

- `docs validate-topology`
- `docs validate-terms`
- `quality check-lines`
- `compose verify`

## Compose Command Bundle

- `cp .env.example .env` when `.env` is absent in CI or a fresh checkout
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres app`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`

## Output Rules

- Commands emit deterministic JSON lines.
- Final JSON line includes `command` and `status`.
- Non-zero process exit indicates contract failure.
- CI wrappers may add logs and artifact upload, but they may not weaken command failure behavior.
