# Phase 06 — Cross-Link Hardening

## Objective

Ensure restructuring docs are discoverable from root and docs entrypoints.

## Inputs

- [root README](../../../README.md)
- [docs/README.md](../../README.md)
- [restructuring README](../README.md)

## Ordered Steps

1. Add restructuring references to root quick-navigation links.
2. Add restructuring references to docs top-level TOC and reading order.
3. Check that restructuring TOCs form a closed internal navigation loop.

## Interleaved Tests

- `T10-root-link` from [interleaved schedule](../tests/interleaved-schedule.md)
- `T11-docs-link` from [interleaved schedule](../tests/interleaved-schedule.md)

## Fundamental Intent

- [FI-07](../tests/fundamental-intent-catalog.md#fi-07-entrypoints-link-to-restructuring-docs) guarantees entrypoint visibility.

## Evidence

- Updated link bullets in root and docs readmes.
- Link checker reports no broken restructuring links.
