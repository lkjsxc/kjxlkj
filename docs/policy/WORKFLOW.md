# Workflow

Back: [/docs/policy/README.md](/docs/policy/README.md)

Reconstruction workflow and verification gates.

## Gate Sequence

1. read policy/spec/reference/todo roots
2. build drift matrix from current state
3. execute one TODO stage at a time
4. run deterministic checks for touched scope
5. update ledgers and TODO in same change
6. commit progress

## TODO Authoring Rule

TODO files MUST include:

- `## Relevant Documents`
- direct links to required docs
- checklist items with linked governing docs

## TODO Status Scan Rule

Use a command scan before completion decisions:

- `rg -n "\\[ \\]" docs/todo`

If unchecked items remain, work is still pending unless explicitly reset by design.

## Related

- CI baseline: [/docs/reference/CI.md](/docs/reference/CI.md)
- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
