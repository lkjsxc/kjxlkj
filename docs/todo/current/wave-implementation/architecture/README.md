# Implementation: Architecture (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement the runtime model and crate topology that everything else depends on:

- task boundaries (host/core/render/services)
- message bus/event model
- service supervision and IO isolation
- crate layout and dependency structure

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Crate topology | [crates/README.md](crates/README.md) |
| Runtime ordering | [runtime/README.md](runtime/README.md) |
| Plugin model (no plugins) | [plugins/README.md](plugins/README.md) |

## Read first (direct, normative)

- Spec:
  - [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
  - [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
  - [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
  - [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)
- Policy and constraints:
  - [/docs/policy/README.md](/docs/policy/README.md)
  - [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
  - [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)

## Coverage traversal (to avoid missing docs)

- Follow the doc coverage subtree:
  - [/docs/todo/doc-coverage/spec/architecture/README.md](/docs/todo/doc-coverage/spec/architecture/README.md)

## Placeholder scaffolding (sub-wave)

- [ ] Reconcile the crate layout expectation vs current workspace layout.
  - If docs and code disagree, record a single canonical decision in `/docs/reference/IMPLEMENTATION_HISTORY.md`.
- [ ] Ensure the workspace contains the crates required by the spec topology, even if some are placeholders.
  - For missing crates, create stubs that compile and are wired into the supervisor.
- [ ] Define shared core types and events required for the runtime model.

## Minimal conformance slice (sub-wave)

- [ ] Implement the smallest end-to-end “event → core → snapshot → render” loop that is:
  - deterministic
  - test-backed
  - compatible with the async-first runtime model
- [ ] Implement at least one supervised service (filesystem or terminal) end-to-end, including:
  - request/response framing
  - error surfaces visible to the editor core
  - shutdown and cleanup semantics

## Full conformance (sub-wave)

- [ ] Implement the full runtime ordering and invariants specified in `runtime.md`.
- [ ] Implement service supervision as defined by the architecture spec:
  - failure handling and recovery
  - backpressure expectations
  - core responsiveness under service load
- [ ] Ensure “no plugins” is enforced (only native features).

## Tests (normative outputs)

- [ ] Add tests that prove:
  - single-writer core task invariant
  - snapshot immutability and render isolation
  - service failure does not corrupt core state
  - shutdown order is deterministic

## Conformance and limitations (required updates)

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
