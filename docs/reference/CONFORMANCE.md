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
- Runtime implementation is reconstructed and reachable in a single-container deployment.
- Acceptance-tier verification is still incomplete; final release status remains partial.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance pivot | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | docs rewritten and internally linked |
| API v1 contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | setup/login/notes/conflict/metadata/tags/search/attachments smoke-verified on live container |
| WS patch protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | WS `subscribe_note` + stale `patch_rejected` + `ack` replay cursor flow verified via Node WS smoke |
| Event sourcing and projections | [/docs/spec/domain/events.md](/docs/spec/domain/events.md) | `partial` | transactional event+projection writes implemented; snapshot table and 100-version snapshot logic added |
| Auth/session/CSRF | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `partial` | setup/login/logout/session + CSRF enforcement + in-memory auth rate limit implemented |
| Single-container deployment | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | `verified` | `docker compose build` + `docker compose up -d` + `/api/v1/readyz` pass |
| Frontend static hosting | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `verified` | React/Vite SPA built in Docker image and served by Actix static route |
| Testing/performance gates | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `partial` | T0 unit tests pass; DB-backed tests present but require `DATABASE_URL`; T2/perf not yet closed |

## Conformance Closure Rule

No `spec-only` or `unverified` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented APIs
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
