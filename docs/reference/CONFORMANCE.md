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

- Runtime and frontend reconstruction artifacts are present in workspace.
- Canonical API/WS paths are unversioned (`/api`, `/ws`).
- Release gate remains open pending full acceptance/performance/ops evidence.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | docs-first rules and execution policy are present |
| API contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | runtime routes reachable in container smoke (`/api/readyz`, auth, notes, workspace suite endpoints) |
| WS protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | `GET /ws` websocket upgrade returns `101` in smoke run |
| Domain model | [/docs/spec/domain/README.md](/docs/spec/domain/README.md) | `partial` | reconstructed schema/services for users/workspaces/projects/notes/views/automation |
| UI/UX contract | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | `partial` | React/Vite workspace shell rebuilt with autosave + command palette + responsive panes |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `partial` | Cargo workspace + Actix server + SQL migrations + container runtime restored |
| Testing/performance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `partial` | compile/tests + container smoke evidence present; full acceptance/perf/ops suites pending |

## Conformance Closure Rule

No `spec-only` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented APIs
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Findings audit: [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md)
