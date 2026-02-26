# Known Limitations

**Back:** [Reference Root](/docs/reference/README.md)

---

## Current Snapshot (Docs-Only Rebuild Baseline)

Runtime implementation has been intentionally reset for reconstruction. These are the active gaps to close during TODO wave execution.

---

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|----|------------------|-----|-------|----------|-------------|
| `LIM-RUNTIME-ABSENT-01` | [/docs/todo/README.md](/docs/todo/README.md) | Runtime source not reconstructed yet | `M1 missing runtime` | high | Execute `S01`-`S10` in order |
| `LIM-COMM-IMPLEMENTATION-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | Communication layer contracts documented but not implemented | `M1 missing runtime` | high | Execute `S03`, `S05`, `S06`, `S07` waves |
| `LIM-TEST-EVIDENCE-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | Full `T0/T1/T2` evidence is pending runtime rebuild | `M4 verification gap` | high | Implement suites and attach evidence |
| `LIM-DB-REGEN-01` | [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) | Migrations must be regenerated from specs | `M2 missing feature` | medium | Recreate migration set during `S01` |
| `LIM-TRACE-CLOSURE-01` | [/docs/reference/TODO_TRACE_MATRIX.md](/docs/reference/TODO_TRACE_MATRIX.md) | TODO-to-implementation rows need runtime proof links | `M4 verification gap` | medium | Fill evidence columns during execution |

---

## Closed Limitations

| ID | Requirement | Closure Evidence | Closed Date |
|----|-------------|------------------|-------------|
| `LIM-DOCS-STRUCTURE-01` | Architecture docs lacked deterministic build order | `BUILD_SEQUENCE.md` added | 2026-02-25 |
| `LIM-DOCS-CROSSREF-01` | Cross-spec ownership unclear | `SPEC_INTERACTIONS.md` added | 2026-02-25 |
| `LIM-COMM-SPEC-01` | Communication contracts underspecified | API/security/testing specs rewritten | 2026-02-25 |
| `LIM-LOG-POLICY-01` | `docs/logs` policy ambiguous in ledgers | reference docs normalized to prohibition | 2026-02-25 |

---

## Specification Completeness

These specification sets are complete and ready for implementation:

| Spec Area | Status | File | Notes |
|-----------|--------|------|-------|
| Communication contracts | ✅ Complete | [/docs/spec/api/README.md](/docs/spec/api/README.md) | request-id, retry, replay semantics |
| Security contracts | ✅ Complete | [/docs/spec/security/README.md](/docs/spec/security/README.md) | deterministic auth/session/csrf |
| Testing contract | ✅ Complete | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | mandatory `T0/T1/T2` |
| Architecture sequencing | ✅ Complete | [/docs/spec/architecture/BUILD_SEQUENCE.md](/docs/spec/architecture/BUILD_SEQUENCE.md) | ordered reconstruction |
| Traceability | ✅ Complete | [/docs/reference/TODO_TRACE_MATRIX.md](/docs/reference/TODO_TRACE_MATRIX.md) | TODO-to-spec-to-artifact mapping |

---

## Limitation Classes

| Class | Description | Resolution Path |
|-------|-------------|-----------------|
| `M1 missing runtime` | Core runtime not implemented | Execute S01-S03 stages |
| `M2 missing feature` | Feature specified but not built | Execute S04-S08 stages |
| `M3 integration gap` | External integration pending | Execute S02-W022, S04 |
| `M4 verification gap` | Tests/CI not configured | Execute S09 stage |

---

## Severity Definitions

| Severity | Definition | Action Required |
|----------|------------|-----------------|
| `high` | Blocks core functionality | Must fix before next stage |
| `medium` | Degrades user experience | Must fix before release |
| `low` | Nice-to-have enhancement | Can defer to backlog |

---

## Closure Rules

A limitation closes **only when**:

1. **Behavior is runtime-reachable** — code implemented and running
2. **Deterministic tests pass** — acceptance IDs verified
3. **Drift and TODO ledgers are synchronized** — CONFORMANCE.md, DRIFT_MATRIX.md updated
4. **Evidence linked** — proof artifacts in EVIDENCE_INDEX.md

---

## Related

- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Program](/docs/todo/README.md) — rebuild execution order
- [Conformance](CONFORMANCE.md) — verified state
- [Release Gate](RELEASE.md) — release criteria
- [Test Matrix](TEST_MATRIX.md) — acceptance-to-suite mapping
