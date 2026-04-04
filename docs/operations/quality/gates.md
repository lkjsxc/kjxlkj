# Quality Gates Contract

## Mandatory Gates

1. `cargo fmt -- --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. `cargo build --release`
5. `cargo run --bin kjxlkj -- docs validate-topology`
6. `cargo run --bin kjxlkj -- docs validate-terms`
7. `cargo run --bin kjxlkj -- quality check-lines`
8. `docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify`
9. `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres app`
10. `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
11. `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify`
12. `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`

## Stop Rule

Any non-zero gate result blocks acceptance.

## Line-Limit Rule

- `check-lines` enforces the 300-line docs limit and the 200-line authored-code limit.
- Authored-code checks cover the authored subsets of `src/`, including `src/tests/` and `src/verify/`.
- Vendored upstream artifacts under `src/web/assets/vendor/` remain pinned and unmodified but are excluded from authored-file pressure.
