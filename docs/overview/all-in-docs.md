# All in Docs

Back: [/docs/overview/README.md](/docs/overview/README.md)

`kjxlkj` is documentation-first by contract.

## Product Contract

- Canonical behavior is defined under `/docs` before implementation.
- Build artifacts and source code are derived outputs.
- Reconstruction MUST be possible from canonical docs plus deterministic tooling.
- Behavior changes MUST update policy/spec/reference/todo in one logical change.

## Scope Pivot

The current canonical product is a web application server for notes and records.
Legacy terminal-editor behavior is out of scope.

## Evidence Rule

No feature is complete unless:

1. behavior is defined in spec
2. implementation path is user-reachable
3. deterministic tests pass
4. reference and TODO ledgers are synchronized

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Spec: [/docs/spec/README.md](/docs/spec/README.md)
- Reference: [/docs/reference/README.md](/docs/reference/README.md)
- TODO: [/docs/todo/README.md](/docs/todo/README.md)
