# Wave Program

Back: [/docs/todo/README.md](/docs/todo/README.md)

Ordered implementation workflow for reconstruction.

## Relevant Documents

- [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/reference/CI.md](/docs/reference/CI.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md)

## Execution Rules

- [ ] execute stages only in listed order per [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- [ ] before closing each wave, run build and test gates from [/docs/reference/CI.md](/docs/reference/CI.md)
- [ ] verify touched requirements in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] sync ledgers after each stage in [/docs/reference/README.md](/docs/reference/README.md)

## Ordered Stages

- [ ] `S00` governance baseline in [/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md)
- [ ] `S01` runtime scaffold in [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)
- [ ] `S02` notes and hybrid search in [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)
- [ ] `S03` editor and responsive UI in [/docs/todo/waves/stage-03-runtime-integration/README.md](/docs/todo/waves/stage-03-runtime-integration/README.md)
- [ ] `S04` automation and `kjxlkj-agent` in [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)
- [ ] `S05` security/reliability in [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md)
- [ ] `S06` API and protocol closure in [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)
- [ ] `S07` realtime sync closure in [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)
- [ ] `S08` frontend completion in [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)
- [ ] `S09` CI and release closure in [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)
- [ ] `S10` hardening backlog in [/docs/todo/waves/stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md)

## Exit

- [ ] no high-severity limitation rows remain in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] no open `M1`/`M2` blockers remain in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] release gate is green in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
