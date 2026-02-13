# Stage 09 Wave 090 Audit: CI Librarian Profiles and Stability

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Scope

Wave 090 delivery for Stage 09 CI profile integration:

- execute `Librarian-runtime` matrix (`API-AUTO-03`, `API-AUTO-04`, `WS-06`, stage-08 run workflow)
- enforce deterministic parser/provider fixture paths
- verify rerun stability to remove flaky librarian profile signals

## Implementation Evidence

Changed governance/documentation paths:

- `docs/reference/CI.md`
  - moved baseline from docs-only state to active librarian runtime profile state
  - documented `Librarian-small-model` readiness and `Release` blocker constraints
- `docs/todo/waves/stage-09-ci-performance-release/wave-090.md`
  - recorded matrix and rerun command evidence with deterministic pass results
- `docs/todo/waves/stage-09-ci-performance-release/README.md`
  - marked Wave 090 complete and stage scope progress updated

## Verification Evidence

Executed checks:

1. `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test automation_rules_api --test automation_provider_adapter --test automation_run_flow --test ws_flow -- --nocapture`
2. `for i in 1 2 3; do TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test automation_provider_adapter --test automation_run_flow --test ws_flow -- --nocapture; done`

Observed results:

- matrix run: pass
  - `automation_rules_api`: `1 passed; 0 failed`
  - `automation_provider_adapter`: `5 passed; 0 failed`
  - `automation_run_flow`: `1 passed; 0 failed`
  - `ws_flow`: `1 passed; 0 failed`
- rerun stability: pass on all 3 iterations with no nondeterministic failures

## Residual Deferred Scope

Wave 090 closes profile-matrix and stability obligations. Stage 09 Wave 091/092 subsequently closed perf/ops archival and final release-ledger synchronization.
