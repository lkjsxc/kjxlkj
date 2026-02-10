# TODO Management

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the execution control plane for full reconstruction.

## Objective

Following TODO items exactly MUST produce an implementation that conforms to policy and spec, with blocker-first verification.

## Normative Inputs

- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/reference/README.md](/docs/reference/README.md)

## Non-Negotiable Rules

- [x] no checkbox is marked complete without deterministic evidence
- [x] no scaffold-only or unreachable path is treated as complete
- [x] every TODO closure updates conformance, limitations, and drift matrix in same change
- [x] every TODO closure references at least one test ID and one spec link
- [x] high-severity user-reported failures remain open until reproduced and fixed

## Execution Layers

| Layer | Purpose |
|---|---|
| [current/README.md](current/README.md) | active blocker-first reconstruction plan |
| [current/verification.md](current/verification.md) | verification gates and closure checks |
| [current/phases/README.md](current/phases/README.md) | phase-by-phase implementation checklist |
| [doc-coverage/README.md](doc-coverage/README.md) | direct-link checklist to every documentation file |
| [RECONSTRUCTION_PROMPT.md](RECONSTRUCTION_PROMPT.md) | machine execution contract |
| [reading/README.md](reading/README.md) | read-before-implement discipline |
| [completed/README.md](completed/README.md) | archive policy for completed waves |

## Completion Definition

A reconstruction wave is complete only when all are true:

1. [ ] all high-severity limitations are closed with live E2E evidence
2. [ ] all blocker IDs in current phase docs are checked
3. [ ] no TODO checkbox lacks direct proof
4. [ ] doc coverage links every markdown document directly
5. [ ] topology and file-size constraints are met per architecture policy
