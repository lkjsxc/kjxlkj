# Dependency Rules

## Dependency Layers

1. Orientation contracts
2. Program control contracts
3. Stage-level contracts
4. Wave-level execution contracts
5. Checkpoint and evidence contracts

## Mandatory Dependencies

- Every wave depends on its stage objective and stage exit criteria docs.
- Every stage depends on sequencing and risk model docs.
- Final acceptance depends on checkpoints and evidence closure.

## Cross-Doc Dependency Rule

When a restructuring file changes semantics, update all inbound TOCs and references in the same change set.

## Blocking Policy

Missing dependency inputs convert wave status to `blocked`; do not mark `complete` with inferred assumptions.
