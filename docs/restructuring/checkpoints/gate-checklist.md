# Gate Checklist

## Required Ordered Gates

1. `cargo run --bin kjxlkj -- docs validate-topology`
2. `cargo run --bin kjxlkj -- docs validate-terms`
3. `cargo run --bin kjxlkj -- quality check-lines`
4. `cargo fmt -- --check`
5. `cargo clippy --all-targets -- -D warnings`
6. `cargo test`
7. `cargo build --release`
8. `docker compose --profile verify run --rm verify`

## Rule

Any failing gate restarts the sequence from gate 1 after fixes.
