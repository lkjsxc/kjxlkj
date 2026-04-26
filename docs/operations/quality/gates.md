# Quality Gates Contract

## Mandatory Gates

1. `cargo fmt -- --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. `cargo build --release`
5. `cargo run --bin kjxlkj -- docs validate-topology`
6. `cargo run --bin kjxlkj -- docs validate-links`
7. `cargo run --bin kjxlkj -- docs validate-terms`
8. `cargo run --bin kjxlkj -- quality check-lines`
9. `docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify`
10. `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres seaweedfs app`
11. `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
12. `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify`
13. `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`

## Stop Rule

- Any non-zero gate result blocks acceptance.

## Line-Limit Rule

- `check-lines` enforces the 300-line docs limit and the 200-line authored-code limit.
- Authored-code checks cover the authored subsets of `src/`, including `src/tests/`, `src/verify/`, and future `src/storage/`.
- Files above the headroom targets in [../../repository/rules/line-limits.md](../../repository/rules/line-limits.md) should be split when they are already being edited.
