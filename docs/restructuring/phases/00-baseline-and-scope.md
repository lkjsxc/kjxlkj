# Phase 00 — Baseline and Scope

## Objective

Freeze the current documentation baseline and declare non-runtime scope for restructuring work.

## Inputs

- [docs/README.md](../../README.md)
- [root README](../../../README.md)
- Current `docs/` directory topology

## Ordered Steps

1. Record all markdown files under `docs/` in deterministic lexical order.
2. Confirm restructuring scope is documentation-only and excludes runtime code changes.
3. Publish baseline assumptions that later phases can reference without reinterpretation.

## Interleaved Tests

- `T00-topology-baseline` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T01-readme-per-dir` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-00](../tests/fundamental-intent-catalog.md#fi-00-topology-contract-is-explicit) and [FI-01](../tests/fundamental-intent-catalog.md#fi-01-ordering-is-deterministic) define a stable baseline and deterministic order.

## Evidence

- Snapshot committed through [coverage matrix](../coverage/matrix.md).
- Scope statement present in [restructuring TOC](../README.md).
