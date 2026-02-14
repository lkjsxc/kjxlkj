# All in Docs

Back: [/docs/overview/README.md](/docs/overview/README.md)

`kjxlkj` uses All in Docs as a permanent doctrine.

## Doctrine

- Documentation is the product.
- Source code, binaries, and containers are derived projections.
- Derived projections may be deleted and rebuilt without losing product value.
- Authority never shifts away from `/docs`.

## Important Nuance

This doctrine is not the same as `docs-only`.

- `All in Docs`: always true governance rule.
- `docs-only`: temporary repository shape when derived artifacts are absent.

## Reconstruction Rule

A reconstruction is valid only when:

1. policy/spec requirements are implemented
2. typed language constraints are satisfied
3. deterministic tests pass
4. reference and TODO ledgers are synchronized

## Typed Language Rule

Runtime reconstruction MUST use statically typed languages only:

- frontend: TypeScript (`strict`)
- backend: Rust

Direct handwritten JavaScript source for application runtime is forbidden.
Generated bundles in `src/frontend/app/dist/` are allowed.

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Spec: [/docs/spec/README.md](/docs/spec/README.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
