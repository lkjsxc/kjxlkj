# Release Gate

**Back:** [Reference Root](/docs/reference/README.md)

---

## Purpose

Defines the gate criteria for declaring a release-ready state.

---

## Current State: Docs-Only Baseline

**Status:** 🔴 NOT READY FOR RELEASE

**Reason:** Runtime rebuild not executed yet; this repository is in docs-first reconstruction mode.

---

## Release Criteria

All criteria MUST be true for release:

### Documentation Gate
- [x] Canonical docs are present and cross-linked
- [x] TODO list reset with direct links to every doc
- [x] Reference ledgers synchronized (CONFORMANCE, LIMITATIONS, DRIFT_MATRIX)
- [x] `docs/logs/` prohibited by policy

### Runtime Gate
- [ ] Cargo workspace builds without errors (`cargo build --workspace`)
- [ ] All tests pass (`cargo test --workspace`)
- [ ] Docker Compose orchestration works (`docker compose up`)
- [ ] Health endpoint responds (`GET /api/healthz`)

### API Gate
- [ ] All HTTP endpoints functional per [http.md](/docs/spec/api/http.md)
- [ ] WebSocket protocol works per [websocket.md](/docs/spec/api/websocket.md)
- [ ] Acceptance IDs verified per [testing.md](/docs/spec/technical/testing.md)
- [ ] Deterministic error envelope and request-id propagation verified

### UX Gate
- [ ] Root URL accessible (`GET /` serves app)
- [ ] Editor works per [editor-flow.md](/docs/spec/ui/editor-flow.md)
- [ ] Layout responsive per [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [ ] 2/3 threshold (1280px) activates correctly
- [ ] 320px minimum width supported

### Agent Gate
- [ ] kjxlkj-agent loads prompts from JSON
- [ ] KV memory persists across loops
- [ ] YOLO mode can create/edit notes
- [ ] Conversation logs disabled by default

### Security Gate
- [ ] Auth/session functional per [security/README.md](/docs/spec/security/README.md)
- [ ] CSRF protection enabled
- [ ] Rate limiting active
- [ ] CSP headers set

### Performance Gate
- [ ] P95 latency targets met per [performance.md](/docs/spec/technical/performance.md)
- [ ] Search P95 < 200ms (hybrid mode)
- [ ] Editor keystroke-to-render < 16ms

### Verification Gate
- [ ] No high-severity rows in [LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] No open M1/M2 in [DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] All source files ≤200 lines per [STRUCTURE.md](/docs/policy/STRUCTURE.md)
- [ ] `tmp/`, `log/`, `docs/logs/` do NOT exist
- [ ] All TODO trace rows closed in [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)
- [ ] All acceptance rows closed in [TEST_MATRIX.md](TEST_MATRIX.md)

---

## Release Checklist

| Stage | Gate | Status | Evidence |
|-------|------|--------|----------|
| S00 | Governance | ✅ Complete | Policy docs complete |
| S01 | Runtime Skeleton | 🔴 Pending | Execute TODO |
| S02 | Notes + Search | 🔴 Pending | Execute TODO |
| S03 | Realtime | 🔴 Pending | Execute TODO |
| S04 | Agent | 🔴 Pending | Execute TODO |
| S05 | Security | 🔴 Pending | Execute TODO |
| S06 | REST API | 🔴 Pending | Execute TODO |
| S07 | WebSocket | 🔴 Pending | Execute TODO |
| S08 | Frontend | 🔴 Pending | Execute TODO |
| S09 | CI + Perf | 🔴 Pending | Execute TODO |
| S10 | Hardening | 🔴 Pending | Execute TODO |

---

## Next Steps

1. Execute TODO stages in order (S00 → S10)
2. Sync ledgers after each stage
3. Verify acceptance IDs
4. Close TODO and test matrix rows
5. Re-evaluate release gate

---

## Related

- [TODO Contract](/docs/todo/README.md) — execution order
- [CI Profiles](CI.md) — verification profiles
- [Conformance](CONFORMANCE.md) — verified state
- [TODO Trace Matrix](TODO_TRACE_MATRIX.md) — implementation trace closure
- [Test Matrix](TEST_MATRIX.md) — test coverage closure
