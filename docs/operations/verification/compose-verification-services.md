# Compose Verification Services

## `verify`

- Purpose: run all Rust and docs quality gates in one deterministic container command
- Canonical command: `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- Exit `0` only when formatting, clippy, tests, release build, docs topology, docs term collection, and line-limit checks all pass
- Stop at the first failing gate

## `verify` Runtime Requirements

- Rust toolchain installed
- Source mounted read-only at `/workspace`
- Writable Cargo target directory mounted outside the source tree at `/target`
- `CARGO_TARGET_DIR=/target`
- Access to the same compose network and healthy `app` dependency used by runtime verification

## `visual-verify`

- Purpose: run browser-rendered screenshot and contrast checks against the live compose `app` service
- Canonical command: `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify`
- Exit `0` only when desktop and compact screenshots pass
- Fail on unreadable controls, broken public-root behavior, drawer regressions, icon regressions, or note-shell regressions

## `visual-verify` Runtime Requirements

- Playwright runtime with Chromium
- Access to the compose network
- Deterministic scripts under `src/verify/browser/`
- Writable artifact target at `/artifacts`, exposed to the repo as `tmp/visual-artifacts/`

## Acceptance Output

- `verify` is the hard gate for authored code and docs consistency.
- `visual-verify` is the hard gate for rendered UI behavior.
- A release candidate is accepted only when both services pass in the compose pipeline.
