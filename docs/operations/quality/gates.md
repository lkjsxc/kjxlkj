# Quality Gates Contract

## Mandatory Gates

1. `cargo fmt -- --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. `cargo build --release`
5. `cargo run --bin kjxlkj -- docs validate-topology`
6. `cargo run --bin kjxlkj -- docs validate-terms`
7. `cargo run --bin kjxlkj -- quality check-lines`
8. `docker compose build app verify visual-verify`
9. `docker compose up -d postgres app`
10. `docker compose --profile verify run --rm verify`
11. `docker compose --profile verify run --rm visual-verify`
12. `docker compose down -v`

## Stop Rule

Any non-zero gate result blocks acceptance.
