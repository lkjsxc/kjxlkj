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

## Current Snapshot (2026-02-15)

High-confidence statement:

- All in Docs governance is active and canonical.
- Stage 00 governance baseline complete.
- Stage 01 crate skeleton complete: 10 crates compile, zero warnings.
- Auth/session/setup-lock, RBAC guards, and workspace membership implemented.
- HTTP routes for health, auth, users, workspaces, projects, notes (CRUD+history).
- WebSocket session actor with heartbeat and connection upgrade.
- All source files respect 200-line layout constraint.
- Runtime not yet tested against live database (no integration test evidence).

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy set defines reconstruction boundaries and documentation precedence |
| All in Docs doctrine | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | `verified` | doctrine defines docs as canonical product value |
| TODO restructure-step workflow | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `verified` | stage/wave program reset and linked for deterministic reconstruction |
| Final completion file map | [/docs/spec/architecture/completion-file-map.md](/docs/spec/architecture/completion-file-map.md) | `verified` | canonical completion tree and per-path purpose are explicit |
| Runtime configuration split (`data/config.json` + `.env`) | [/docs/spec/architecture/configuration.md](/docs/spec/architecture/configuration.md) | `verified` | non-secret vs secret boundary is explicit and documented |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `partial` | 10 crates compile; startup sequence coded; live DB test pending |
| HTTP/API reachability | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | all route handlers compiled; acceptance tests pending |
| WS protocol reachability | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | session actor and upgrade handler compiled; replay pending |
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `spec-only` | frontend source intentionally absent |
| Mobile menu top-right and close-on-select behavior | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `spec-only` | requirement is documented; runtime implementation pending rebuild |
| Create New Note regression test | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `spec-only` | requirement is documented as `E2E-23`; runtime test implementation pending rebuild |
| Release gate | [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) | `blocked` | runtime and acceptance evidence must be reconstructed |

## Conformance Closure Rule

No row may move to `verified` without all of:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
