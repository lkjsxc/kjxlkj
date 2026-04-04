# Root Layout Contract

## Required Root Entries

- `.gitignore`
- `.env.example`
- `.github/`
- `LICENSE`
- `README.md`
- `docs/`
- `Cargo.toml`
- `Cargo.lock`
- `src/`
- `Dockerfile`
- `Dockerfile.verify`
- `Dockerfile.visual`
- `docker-compose.yml`
- `docker-compose.verify.yml`
- `verify.sh`

## Root Policy

- Authored code, authored tests, browser verification, and site-owned static assets live under `src/`.
- Root stays limited to entrypoint manifests, containers, docs, and disposable runtime state.
- The root `LICENSE` file is the single in-repo license text location for project licensing and bundled notice text.
- No root-level compatibility copies of `tests/`, `visual/`, or `vendor/` remain.

## Runtime State Entries

- `tmp/visual-artifacts/` may exist locally and remains disposable.
