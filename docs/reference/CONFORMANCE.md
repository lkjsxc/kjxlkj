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
- TODO ledgers are doc-link-driven for reconstruction.
- top-level start-gate rows are complete.
- Runtime scaffold is reconstructed (Rust app crate + TypeScript app scaffold).
- HTTP route surface is reachable with core auth/users/workspaces/projects/notes flows.
- Saved-view lifecycle endpoints are executable and deterministic-test verified.
- WebSocket endpoint is reachable with subscribe/ack/apply-patch and idempotency replay.
- WebSocket reconnect flow now supports deterministic subscribe-time `ack_cursor` replay behavior.
- Typed frontend shell includes note list, title edit propagation, autosave debounce, and save/conflict/offline status rail.
- Frontend deterministic regression tests now validate pre-auth handling, idempotency fallback, title propagation, and status-rail failure states.
- Required backend/frontend path scaffolding for final structure contract is present.
- Docker artifact gate checks are completed with deterministic proof.
- Release gate is currently blocked by missing reconstructed runtime evidence.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy set defines All in Docs and typed constraints |
| All in Docs doctrine | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | `verified` | doctrine distinguishes governance from repository shape |
| Typed language contract | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | explicit Rust + TypeScript + no direct JS rule exists |
| No direct JS runtime source | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | repository scan shows no direct `.js` runtime source |
| TODO doc-link workflow | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | `verified` | all checklist rows in `docs/todo/` contain docs links |
| Final file structure path scaffolding | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | `partial` | required backend/frontend paths are present; full behavior-complete structure contract remains open |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `partial` | startup/health/readiness runtime scaffold exists; full topology (WS/background/domain flow) remains open |
| HTTP/API reachability | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | route surface is reachable; setup/login/session, users/workspaces/projects, notes conflict and metadata-delete contracts, and saved-view CRUD lifecycle are verified by deterministic tests |
| WS protocol reachability | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | `/ws` handshake and core subscribe/apply-patch/conflict/idempotency flows are implemented; reconnect subscribe with `ack_cursor` replay is test-covered |
| Automation/librarian review flow | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `partial` | automation rule CRUD, provider/protocol validation, run launch/status, and review persistence are implemented and test-covered |
| Security enforcement (auth/session/csrf/rbac) | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `partial` | setup lockout, cookie sessions, csrf checks, and role guards are implemented; full security matrix remains open |
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `partial` | note-first shell and autosave editor flow are implemented in strict TypeScript; full UX/accessibility matrix remains open |
| Deterministic acceptance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `partial` | targeted deterministic tests pass for auth/setup, note conflict + metadata delete, ws idempotency/conflict + reconnect cursor replay, saved-view CRUD, and frontend regression slices (`REG-IMP-001`, `REG-USR-001/002/003/007/008`, `REG-UX-005`); full acceptance pack remains open |
| Type verification gates (`TYPE-01..03`) | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | `cargo check --workspace`, strict TypeScript typecheck, and no direct `.js` runtime source were verified |
| Docker artifact guidance and root presence | [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) | `verified` | root artifacts, `docker compose config`, and compose runtime smoke (`healthz`/`readyz`) passed |

## Conformance Closure Rule

No `spec-only`, `unverified`, or `blocked` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
