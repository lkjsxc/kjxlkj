# Compose Verification Services

## `verify`

- Purpose: run all Rust and docs quality gates in one deterministic container command.
- Canonical command: `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`.
- Exit `0` only when formatting, clippy, tests, release build, docs topology, docs links, docs terms, and line-limit checks all pass.

## `verify` Runtime Requirements

- Rust toolchain installed.
- Source mounted read-only at `/workspace`.
- Writable Cargo target directory mounted outside the source tree at `/target`.
- Access to the same compose network and healthy runtime dependencies used by app verification.

## `visual-verify`

- Purpose: run browser-rendered screenshot and interaction checks against the live compose stack.
- Canonical command: `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify`.
- Exit `0` only when desktop and compact screenshots pass.
- Fail on broken upload flows, file delivery, resource-card regressions, drawer regressions, or preview regressions.

## `visual-verify` Runtime Requirements

- Playwright runtime with Chromium.
- Access to healthy `postgres`, `minio`, and `app` services.
- Deterministic scripts under `src/verify/browser/`.
- Writable artifact target at `/artifacts`, exposed to the repo as `tmp/visual-artifacts/`.
