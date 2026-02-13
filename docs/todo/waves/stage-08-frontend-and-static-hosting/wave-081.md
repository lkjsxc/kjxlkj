# Wave 081: Operation Diff Review and Apply Controls

Back: [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] show per-operation diffs for create/rewrite/retitle/relink actions
- [x] support accept/reject decisions with deterministic audit linkage
- [x] prevent apply when active editor has unresolved local draft conflicts

## Verification Tasks

- [x] run apply/reject race-condition and stale-version scenarios
- [x] run audit trail visibility checks

## Evidence Placeholder

- [x] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test automation_run_flow --test ws_flow -- --nocapture`
- [x] `Result:` pass
- [x] `Proof:` run review endpoint validates decisions deterministically, records `automation_run_reviewed` workspace events, and enforces UI-side unresolved-draft apply guard behavior
