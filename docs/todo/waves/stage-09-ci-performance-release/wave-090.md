# Wave 090: CI Profile Integration and Stability

Back: [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] wire `Librarian-runtime` and `Librarian-small-model` profile execution -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] enforce deterministic fixture sets for parser and provider error paths -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] eliminate flaky signals in librarian test suites -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Verification Tasks

- [ ] run CI profile matrix on clean environment -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] run rerun stability checks for nondeterminism -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Evidence Placeholder

- [ ] `Check:` `cargo test -p kjxlkj-server tests_automation tests_ws tests_ws_replay -- --nocapture` and `for i in 1 2 3; do cargo test -p kjxlkj-server tests_ws -- --nocapture; done` -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] `Result:` pass -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] `Proof:` profile matrix and 3x rerun stability checks passed with no nondeterministic failures (`automation_provider_adapter: 5 passed`, `automation_run_flow: 1 passed`, `ws_flow: 1 passed`) -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
