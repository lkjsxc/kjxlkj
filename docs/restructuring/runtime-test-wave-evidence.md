# Runtime Test Wave Evidence

This note records the temporary runtime test wave against restructuring intents and
the final cleanup that restores docs-only authority.

## Command-to-Intent Mapping

| Check command | Outcome | Intent mapping |
| --- | --- | --- |
| `cargo fmt --check` | Passed | `T16-final-audit` evidence entry; supports `FI-08` repeatable validation. |
| `cargo clippy --all-targets -- -D warnings` | Passed | `T16-final-audit` evidence entry; supports `FI-08` repeatable validation. |
| `cargo test` | Passed (all tests, including compose contract tests) | `T16-final-audit` evidence entry; supports `FI-08` repeatable validation. |
| `docker compose config --quiet` | Passed | `T16-final-audit` evidence entry for deterministic compose config validity. |
| `docker compose build app` | Passed | `T16-final-audit` evidence entry for buildability contract. |
| `docker compose up` default-profile validation | Passed (`verify` not auto-started) | `T16-final-audit` evidence entry for profile contract. |
| `docker compose --profile verify run --rm verify` | Passed | `T16-final-audit` evidence entry for verify profile execution. |
| SQL status update (`UPDATE todos SET status='done' WHERE id='final-docs-only-closure'`) | Completed | `T17-status-update` terminal-state requirement. |

## Notes

- The restructuring test catalog has no compose-specific `Txx` ID, so runtime/container checks are mapped under `T16-final-audit` as explicit validation outcomes.
- Terminal SQL update is tracked separately by `T17-status-update`.

## Cleanup Closure Evidence

- Runtime-phase artifacts were removed from repository root:
  - `.dockerignore`
  - `Cargo.lock`
  - `Cargo.toml`
  - `Dockerfile`
  - `docker-compose.yml`
  - `data/`
  - `migrations/`
  - `src/`
  - `target/`
  - `tests/`
- Post-cleanup root keep-set is exact: `.gitignore`, `LICENSE`, `README.md`, and `docs/`.
- Post-cleanup validation checks passed for topology, line limits (`<300`), and README/restructuring link sanity.
