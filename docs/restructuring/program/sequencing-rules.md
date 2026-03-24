# Sequencing Rules

## Stage Order

`S00 -> S01 -> S02 -> S03 -> S04 -> S05 -> S06 -> S07 -> S08 -> S09 -> S10`

## Wave Order Rule

Within each stage, waves run in numerical order and may not be closed out of order.

## Gate Restart Rule

If any mandatory gate fails, restart from the first gate listed in [../checkpoints/gate-checklist.md](../checkpoints/gate-checklist.md) after remediation.

## Transition Conditions

A stage can transition only when:

- all three wave completion checklists are complete,
- all stage exit IDs are satisfied,
- no unresolved high-severity blocker remains.
