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

## Current Snapshot (2026-02-13)

High-confidence statement:

- All in Docs governance is active and canonical.
- TODO/wave ledgers are in completed state with linked wave evidence.
- Docker app-runtime artifacts are present and synchronized with guidance.
- Runtime conformance is partially re-established (startup and health checks verified).

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy set defines All in Docs and typed constraints |
| All in Docs doctrine | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | `verified` | doctrine distinguishes governance from repository shape |
| Typed language contract | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | explicit Rust + TypeScript + no direct JS rule exists |
| No direct JS runtime source | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | repository scan shows no `.js` runtime source files |
| Docker artifact guidance and root presence | [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) | `verified` | root Docker artifacts exist, app container builds, and service reaches healthy state |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `partial` | runtime startup and DB migration path verified through container launch |
| HTTP/API reachability | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | `/api/healthz`, `/api/readyz`, and `/api/setup/register` verified via Docker smoke |
| WS protocol reachability | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | runtime WS path not currently verified in baseline |
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `spec-only` | TypeScript runtime evidence pending |
| Deterministic acceptance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `unverified` | required acceptance suites not currently passing in baseline |

## Conformance Closure Rule

No `spec-only` or `unverified` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reset audit: [/docs/log/audits/2026-02-13-all-in-docs-baseline-reset.md](/docs/log/audits/2026-02-13-all-in-docs-baseline-reset.md)
