# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports only currently verified behavior.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is open |
| `partial` | behavior exists but verification is incomplete |
| `blocked` | known user-visible failure or contradiction is open |
| `unverified` | no trustworthy runtime evidence exists |
| `spec-only` | behavior is defined in spec only |

## Current Snapshot (2026-02-12)

High-confidence statement:

- Repository is intentionally docs-only for reconstruction.
- Runtime/source/deployment artifacts are intentionally absent.
- Contracts and execution waves are synchronized in docs.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | docs-first rules and execution policy are present |
| API contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | endpoints and OpenAPI are synchronized; runtime absent |
| WS protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | protocol includes replay and workspace streams; runtime absent |
| Domain model | [/docs/spec/domain/README.md](/docs/spec/domain/README.md) | `spec-only` | workspace/project/notes/automation docs are linked |
| UI/UX contract | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | `spec-only` | workspace suite UX contracts are specified |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `spec-only` | implementation artifacts absent in current state |
| Testing/performance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `spec-only` | acceptance pack defined; runtime evidence pending |

## Conformance Closure Rule

No `spec-only` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented APIs
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Findings audit: [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md)
