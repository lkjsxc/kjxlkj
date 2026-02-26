# Evidence Index

**Back:** [Reference Root](/docs/reference/README.md)

---

## Evidence Rules

- Evidence MUST be reproducible
- Evidence MUST reference acceptance IDs in [Testing](/docs/spec/technical/testing.md)
- TODO completion MUST NOT outpace evidence capture
- Evidence MUST include request IDs for communication-layer failures
- Evidence MUST link to both [TEST_MATRIX.md](TEST_MATRIX.md) and [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)

---

## Stage Evidence Map

| Stage | Scope | Primary TODO | Required Proof | Status |
|-------|-------|--------------|----------------|--------|
| `S00` | Governance baseline | [Stage-00](/docs/todo/waves/stage-00-pivot-governance/README.md) | Docs integrity checks | `pending` |
| `S01` | Runtime scaffold | [Stage-01](/docs/todo/waves/stage-01-spec-rebuild/README.md) | Build and type gates | `pending` |
| `S02` | Notes + search | [Stage-02](/docs/todo/waves/stage-02-workspace-bootstrap/README.md) | `API-NOTE-*`, `API-SEARCH-*` | `pending` |
| `S03` | Runtime integration | [Stage-03](/docs/todo/waves/stage-03-runtime-integration/README.md) | HTTP + WS integration core | `pending` |
| `S04` | Automation + agent | [Stage-04](/docs/todo/waves/stage-04-schema-and-projections/README.md) | `API-AUTO-*`, `AGENT-*` | `pending` |
| `S05` | Security closure | [Stage-05](/docs/todo/waves/stage-05-auth-and-security/README.md) | auth/session/csrf + rate limit | `pending` |
| `S06` | REST contract | [Stage-06](/docs/todo/waves/stage-06-rest-api/README.md) | deterministic HTTP + error semantics | `pending` |
| `S07` | WebSocket sync | [Stage-07](/docs/todo/waves/stage-07-websocket-sync/README.md) | replay/idempotency/stale cursor | `pending` |
| `S08` | Frontend + hosting | [Stage-08](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md) | E2E + build checks | `pending` |
| `S09` | CI + release | [Stage-09](/docs/todo/waves/stage-09-ci-performance-release/README.md) | full `T0/T1/T2` + matrix closure | `pending` |
| `S10` | Hardening | [Stage-10](/docs/todo/waves/stage-10-hardening-and-investigation/README.md) | Targeted hardening | `pending` |

---

## Evidence Capture Template

For each wave completion, archive:

```markdown
### Stage SX-WXX Evidence

- **Date:** ISO8601 timestamp
- **Build:** `cargo build --workspace` output
- **Test:** `cargo test --workspace` output
- **Acceptance IDs:** List of passing IDs from [Testing](/docs/spec/technical/testing.md)
- **Trace rows:** matching IDs in [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)
- **Suite rows:** matching IDs in [TEST_MATRIX.md](TEST_MATRIX.md)
- **Screenshots/Logs:** (if applicable)
```

---

## Related

- [CI Profiles](CI.md) — verification definitions
- [Release Gate](RELEASE.md) — release criteria
- [TODO Contract](/docs/todo/README.md) — execution order
- [Test Matrix](TEST_MATRIX.md) — acceptance-to-suite mapping
- [TODO Trace Matrix](TODO_TRACE_MATRIX.md) — TODO-to-artifact mapping
