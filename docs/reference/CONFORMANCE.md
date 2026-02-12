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

- Web-server product contract is canonically defined in docs.
- Runtime implementation has not yet been reconstructed in this repo.
- No API/WS/runtime claims are accepted as verified yet.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance pivot | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | docs rewritten and internally linked |
| API v1 contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | no runtime endpoints verified |
| WS patch protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | no runtime WS evidence |
| Event sourcing and projections | [/docs/spec/domain/events.md](/docs/spec/domain/events.md) | `spec-only` | schema and write-path not yet verified |
| Auth/session/CSRF | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `spec-only` | auth runtime not yet verified |
| Single-container deployment | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | `unverified` | Docker artifacts not yet reconstructed |
| Testing/performance gates | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `partial` | acceptance matrix documented only |

## Conformance Closure Rule

No `spec-only` or `unverified` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented APIs
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
