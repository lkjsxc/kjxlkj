# TODO Management

Back: [/docs/README.md](/docs/README.md)

This directory is the reconstruction control plane.

## Operating Rules

- Every item MUST use checkbox form.
- All items remain unchecked until deterministic evidence exists.
- No checkbox may be checked from type-only or unreachable implementation work.
- TODOs must remain stricter than MVP: target full feature wiring.

## Documents

| Document | Purpose |
|---|---|
| [current/README.md](current/README.md) | Active reconstruction checklists |
| [doc-coverage/README.md](doc-coverage/README.md) | Direct-link checklist for every documentation file |
| [RECONSTRUCTION_PROMPT.md](RECONSTRUCTION_PROMPT.md) | Rebuild execution contract |
| [reading/README.md](reading/README.md) | Reading discipline before implementation |
| [completed/README.md](completed/README.md) | Optional archive for completed waves |

## Required Gate Sequence

1. Read canonical docs.
2. Build mismatch matrix.
3. Select one coherent reconstruction slice.
4. Implement via user-reachable paths.
5. Run deterministic tests.
6. Update conformance and limitations.
7. Check only proven TODOs.

## Anti-Shortcut Policy

| Shortcut Pattern | Disallowed Outcome |
|---|---|
| Type scaffolding without runtime path | Cannot mark feature complete |
| Passing only unit tests for UI/runtime behavior | Cannot mark E2E-related item complete |
| Updating spec without limitations/conformance sync | Cannot mark item complete |
| Ignoring known boundary behavior | Cannot mark parent area complete |

## Related

- Policy: [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
