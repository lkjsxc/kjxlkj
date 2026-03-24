# Stage 04: Schema, Automation, and Jobs

Back: [../README.md](../README.md)

## Context

Define migration, automation, and job orchestration contract surfaces.

## Objective

Document schema integrity, automation execution model, and export/backup lifecycle requirements.

- Detailed objective: [objective.md](objective.md)
- Detailed exits: [exit-criteria.md](exit-criteria.md)

## Prerequisites

- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/sequencing-rules.md](../../program/sequencing-rules.md)
- [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md)
- [../../../operations/automation/command-contracts.md](../../../operations/automation/command-contracts.md)
- [../../../operations/automation/failure-handling.md](../../../operations/automation/failure-handling.md)

## Ordered Wave Sequence

- [wave-040.md](wave-040.md) - Migration and Projection Structure
- [wave-041.md](wave-041.md) - Automation State Machine
- [wave-042.md](wave-042.md) - Export and Backup Jobs

## Exit Criteria

See [exit-criteria.md](exit-criteria.md). Stage closure requires all wave checklists complete.

## Evidence Expectations

- Update [../../evidence/drift-ledger.md](../../evidence/drift-ledger.md).
- Add run entries using [../../evidence/run-log-template.md](../../evidence/run-log-template.md).
- Reflect final stage state in [../../evidence/final.md](../../evidence/final.md).

## Failure Escalation

- Any failed mandatory gate blocks stage closure.
- Any unresolved high-severity risk blocks transition to next stage.
- Escalate unresolved blockers through [../../program/risk-model.md](../../program/risk-model.md).
