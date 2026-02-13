# Wave 091: Performance and Operations Evidence Archive

Back: [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] execute `PERF-01`, `PERF-02`, and `PERF-03`
- [ ] execute `OPS-01` and `OPS-02`
- [ ] archive throughput, replay, and provider-failure evidence artifacts

## Verification Tasks

- [ ] validate performance envelope against target limits
- [ ] validate backup/restore and restart recovery proofs

## Evidence Placeholder

- [ ] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test performance_smoke --test ops_recovery --test admin_jobs_api --test automation_provider_adapter -- --nocapture`
- [ ] `Result:` pass
- [ ] `Proof:` `performance_smoke`: `2 passed` (`PERF-01`, `PERF-02`, `PERF-03`); `ops_recovery`: `1 passed`; `admin_jobs_api`: `1 passed`; `automation_provider_adapter`: `5 passed` including provider-failure and retry envelope diagnostics
