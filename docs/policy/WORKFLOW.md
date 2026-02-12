# Workflow

Back: [/docs/policy/README.md](/docs/policy/README.md)

Reconstruction workflow and verification gates.

## Gate Sequence

1. Read policy, spec, reference, and todo indexes.
2. Build mismatch matrix (spec vs implementation vs tests).
3. Select one coherent migration slice.
4. Implement only user-reachable behavior in that slice.
5. Run deterministic tests for touched behavior and relevant global gates.
6. Update reference ledgers and TODO state in the same change.

## Verification Gate

The baseline gate is defined in [/docs/reference/CI.md](/docs/reference/CI.md).

A change is complete only if:

- required checks are green for the targeted state
- conformance and limitations are synchronized
- TODO updates are evidence-backed

## Drift Handling

When mismatch is found:

- classify mismatch type
- prioritize correctness and user-visible behavior first
- either close mismatch now or record explicit defer rationale with next action

## Docs-Only Baseline Rule

In docs-only state, missing implementation artifacts are acceptable,
but TODO and reference documents MUST explicitly define regeneration steps.

## Related

- Operating contract: [INSTRUCT.md](INSTRUCT.md)
- CI baseline: [/docs/reference/CI.md](/docs/reference/CI.md)
- Reconstruction plan: [/docs/todo/README.md](/docs/todo/README.md)
