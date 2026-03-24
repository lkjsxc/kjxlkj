# Stage 03: Web App Shell and Editor UX

Back: [../README.md](../README.md)

## Context

Define shell routing and editor-state behavior for deterministic user flow.

## Objective

Document auth-aware shell transitions, editor lifecycle, and responsive interaction constraints.

- Detailed objective: [objective.md](objective.md)
- Detailed exits: [exit-criteria.md](exit-criteria.md)

## Prerequisites

- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/sequencing-rules.md](../../program/sequencing-rules.md)
- [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md)
- [../../../product/surface/routes.md](../../../product/surface/routes.md)
- [../../../product/behavior/errors.md](../../../product/behavior/errors.md)

## Ordered Wave Sequence

- [wave-030.md](wave-030.md) - Shell and Auth Routing
- [wave-031.md](wave-031.md) - Editor State and Autosave
- [wave-032.md](wave-032.md) - Responsive Interaction Rules

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
