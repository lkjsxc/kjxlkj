# Phase Plan Contract

## Order

1. Read the relevant canon in `docs/`.
2. Update the docs canon if behavior or structure must change.
3. Update code, tooling, or topology to satisfy the canon.
4. Run the relevant cargo gates and, when acceptance scope is affected, the compose verification bundle.
5. Commit the verified batch.

## Intent

- Keep documentation decisions ahead of code changes.
- Prefer small coherent batches over one massive unverified diff.
- Treat compose verification as the acceptance boundary for each major batch.
