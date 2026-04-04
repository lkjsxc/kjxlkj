# Compose Files Contract

## Runtime File

- `docker-compose.yml` is the canonical runtime stack file.
- It defines `postgres` and `app`.
- It is the file used for normal deploy, boot, and shutdown operations.

## Verification Overlay

- `docker-compose.verify.yml` is the canonical verification overlay file.
- It defines `verify` and `visual-verify`.
- It is layered on top of `docker-compose.yml` for build and verification work.

## Environment File

- `.env.example` is the tracked template.
- `.env` is the local active file loaded by Compose.
- Session timeout is not sourced from `.env`; it is stored in `app_settings`.
