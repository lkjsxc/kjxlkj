# Wave: Reconstruction Runbook (Iteration 33)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Enable a one-shot “docs-only → full repository” reconstruction run.

This wave exists because this project follows “All in Docs”: the implementation may be deleted and rebuilt from `/docs/` without losing knowledge.

## Entry points

| Document | Why it matters |
|----------|----------------|
| [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md) | The pasteable one-shot prompt (Copilot/Claude) |
| [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md) | The crate/workspace topology to recreate |
| [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | The runtime ordering model to implement |
| [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) | The “what exists” ledger (must be kept accurate) |

## Checklist (normative)

### A. Define the reconstruction target

- [x] Decide the reconstruction target:
  - full target spec, or
  - “current surface” as documented in `/docs/reference/CONFORMANCE.md`
- [x] Record the chosen target explicitly in:
  - `/docs/reference/IMPLEMENTATION_HISTORY.md` (append an entry for the reconstruction run)

### B. Bootstrap required root artifacts

- [x] Recreate the Cargo workspace and crate layout described in:
  - `/docs/spec/architecture/crates.md`
- [x] Ensure `cargo test` can run early (even with placeholders) so later work is continuously verifiable.

### C. Make reconstruction deterministic

- [x] Add a headless/E2E-capable test harness early.
- [x] Require tests for every new observable behavior before marking an item complete.
- [x] Keep `/docs/reference/CONFORMANCE.md` and `/docs/reference/LIMITATIONS.md` correct as the surface changes.

### D. Close the loop

- [x] Ensure the documentation remains sufficient to reconstruct the rebuilt repository again.
- [x] Update `/docs/todo/RECONSTRUCTION_PROMPT.md` if the workflow or constraints changed.

## Related

- All-in-Docs contract: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)
- Doc coverage traversal: [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
