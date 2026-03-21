# Container Deployment Assumptions

## Baseline

- `docker compose` command contracts in this docs tree are the canonical local orchestration entrypoint.
- The default stack includes `app` and `postgres` only.
- App build and mount policy is canonical in [compose/build-storage-contract.md](compose/build-storage-contract.md).

## Verification Mode

- Verification runs in an isolated `verify` profile and is opt-in.
- Verification profile executes formatting, linting, tests, and build checks.

## Contract Rule

- Compose commands documented in this tree are the canonical automation path.
