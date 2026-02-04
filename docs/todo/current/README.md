# Current TODO (Iteration 34)

Back: [/docs/todo/README.md](/docs/todo/README.md)

## Purpose

This TODO is the execution entrypoint for turning the documents into a complete, correct implementation.

Contract reference: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)

It is:

- Recursive and hierarchical (parent checklists link to child checklists)
- Wave-based (placeholders → progressively more detailed → full implementation)
- Doc-driven (behavior is derived from documents; code follows docs)

## Local rules (normative)

- This iteration MUST be executed in wave order.
- Work MUST be decomposed into leaf checklists that are:
  - small enough to complete
  - each gated by tests and conformance updates where applicable
- Checkboxes MUST be meaningful:
  - `- [ ]` means not complete (or not yet verified)
  - `- [x]` means complete and verified against the appropriate gate (tests, link checks, policy checks)
- Directory and file names under this TODO MUST NOT contain digits.
- The second-to-last wave MUST be “Recreate the TODO list”.
- The last wave MUST be “Continue to the next iteration”.

## Completion gate (normative)

Before considering this iteration complete:

1. Ensure no unchecked checklist items remain in the current iteration by running `rg -n '^\s*-\s+\[\s\]' docs/todo/current`.
2. Ensure documentation policy is satisfied (see [/docs/policy/README.md](/docs/policy/README.md)).
3. Ensure the verification gate is green for the reconstructed implementation (see [/docs/reference/CI.md](/docs/reference/CI.md)).

## TODO List

### Wave: Placeholder scaffolding (fast, incomplete by design)

- [wave-placeholder/README.md](wave-placeholder/README.md)

### Wave: Reconstruction runbook (docs-only → full repo)

- [wave-reconstruction/README.md](wave-reconstruction/README.md)

### Wave: Read and reconcile all documents (doc-complete)

- [wave-reading/README.md](wave-reading/README.md)

### Wave: Repair docs and the TODO system (make drift explicit)

- [wave-docs/README.md](wave-docs/README.md)

### Wave: Convert documents into a complete plan (fill placeholders)

- [wave-planning/README.md](wave-planning/README.md)

### Wave: Implement the planned surface (placeholders → full behavior)

- [wave-implementation/README.md](wave-implementation/README.md)

### Wave: Verify conformance (tests, policy checks, perf baselines)

- [wave-verification/README.md](wave-verification/README.md)

### Wave: Recreate the TODO list (second-to-last)

- [wave-recursion/recreate-todo/README.md](wave-recursion/recreate-todo/README.md)

### Wave: Continue to the next iteration (last)

- [wave-recursion/next-iteration/README.md](wave-recursion/next-iteration/README.md)
