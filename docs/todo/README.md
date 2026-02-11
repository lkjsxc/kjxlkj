# TODO Management

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the reconstruction control plane.

## Objective

Executing TODO items exactly must produce an implementation that conforms to
policy and spec, with blocker-first verification.

## Normative Inputs

- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/reference/README.md](/docs/reference/README.md)

## Non-Negotiable Rules

- [x] no checkbox is marked complete without deterministic evidence
- [x] no unreachable or stub-only path is treated as complete
- [x] every blocker closure updates conformance, limitations, and drift matrix
- [x] every closure references at least one requirement ID and one test ID
- [x] user-reported runtime failures outrank stale completion claims

## Execution Layers

| Layer | Purpose |
|---|---|
| [current/README.md](current/README.md) | active blocker-first wave |
| [current/verification.md](current/verification.md) | verification gates and closure checks |
| [current/phases/README.md](current/phases/README.md) | phase-by-phase implementation checklist |
| [current/requirement-matrix.md](current/requirement-matrix.md) | requirement inventory and status |
| [current/mismatch-matrix.md](current/mismatch-matrix.md) | active mismatch tracking |
| [doc-coverage/README.md](doc-coverage/README.md) | direct-link checklist to every markdown file |
| [RECONSTRUCTION_PROMPT.md](RECONSTRUCTION_PROMPT.md) | machine execution contract |
| [reading/README.md](reading/README.md) | read-before-implement discipline |
| [completed/README.md](completed/README.md) | archive policy for closed waves |

## Completion Definition

A reconstruction wave is complete only when all are true:

1. [ ] all high-severity limitations are closed with matching `*R` E2E evidence
2. [ ] all blocker rows in `current/` matrices are closed or explicitly deferred
3. [ ] no checked TODO item lacks direct proof
4. [ ] doc coverage links every markdown file directly and has no stale links
5. [ ] topology and file-size constraints are satisfied by source layout policy

## Related

- [x] Current wave: [/docs/todo/current/README.md](/docs/todo/current/README.md)
- [x] Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
