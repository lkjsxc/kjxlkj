# Stage 05: Security, Reliability, and Recovery

Back: [../README.md](../README.md)

## Context

Define hardening boundaries for transport, session, and fault recovery behavior.

## Objective

Document enforceable controls for auth, csrf, session safety, and recovery outcomes.

- Detailed objective: [objective.md](objective.md)
- Detailed exits: [exit-criteria.md](exit-criteria.md)

## Prerequisites

- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/sequencing-rules.md](../../program/sequencing-rules.md)
- [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md)
- [../../../operations/quality/test-strategy.md](../../../operations/quality/test-strategy.md)
- [../../../operations/automation/failure-handling.md](../../../operations/automation/failure-handling.md)

## Ordered Wave Sequence

- [wave-050.md](wave-050.md) - Auth and Transport Hardening
- [wave-051.md](wave-051.md) - Regression Closure
- [wave-052.md](wave-052.md) - Performance and Recovery Baseline

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
