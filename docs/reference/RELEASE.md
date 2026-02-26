# Release Gate

**Back:** [Reference Root](/docs/reference/README.md)

---

## Purpose

Defines mandatory criteria for declaring release-ready state.

---

## Current State (Docs-Only Baseline)

**Status:** `NOT READY`

**Primary blockers:**

1. runtime has been intentionally removed and must be reconstructed
2. acceptance and supplemental suites are not yet executable in docs-only state
3. communication and frontend closure are pending reconstruction waves
4. release gates require end-to-end evidence after rebuild

---

## Release Criteria

All criteria MUST be true.

### Documentation Gate

- [x] canonical docs are present and cross-linked
- [x] TODO list links directly to governing docs
- [x] implementation logs remain prohibited (`docs/logs/` absent)
- [x] reference ledgers include active drift and conformance snapshots

### Build and Runtime Gate

- [ ] `cargo build --workspace` passes
- [ ] `cargo test --workspace` passes
- [ ] `docker compose up -d --build` succeeds with healthy services
- [ ] `GET /api/healthz` is reachable and deterministic

### Contract Gate

- [ ] HTTP contract passes per [http.md](/docs/spec/api/http.md)
- [ ] WebSocket contract passes per [websocket.md](/docs/spec/api/websocket.md)
- [ ] error envelope contract is deterministic per [errors.md](/docs/spec/api/errors.md)
- [ ] auth/session/csrf rules pass per [/docs/spec/security/README.md](/docs/spec/security/README.md)

### Frontend and UX Gate

- [ ] root URL flow matches [web-app.md](/docs/spec/ui/web-app.md)
- [ ] editor behavior matches [editor-flow.md](/docs/spec/ui/editor-flow.md)
- [ ] responsive behavior matches [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [ ] compact threshold (`<=1280px`) behavior passes (`E2E-25`)
- [ ] `320px` usability/no horizontal scroll passes (`E2E-19`)

### Test and Evidence Gate

- [ ] all acceptance rows are `pass` in [TEST_MATRIX.md](TEST_MATRIX.md)
- [ ] supplemental suites are `pass` in [TEST_MATRIX.md](TEST_MATRIX.md)
- [ ] all required TODO rows are closed in [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)
- [ ] each completed TODO checkbox has explicit evidence linkage in [EVIDENCE_INDEX.md](EVIDENCE_INDEX.md)
- [ ] per-checkbox closure rule from [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) is satisfied

### Hygiene Gate

- [ ] no open `M1/M2/M3` drifts for release scope in [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- [ ] no release-blocking rows in [LIMITATIONS.md](LIMITATIONS.md)
- [ ] root prohibitions hold (`tmp/`, `log/`, `docs/logs/` absent)

---

## Stage Gate Summary

| Stage | Gate | Status |
|-------|------|--------|
| `S00` | governance normalization | `in-progress` |
| `S01` | runtime skeleton | `pending` |
| `S02` | notes/search core | `pending` |
| `S03` | runtime integration | `pending` |
| `S04` | automation/agent | `pending` |
| `S05` | auth/security | `pending` |
| `S06` | REST closure | `pending` |
| `S07` | WebSocket closure | `pending` |
| `S08` | frontend closure | `pending` |
| `S09` | CI/performance/evidence | `pending` |
| `S10` | hardening | `pending` |

---

## Related

- [TODO Contract](/docs/todo/README.md) — execution order
- [CI Profiles](CI.md) — verification profiles
- [Conformance](CONFORMANCE.md) — state ledger
- [TODO Trace Matrix](TODO_TRACE_MATRIX.md) — trace closure
- [Test Matrix](TEST_MATRIX.md) — acceptance closure
