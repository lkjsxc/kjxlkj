# Conformance

**Back:** [Reference Root](/docs/reference/README.md)

---

## Status Vocabulary

| Status | Meaning |
|--------|---------|
| `verified` | Deterministic evidence exists |
| `partial` | Behavior exists but evidence or contract coverage is incomplete |
| `spec-only` | Specified but not implemented in active runtime |
| `blocked` | Contradicted or impossible in current state |

---

## Current Snapshot (Docs-Only Baseline)

**Repository state (2026-02-26):** Reconstructable runtime artifacts were removed to enforce docs-only reset state.

- ✅ canonical docs and TODO program remain
- ✅ runtime/build artifacts intended for reconstruction were deleted
- ✅ implementation logs remain prohibited and absent
- ✅ strict checkbox-level evidence policy is active
- ⏳ runtime behavior is intentionally deferred to reconstruction waves

Legacy implementation context is archived in [LEGACY_RUNTIME_SNAPSHOT.md](LEGACY_RUNTIME_SNAPSHOT.md).

---

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|--------|----------------|--------|----------|
| Docs governance | [Policy Root](/docs/policy/README.md) | `verified` | policy and TODO invariants |
| Architecture sequencing | [/docs/spec/architecture/BUILD_SEQUENCE.md](/docs/spec/architecture/BUILD_SEQUENCE.md) | `verified` | deterministic stage order defined |
| HTTP contract | [HTTP Spec](/docs/spec/api/http.md) | `verified` | normative contract and test ownership defined |
| WebSocket contract | [WS Spec](/docs/spec/api/websocket.md) | `verified` | normative contract and replay semantics defined |
| Auth/session/csrf | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `verified` | deterministic security contract defined |
| Frontend architecture contract | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | `verified` | deep target tree and communication modules defined |
| Testing contract | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `verified` | mandatory acceptance and suite categories defined |
| Runtime implementation | [/docs/todo/README.md](/docs/todo/README.md) | `spec-only` | pending rebuild by waves |

---

## Acceptance Coverage State

All acceptance IDs are specified and mapped; runtime evidence is intentionally pending reconstruction.

Source of truth:

- [TEST_MATRIX.md](TEST_MATRIX.md)
- [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)
- [EVIDENCE_INDEX.md](EVIDENCE_INDEX.md)

---

## Closure Rule

No row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability
3. synchronized reference updates (`CONFORMANCE.md`, `LIMITATIONS.md`, `DRIFT_MATRIX.md`)
4. TODO completion with linked proofs
5. trace and test matrix closure

---

## Related

- [LEGACY_RUNTIME_SNAPSHOT.md](LEGACY_RUNTIME_SNAPSHOT.md) — archived pre-reset runtime state
- [Limitations](LIMITATIONS.md) — open gaps
- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Contract](/docs/todo/README.md) — execution order
- [Release Gate](RELEASE.md) — release criteria
