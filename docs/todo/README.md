# TODO Management

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the execution control plane for full reconstruction.

## Objective

Following TODO items exactly MUST produce a complete reconstructed implementation that conforms to all normative docs.

## Normative Inputs

- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/reference/README.md](/docs/reference/README.md)

## Non-Negotiable Rules

- Every checkbox stays unchecked until evidence exists.
- No type-only or unreachable work may be marked complete.
- Conformance and limitations must be updated with every verified behavior change.
- Requirement coverage must be explicit and auditable.

## Execution Layers

| Layer | Purpose |
|---|---|
| [current/README.md](current/README.md) | Standby baseline checks and active wave gates |
| [doc-coverage/README.md](doc-coverage/README.md) | Documentation file coverage tracking |
| [RECONSTRUCTION_PROMPT.md](RECONSTRUCTION_PROMPT.md) | Execution contract for full rebuild runs |
| [reading/README.md](reading/README.md) | Mandatory reading discipline |
| [completed/README.md](completed/README.md) | Archived waves |

## Completion Definition

A wave is complete only when all are true:

1. Every normative requirement has coverage status (`verified` or explicit limitation).
2. Runtime behavior is user-reachable.
3. Deterministic verification gate is green for reconstructed profile.
4. Reference ledgers and TODO status are synchronized.
