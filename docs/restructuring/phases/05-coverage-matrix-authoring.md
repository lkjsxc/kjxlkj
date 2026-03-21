# Phase 05 — Coverage Matrix Authoring

## Objective

Build a complete matrix that directly references every markdown file under `docs/`.

## Inputs

- [coverage/matrix.md](../coverage/matrix.md)
- Current docs file inventory from [Phase 00](00-baseline-and-scope.md)

## Ordered Steps

1. Collect `docs/**/*.md` in lexical order.
2. Publish one matrix row per file with a direct markdown link target.
3. Verify no markdown file is omitted or duplicated in inventory coverage.

## Interleaved Tests

- `T08-coverage-completeness` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T09-link-target-exists` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-06](../tests/fundamental-intent-catalog.md#fi-06-coverage-matrix-covers-all-docs-markdown) requires total docs reference coverage.

## Evidence

- Matrix row count equals markdown file count under `docs/`.
- Every matrix link resolves during link validation.
