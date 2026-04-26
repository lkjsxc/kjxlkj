# Root Layout Contract

## Required Root Entries

- `.gitignore`
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
- No root-level compatibility copies of `tests/`, `visual/`, `vendor/`, proxy config, or TURN config remain.

## Runtime State Entries

- `tmp/visual-artifacts/` may exist locally and remains disposable.
