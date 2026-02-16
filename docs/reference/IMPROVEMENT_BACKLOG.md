# Improvement Backlog

Back: [/docs/reference/README.md](/docs/reference/README.md)

Canonical backlog for the next full rebuild.

## Governance

- This is the only durable source for improvement ideas.
- Entries below were harvested from the prior reconstruction logs.
- `docs/logs/` is non-canonical and has been removed after this capture.
- Each row MUST map to a TODO wave before implementation starts.

## Backlog Matrix

| Backlog ID | Improvement | Canonical Docs | Proposed Wave | Status |
|---|---|---|---|---|
| `IMP-ARC-01` | Enable SQLx compile-time query checking with offline cache | [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) | `S10/W100` | `open` |
| `IMP-ARC-02` | Add cross-session WebSocket broadcast registry | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `S07/W071` | `done` |
| `IMP-ARC-03` | Tune DB pool (`max_connections`, timeout policy) from measured load | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | `S09/W090` | `done` |
| `IMP-FE-01` | Split monolithic UI shell into focused components | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `S08/W080` | `done` |
| `IMP-FE-02` | Integrate rich markdown editor (CodeMirror/ProseMirror class) | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `S08/W081` | `done` |
| `IMP-FE-03` | Add offline/PWA support with local cache and reconnect sync | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | `S08/W082` | `done` |
| `IMP-TEST-01` | Add property-based tests for domain invariants | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S10/W101` | `done` |
| `IMP-TEST-02` | Add snapshot tests for API contract stability | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S10/W101` | `done` |
| `IMP-TEST-03` | Build DB-backed integration harness (containerless or ephemeral DB) | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S09/W090` | `open` |
| `IMP-SEC-01` | Add CSP nonce strategy for script execution hardening | [/docs/spec/security/transport.md](/docs/spec/security/transport.md) | `S05/W051` | `done` |
| `IMP-SEC-02` | Add auth endpoint rate limiting for brute-force resistance | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | `S05/W050` | `done` |
| `IMP-SEC-03` | Broadcast session revocation events on credential reset | [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md) | `S07/W072` | `done` |
| `IMP-OPS-01` | Replace ad-hoc prints with structured tracing and spans | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | `S10/W100` | `done` |
| `IMP-OPS-02` | Expose metrics endpoint for request/latency/pool telemetry | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | `S09/W090` | `done` |
| `IMP-OPS-03` | Implement graceful shutdown with in-flight drain handling | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | `S10/W100` | `done` |
| `IMP-STRUCT-01` | Split >200-line runtime modules during reconstruction waves | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | `S10/W101` | `open` |

## Historical Large-File Split Targets

The last runtime reconstruction produced these high-risk files over 200 lines.
Treat these as required split targets when source code is regenerated.

| Historical Path | Prior Lines | Priority |
|---|---:|---|
| `src/frontend/app/src/components/app-shell.ts` | 422 | high |
| `src/crates/http/kjxlkj-http/src/routes_note.rs` | 306 | high |
| `src/crates/db/kjxlkj-db/src/repo_note.rs` | 302 | high |
| `src/crates/ws/kjxlkj-ws/src/session.rs` | 229 | medium |
| `src/crates/db/kjxlkj-db/src/repo_automation.rs` | 205 | medium |

## Related

- TODO execution plan: [/docs/todo/README.md](/docs/todo/README.md)
- Drift ledger: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
