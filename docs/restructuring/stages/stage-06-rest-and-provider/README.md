# Stage 06: REST and Provider Completion

Back: [../README.md](../README.md)

## Context

Define protocol completeness for HTTP surfaces and provider integration.

## Objective

Document HTTP parity, prompt-pack loading semantics, and protocol safety guarantees.

- Detailed objective: [objective.md](objective.md)
- Detailed exits: [exit-criteria.md](exit-criteria.md)

## Prerequisites

- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/sequencing-rules.md](../../program/sequencing-rules.md)
- [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md)
- [../../../product/api-contract.md](../../../product/api-contract.md)
- [../../../architecture/integrations/time-contract.md](../../../architecture/integrations/time-contract.md)

## Ordered Wave Sequence

- [wave-060.md](wave-060.md) - HTTP and Type Parity
- [wave-061.md](wave-061.md) - Provider and Prompt Loading
- [wave-062.md](wave-062.md) - Protocol Safety and Retry

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
