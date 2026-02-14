# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports currently verified behavior only.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is open |
| `partial` | behavior exists but verification is incomplete |
| `blocked` | known user-visible failure or contradiction is open |
| `unverified` | no trustworthy runtime evidence exists |
| `spec-only` | behavior is defined in spec only |

## Current Snapshot (2026-02-14)

High-confidence statement:

- All in Docs governance is active and canonical.
- TODO/wave ledgers are reset and now doc-link-driven for reconstruction.
- Runtime source was intentionally deleted during reset and must be rebuilt.
- Release gate is currently blocked by missing reconstructed runtime evidence.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy set defines All in Docs and typed constraints |
| All in Docs doctrine | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | `verified` | doctrine distinguishes governance from repository shape |
| Typed language contract | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | explicit Rust + TypeScript + no direct JS rule exists |
| No direct JS runtime source | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | repository scan shows no direct `.js` runtime source |
| TODO doc-link workflow | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | `verified` | all checklist rows in `docs/todo/` contain docs links |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `blocked` | runtime source tree absent after reset |
| HTTP/API reachability | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | implementation and runtime evidence pending |
| WS protocol reachability | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | implementation and runtime evidence pending |
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `spec-only` | TypeScript web app source and verification pending |
| Deterministic acceptance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `unverified` | acceptance suites are not currently runnable |
| Docker artifact guidance and root presence | [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) | `partial` | root artifacts exist, runtime smoke must be re-established |

## Conformance Closure Rule

No `spec-only`, `unverified`, or `blocked` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
