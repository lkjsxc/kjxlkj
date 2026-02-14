# Workflow

Back: [/docs/policy/README.md](/docs/policy/README.md)

Reconstruction workflow and verification gates.

## Gate Sequence

1. Read policy/spec/reference/todo indexes.
2. Build mismatch matrix (spec vs implementation vs tests).
3. Select one coherent migration slice.
4. Implement only user-reachable behavior in that slice.
5. Run deterministic checks mapped to touched acceptance IDs.
6. Update reference ledgers and TODO state in the same change.
7. Record evidence and commit in small auditable units.

## Drift Handling

When mismatch is found:

- classify by `M1..M5`
- resolve `M1` then user-facing `M2`
- close now or defer with explicit rationale and next action

## All in Docs Rule

At any time, repository validity is measured by docs integrity first.

- derived source/runtime artifacts may be removed
- removal is valid if ledgers and TODO reflect the state truthfully
- reconstruction restarts from docs without loss of canonical intent

## Typed Reconstruction Gate

Any runtime reconstruction claim is invalid unless all are true:

1. backend is Rust
2. frontend is TypeScript with strict typing
3. handwritten JavaScript runtime source is absent

## Related

- Operating contract: [INSTRUCT.md](INSTRUCT.md)
- CI baseline: [/docs/reference/CI.md](/docs/reference/CI.md)
- Execution plan: [/docs/todo/README.md](/docs/todo/README.md)
