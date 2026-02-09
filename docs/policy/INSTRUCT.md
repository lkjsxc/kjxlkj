# Operating Contract

Back: [/docs/policy/README.md](/docs/policy/README.md)

This document defines hard rules for any reconstruction session.

## Hard Rules

- Follow precedence from [/docs/README.md](/docs/README.md).
- Treat `/docs/spec/` as target behavior.
- Treat `/docs/reference/` as current verified behavior.
- Do not mark work complete without deterministic evidence.
- Keep policy/spec/reference/todo synchronized when behavior or status changes.

## Prohibited Outcomes

- Evidence-free completion claims.
- Runtime features that are unreachable from real user input.
- Stale conformance claims that contradict limitations.
- TODO checkboxes marked complete without matching proof.

## Completion Minimum

A feature is complete only when all are true:

1. Behavior matches linked spec requirements.
2. Runtime path is user-reachable.
3. Deterministic tests cover success and boundaries.
4. Reference ledgers are updated in the same change.

## Related

- Workflow gates: [WORKFLOW.md](WORKFLOW.md)
- Structure constraints: [STRUCTURE.md](STRUCTURE.md)
