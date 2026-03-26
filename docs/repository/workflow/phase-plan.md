# Phase Plan Contract

## Order

1. Update the docs canon.
2. Refactor shared layout and route surfaces.
3. Replace or simplify client-side editor behavior.
4. Update tests and compose verification.
5. Commit only after the active batch passes its gates.

## Intent

- Keep documentation decisions ahead of code changes.
- Prefer small coherent batches over one massive unverified diff.
- Treat compose verification as the acceptance boundary for each major batch.
