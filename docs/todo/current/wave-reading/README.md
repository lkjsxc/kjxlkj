# Wave: Read and Reconcile (Iteration 35)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Make implementation doc-driven by ensuring every document file is reachable, read, and reconciled before it drives behavior changes.

## Entry points

| Checklist | What it covers |
|---|---|
| [reading/README.md](reading/README.md) | Reading workflow and gates |
| [doc-topology/README.md](doc-topology/README.md) | Documentation topology, links, and policy compliance |

## Checklist (normative)

### A. Achieve full document link coverage

- [ ] Ensure every documentation file is directly linked by the TODO system.
- [ ] Use the coverage checklist as the canonical traversal mechanism:
  - [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)

### B. Read everything that can affect behavior

- [ ] Read every document linked by the coverage checklist.
- [ ] For each document, extract:
  - MUST/SHOULD requirements
  - invariants and error cases
  - acceptance criteria (Given/When/Then)
  - anything that implies observable user behavior

### C. Record reading and contradictions

- [ ] Record what was read and what was learned in:
  - [/docs/todo/reading/README.md](/docs/todo/reading/README.md)
- [ ] When contradictions exist:
  - record the conflict as a TODO leaf in this iteration
  - identify the canonical rule (where it should live)
  - update navigation so readers discover the canonical rule first
