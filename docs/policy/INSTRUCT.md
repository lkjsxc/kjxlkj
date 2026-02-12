# Operating Contract

Back: [/docs/policy/README.md](/docs/policy/README.md)

This document defines hard rules for reconstruction sessions.

## Hard Rules

- Follow precedence from [/docs/README.md](/docs/README.md).
- Treat `/docs/spec/` as target behavior.
- Treat `/docs/reference/` as current verified behavior.
- Do not mark work complete without deterministic evidence.
- Keep policy/spec/reference/todo synchronized when behavior or status changes.
- Commit to git frequently in small, coherent units with clear messages.

## Prohibited Outcomes

- Evidence-free completion claims.
- Runtime behavior not reachable from documented API/WS paths.
- Stale conformance claims that contradict open limitations.
- TODO checkboxes marked complete without matching proof.

## Completion Minimum

A feature is complete only when all are true:

1. behavior matches linked spec requirements
2. runtime path is user-reachable
3. deterministic tests cover success and boundaries
4. reference ledgers are updated in the same change

## Commit Cadence

- Long-running work MUST be split into multiple commits.
- Each commit SHOULD represent one logical step (spec update, implementation unit, or verification/ledger sync).
- Destructive cleanup steps MUST be isolated in their own commit.

## Related

- Workflow gates: [WORKFLOW.md](WORKFLOW.md)
- Structure constraints: [STRUCTURE.md](STRUCTURE.md)
