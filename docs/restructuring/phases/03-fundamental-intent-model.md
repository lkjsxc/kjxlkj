# Phase 03 — Fundamental Intent Model

## Objective

Define and stabilize explicit fundamental intents that each phase must preserve.

## Inputs

- [tests/fundamental-intent-catalog.md](../tests/fundamental-intent-catalog.md)
- Phase contracts from [Phase 02](02-phase-contract-authoring.md)

## Ordered Steps

1. Define intent IDs (`FI-00`..`FI-10`) with single authoritative statements.
2. Map each phase to at least one intent ID.
3. Cross-link intents into phase docs so intent validation is machine-readable.

## Interleaved Tests

- `T04-intent-catalog` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T05-phase-intent-links` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-03](../tests/fundamental-intent-catalog.md#fi-03-fundamental-intents-are-explicit) and [FI-04](../tests/fundamental-intent-catalog.md#fi-04-phase-to-intent-mapping-is-total) ensure full intent traceability.

## Evidence

- Intent catalog table includes all IDs and owners.
- Every phase references at least one `FI-*` anchor.
