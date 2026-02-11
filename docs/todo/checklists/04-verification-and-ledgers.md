# Checklist 04: Verification and Ledgers

Back: [/docs/todo/checklists/README.md](/docs/todo/checklists/README.md)

## Verification Gates

- [ ] run the targeted blocker suites first
- [ ] run full workspace gate after blocker suites are green
- [ ] confirm no high-severity mismatch remains open

## Mandatory Runtime Commands

- [ ] `cargo test -p kjxlkj-test-harness --test key_mode_e2e`
- [ ] `cargo test -p kjxlkj-test-harness --test window_nav_e2e --test window_nav_more_e2e --test window_nav_session_terminal_e2e`
- [ ] `cargo test -p kjxlkj-test-harness --test explorer_terminal_paths_e2e --test explorer_terminal_more_e2e --test explorer_terminal_stress_e2e`
- [ ] `cargo test -p kjxlkj-test-harness --test cursor_wrap_e2e --test cursor_wrap_more_e2e`
- [ ] `cargo test -p kjxlkj-test-harness --test profiling_e2e`
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`

## Ledger Synchronization

- [ ] update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] update [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] confirm [/docs/reference/CI.md](/docs/reference/CI.md) profile matches current state
- [ ] confirm [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) gate statement matches open blockers

## Exit to Next Checklist

- [ ] continue to [05-release-readiness.md](05-release-readiness.md)
