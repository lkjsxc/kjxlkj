# Stage 07: WebSocket Replay and Automation Events

Back: [../README.md](../README.md)

## Context

Define deterministic realtime ordering, replay, and reconnect semantics.

## Objective

Document event ordering, ack/replay behavior, and stale-cursor handling requirements.

- Detailed objective: [objective.md](objective.md)
- Detailed exits: [exit-criteria.md](exit-criteria.md)

## Prerequisites

- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/sequencing-rules.md](../../program/sequencing-rules.md)
- [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md)
- [../../../operations/verification/local-runbook.md](../../../operations/verification/local-runbook.md)
- [../../../operations/quality/gates.md](../../../operations/quality/gates.md)

## Ordered Wave Sequence

- [wave-070.md](wave-070.md) - Workspace and Automation Event Surface
- [wave-071.md](wave-071.md) - Ack, Replay, and Cursor Behavior
- [wave-072.md](wave-072.md) - End-to-End Realtime Closure

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
