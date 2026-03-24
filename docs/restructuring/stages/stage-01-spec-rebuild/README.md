# Stage 01: Workspace and Auth Foundation

Back: [../README.md](../README.md)

## Context

Define baseline runtime and access-control intent at documentation contract level.

## Objective

Encode typed skeleton, setup/login/session semantics, and role enforcement requirements.

- Detailed objective: [objective.md](objective.md)
- Detailed exits: [exit-criteria.md](exit-criteria.md)

## Prerequisites

- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/sequencing-rules.md](../../program/sequencing-rules.md)
- [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md)
- [../../../architecture/runtime/module-map.md](../../../architecture/runtime/module-map.md)
- [../../../product/surface/write-auth.md](../../../product/surface/write-auth.md)

## Ordered Wave Sequence

- [wave-010.md](wave-010.md) - Runtime Topology Skeleton
- [wave-011.md](wave-011.md) - Auth and Session Baseline
- [wave-012.md](wave-012.md) - Role and Membership Baseline

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
