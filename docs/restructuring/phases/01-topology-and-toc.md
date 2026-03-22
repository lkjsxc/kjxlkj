# Phase 01 — Topology and TOC Scaffolding

## Objective

Create the restructuring directory tree with one TOC README per directory.

## Inputs

- Baseline outputs from [Phase 00](00-baseline-and-scope.md)
- [docs/repository/structure/docs-layout.md](../../repository/structure/docs-layout.md)

## Ordered Steps

1. Create `docs/restructuring/`, `phases/`, `tests/`, and `coverage/` directories.
2. Author `README.md` in each new directory as the canonical TOC for that directory.
3. Link each TOC to all immediate child docs for deterministic navigation.

## Interleaved Tests

- `T01-readme-per-dir` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T21-toc-child-links` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-00](../tests/fundamental-intent-catalog.md#fi-00-topology-contract-is-explicit) requires recursive TOC coverage.

## Evidence

- Presence of required files listed in [restructuring README](../README.md).
- Topology validation output from final checks.
