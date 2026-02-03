# Current TODO (Iteration 33)

Back: [/docs/todo/README.md](/docs/todo/README.md)

## Purpose

This TODO is the execution entrypoint for turning the documents into a complete, correct implementation.

It is:

- Recursive and hierarchical (parent checklists link to child checklists)
- Wave-based (placeholders → progressively more detailed → full implementation)
- Doc-driven (behavior is derived from documents; code follows docs)

## Local rules (normative)

- This iteration MUST be executed in wave order.
- Work MUST be decomposed into leaf checklists that are:
  - small enough to complete
  - each gated by tests and conformance updates where applicable
- Directory and file names under this TODO MUST NOT contain digits.
- The second-to-last wave MUST be “Recreate the TODO list”.
- The last wave MUST be “Continue to the next iteration”.

## TODO List

### Wave: Placeholder scaffolding (fast, incomplete by design)

- [wave-placeholder/README.md](wave-placeholder/README.md)

### Wave: Read and reconcile all documents (doc-complete)

- [wave-reading/README.md](wave-reading/README.md)

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
