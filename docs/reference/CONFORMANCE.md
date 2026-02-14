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
- Repository state is docs-only by design.
- TODO ledgers are reset to unchecked reconstruction baseline.
- Runtime claims are intentionally deferred to future reconstruction waves.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy set defines docs-only validity and reconstruction boundaries |
| All in Docs doctrine | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | `verified` | doctrine defines docs as canonical product value |
| Typed language contract | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | typed-only reconstruction requirements are explicit |
| Root docs-only layout | [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) | `verified` | root contains only canonical docs/hygiene entries |
| TODO restructure-step workflow | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `verified` | ordered stage/wave checklists define implementation-ready execution |
| JSON prompt-pack canonical files | [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) | `verified` | manifest and stage JSON prompt files are canonicalized in docs |
| Responsive split/menu UX requirements | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `verified` | desktop split-pane and compact top-left menu behavior are normatively specified |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `spec-only` | runtime artifacts are intentionally absent |
| HTTP/API reachability | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | endpoint behaviors are defined but not currently reconstructed |
| WS protocol reachability | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | protocol is defined but not currently reconstructed |
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `spec-only` | UX contracts exist; frontend runtime is absent |
| Automation/librarian runtime | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `spec-only` | pipeline and prompt contracts exist; runtime is absent |
| Deterministic acceptance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `unverified` | acceptance suites require runtime reconstruction |
| Release gate | [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) | `blocked` | release cannot close in docs-only baseline |

## Conformance Closure Rule

No row may move to `verified` without all of:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
