# Compose Service Contract

## Services

- `app`: runtime service exposing port `8080`.
- `verify`: profile-gated quality verification service.

## Profile Rule

- Default `docker compose up` starts `app` only.
- `verify` runs only with `--profile verify`.
