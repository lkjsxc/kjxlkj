# TODO Management

Back: [/docs/README.md](/docs/README.md)

Structured task tracking for reconstruction and continuous improvement.

## Documents

| Document | Content |
|---|---|
| [current/README.md](current/README.md) | Active TODO list with implementation checklists |
| [doc-coverage/README.md](doc-coverage/README.md) | Direct-link index of all repository documentation files |
| [completed/README.md](completed/README.md) | Completed tasks archive |
| [reading/README.md](reading/README.md) | Reading discipline |
| [RECONSTRUCTION_PROMPT.md](RECONSTRUCTION_PROMPT.md) | One-shot rebuild prompt with evidence-gated closure contract |

## Workflow

| Phase | Description |
|---|---|
| Plan | Read docs and select reconstruction target scope |
| Implement | Build one coherent slice via real user-reachable paths |
| Verify | Run deterministic checks and synchronize conformance/limitations |
| Audit | Record mismatch matrix and evidence under `/docs/log/reconstruction/audits/` |
| Recurse | Carry forward explicit next-iteration tasks for deferred items |

## Invariants

| Rule | Requirement |
|---|---|
| Anti-MVP | Follow [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md) |
| Testing | Follow [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) |
| Evidence | No checkbox checked without passing tests on real user paths |
| Second-to-last task | Always: Recreate the TODO list |
| Last task | Always: Continue to next iteration |

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Spec: [/docs/spec/README.md](/docs/spec/README.md)
- Anti-MVP measures: [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md)
