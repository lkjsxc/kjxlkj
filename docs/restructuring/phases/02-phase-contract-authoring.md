# Phase 02 — Phase Contract Authoring

## Objective

Publish a deterministic 00..09 phase set with consistent section semantics.

## Inputs

- [phases TOC](README.md)
- Template constraints from [Phase 01](01-topology-and-toc.md)

## Ordered Steps

1. Define phase filenames with zero-padded numeric prefixes (`00`..`09`).
2. Use the same six sections in every phase: objective, inputs, ordered steps, interleaved tests, fundamental intent, evidence.
3. Ensure all phase steps are actionable and explicitly ordered.

## Interleaved Tests

- `T02-phase-order` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T03-section-shape` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-01](../tests/fundamental-intent-catalog.md#fi-01-ordering-is-deterministic) and [FI-02](../tests/fundamental-intent-catalog.md#fi-02-phase-shape-is-normalized) constrain phase structure.

## Evidence

- All ten phase files linked by [phases README](README.md).
- Section headers are identical across phase docs.
