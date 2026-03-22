# Phase 07 — Validation Passes

## Objective

Run deterministic CLI validation checks required for docs/runtime convergence replay.

## Inputs

- All authored files from [Phases 00-06](README.md)
- Validation requirements from restructuring request

## Ordered Steps

1. Run topology gate: `cargo run --bin kjxlkj -- docs validate-topology`.
2. Run terms gate: `cargo run --bin kjxlkj -- docs validate-terms`.
3. Run line-limit gate: `cargo run --bin kjxlkj -- quality check-lines`.

## Interleaved Tests

- `T12-topology-check` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T13-terms-check` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T14-line-limit-check` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-08](../tests/fundamental-intent-catalog.md#fi-08-validation-is-explicit-and-repeatable) enforces repeatable quality gates.

## Evidence

- Validation command output captured in task report.
- No failing checks in topology, terms, or line-limit gates.
