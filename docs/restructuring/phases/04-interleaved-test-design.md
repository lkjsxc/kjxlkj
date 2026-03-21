# Phase 04 — Interleaved Test Design

## Objective

Define tests that run between phases instead of only at the end.

## Inputs

- [tests/interleaved-schedule.md](../tests/interleaved-schedule.md)
- Intent mappings from [Phase 03](03-fundamental-intent-model.md)

## Ordered Steps

1. Define deterministic test IDs and fixed execution points.
2. Attach every test to one or more phase transition gates.
3. Specify expected evidence for each gate to prevent ambiguous pass/fail outcomes.

## Interleaved Tests

- `T06-schedule-order` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T07-gate-evidence` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-05](../tests/fundamental-intent-catalog.md#fi-05-testing-is-interleaved-not-terminal) keeps verification incremental.

## Evidence

- Interleaved schedule defines gate order from phase 00 to 09.
- Each gate has explicit evidence artifacts.
