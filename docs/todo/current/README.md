# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Standby reconstruction baseline.

This state is used when derived artifacts are intentionally removed and the next
implementation wave has not started yet.

## Standby Baseline Checks

- [ ] Confirm docs-only repository shape (no workspace/source/CI artifacts).
- [ ] Confirm conformance and limitations reflect docs-first baseline.
- [ ] Confirm all reconstruction checklists remain evidence-empty (unchecked).
- [ ] Confirm doc coverage inventory is up to date.

When starting a new wave, complete Global Preconditions first and then proceed
through the domain checklists.

## Global Preconditions

- [ ] Read canonical documents in required order.
- [ ] Create a requirement inventory covering all normative spec files.
- [ ] Create mismatch matrix (spec vs implementation vs tests).
- [ ] Prioritize correctness and user-visible gaps first.

## Domain Checklists

- [ ] [areas/architecture.md](areas/architecture.md)
- [ ] [areas/editor-core.md](areas/editor-core.md)
- [ ] [areas/modes.md](areas/modes.md)
- [ ] [areas/editing.md](areas/editing.md)
- [ ] [areas/commands.md](areas/commands.md)
- [ ] [areas/scripting.md](areas/scripting.md)
- [ ] [areas/ui-rendering.md](areas/ui-rendering.md)
- [ ] [areas/ux.md](areas/ux.md)
- [ ] [areas/technical.md](areas/technical.md)
- [ ] [features/features-core.md](features/features-core.md)
- [ ] [features/features-services.md](features/features-services.md)
- [ ] [features/features-editing.md](features/features-editing.md)
- [ ] [verification.md](verification.md)

## Global Exit Criteria

- [ ] No normative requirement remains without either verification evidence or an explicit open limitation.
- [ ] Conformance ledgers contain dated evidence links.
- [ ] Limitations ledger contains only intentional, scoped residual gaps.
- [ ] TODO checkboxes reflect evidence-backed state only.
