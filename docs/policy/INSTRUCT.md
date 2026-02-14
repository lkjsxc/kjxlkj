# Operating Contract

Back: [/docs/policy/README.md](/docs/policy/README.md)

Hard rules for reconstruction sessions.

## Hard Rules

- Follow precedence from [/docs/README.md](/docs/README.md).
- Treat `/docs/spec/` as target behavior contract.
- Treat `/docs/reference/` as verified state contract.
- Do not mark work complete without deterministic evidence.
- Keep policy/spec/reference/todo synchronized for every status change.
- Keep changes in small, coherent commits.

## Typed Runtime Rule

When runtime artifacts are produced:

- frontend application code MUST be TypeScript (`.ts`/`.tsx`) with `strict` enabled
- backend application code MUST be Rust
- handwritten JavaScript runtime source (`.js` business logic) MUST NOT be committed
- generated frontend bundles under `src/frontend/app/dist/` are allowed

## Prohibited Outcomes

- evidence-free completion claims
- stale conformance claims contradicting limitations/drift ledgers
- TODO checkboxes marked complete without proof
- unsynchronized ledgers after behavioral changes
- untyped runtime source in reconstruction snapshots

## Completion Minimum

A feature is complete only when all are true:

1. behavior matches linked spec requirements
2. runtime path is user-reachable
3. deterministic tests cover required acceptance IDs
4. reference and TODO ledgers are updated in the same change

## Related

- Workflow gates: [WORKFLOW.md](WORKFLOW.md)
- Structure constraints: [STRUCTURE.md](STRUCTURE.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
