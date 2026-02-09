# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records open, user-visible mismatches between current behavior and target spec.

## Baseline Entry

| ID | Expected Behavior | Observed Behavior | Status | Evidence | Next Action |
|---|---|---|---|---|---|
| LIM-BASELINE-01 | Full implementation matches `/docs/spec/` | Repository may be docs-only or partially reconstructed | `open` | `/docs/reference/CONFORMANCE.md` | Use `/docs/todo/` gates to reconstruct and replace this baseline entry with concrete, scoped limitations |

## Entry Rules

Each limitation entry MUST include:

- link to expected behavior (`/docs/spec/...`)
- concise observed behavior statement
- deterministic evidence pointer
- concrete next action

## Lifecycle Rules

- Close or update limitations in the same change that modifies behavior.
- Do not remove a limitation without evidence that the gap is closed.
- If behavior is intentionally deferred, keep limitation open with dated rationale.

## Related

- Current-state claims: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Reconstruction gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
