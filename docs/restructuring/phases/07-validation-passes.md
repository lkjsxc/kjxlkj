# Phase 07 — Validation Passes

## Objective

Run deterministic validation checks for topology, line limits, and link sanity.

## Inputs

- All authored files from [Phases 00-06](README.md)
- Validation requirements from restructuring request

## Ordered Steps

1. Run topology check: every `docs/**/` directory contains `README.md`.
2. Run line-limit check: every markdown file under `docs/` has fewer than 300 lines.
3. Run link sanity checks focused on README and restructuring paths.

## Interleaved Tests

- `T12-topology-check` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T13-line-limit-check` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T14-link-sanity` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-08](../tests/fundamental-intent-catalog.md#fi-08-validation-is-explicit-and-repeatable) enforces repeatable quality gates.

## Evidence

- Validation command output captured in task report.
- No failing checks in topology, line-limit, or link sanity.
