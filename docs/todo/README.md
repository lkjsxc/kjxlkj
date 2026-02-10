# TODO Management

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the execution control plane for full reconstruction.

## Objective

Following TODO items exactly MUST produce a complete implementation that
conforms to policy, spec, and reference contracts.

## Normative Inputs

- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/reference/README.md](/docs/reference/README.md)

## Non-Negotiable Rules

- [ ] no checkbox is marked complete without deterministic evidence
- [ ] no scaffold-only or unreachable path is treated as complete
- [ ] every TODO closure updates conformance and limitations in the same change
- [ ] every TODO closure references at least one test ID and one spec link

## Execution Layers

| Layer | Purpose |
|---|---|
| [current/README.md](current/README.md) | active reconstruction phases and global gates |
| [current/verification.md](current/verification.md) | hard verification checkpoints |
| [current/phases/README.md](current/phases/README.md) | phase-by-phase implementation checklists |
| [doc-coverage/README.md](doc-coverage/README.md) | direct-link checklist to every documentation file |
| [RECONSTRUCTION_PROMPT.md](RECONSTRUCTION_PROMPT.md) | machine-execution contract |
| [reading/README.md](reading/README.md) | deterministic read-before-implement discipline |
| [completed/README.md](completed/README.md) | archive policy for completed waves |

## Completion Definition

A reconstruction wave is complete only when all are true:

1. [ ] every normative requirement is `verified` or an explicit limitation
2. [ ] all claimed behaviors are reachable from real key/command paths
3. [ ] required deterministic gates are green
4. [ ] conformance, limitations, and TODO states are synchronized
5. [ ] doc-coverage lists every documentation file with direct links
