# Operating Contract

Back: [/docs/policy/README.md](/docs/policy/README.md)

Hard rules for reconstruction sessions.

## Hard Rules

- Follow precedence from [/docs/README.md](/docs/README.md).
- Treat `/docs/spec/` as target behavior.
- Treat `/docs/reference/` as verified present state.
- Keep policy/spec/reference/todo synchronized when behavior or status changes.
- Commit in small coherent units.

## Prohibited Outcomes

- evidence-free completion claims
- TODO items checked without proof
- stale ledgers contradicting TODO or spec
- undocumented runtime behavior

## Completion Minimum

A feature is complete only when:

1. behavior matches linked spec
2. runtime path is reachable
3. deterministic tests pass
4. ledgers and TODO are updated

## Related

- Workflow: [WORKFLOW.md](WORKFLOW.md)
- Structure policy: [STRUCTURE.md](STRUCTURE.md)
