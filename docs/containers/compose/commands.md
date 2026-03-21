# Compose Commands

## Development

```bash
docker compose build app
docker compose up
```

## Verification

```bash
docker compose --profile verify run --rm verify
```

## Contract

- Development flow MUST prebuild the app image from `Dockerfile` before `up`.
- `docker compose up` MUST NOT start `verify`.
- Command outputs and exit codes are consumed by automation.
- Non-zero exit code indicates contract failure.
