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

## Current Snapshot (2025-01-20)

High-confidence statement:

- All in Docs governance is active and canonical.
- Runtime reconstruction stages 00–09 are complete.
- Rust workspace compiles with zero warnings.
- TypeScript frontend compiles with strict mode, zero errors.
- Vite production build succeeds.
- CI workflow defined with verification profiles.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy set defines docs-only validity and reconstruction boundaries |
| All in Docs doctrine | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | `verified` | doctrine defines docs as canonical product value |
| Typed language contract | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | cargo check + tsc --noEmit pass with zero errors |
| Root docs-only layout | [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) | `verified` | root contains canonical docs + derived runtime artifacts |
| TODO restructure-step workflow | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `verified` | all stage/wave checklists complete (S00–S09) |
| JSON prompt-pack canonical files | [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) | `verified` | manifest and stage JSON prompt files canonicalized; prompts.rs loads via include_str! |
| Responsive split/menu UX requirements | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `verified` | desktop split-pane and compact top-left menu implemented in NotesLayout |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `verified` | 10-crate Rust workspace with actix-web server, PgPool, migrations |
| HTTP/API reachability | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `verified` | full route table implemented in routes.rs matching spec |
| WS protocol reachability | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `verified` | WsSession actor with subscribe/patch/ack/presence/heartbeat |
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `verified` | React SPA with strict TypeScript, command palette, librarian review |
| Automation/librarian runtime | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `verified` | runner state machine, xml_attrless parser, provider adapters, prompt loading |
| Deterministic acceptance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `partial` | type gates pass; runtime acceptance pending integration test infrastructure |
| Release gate | [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) | `partial` | all code implemented; integration test evidence pending |

## Conformance Closure Rule

No row may move to `verified` without all of:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
