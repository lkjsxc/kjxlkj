# Stage 02: Notes and Realtime Core

Back: [../README.md](../README.md)

## Context

Define note lifecycle and replay semantics that govern core runtime behavior.

## Objective

Capture deterministic note, event, replay, metadata, and search contracts.

- Detailed objective: [objective.md](objective.md)
- Detailed exits: [exit-criteria.md](exit-criteria.md)

## Prerequisites

- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/sequencing-rules.md](../../program/sequencing-rules.md)
- [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md)
- [../../../product/behavior/list-and-fetch.md](../../../product/behavior/list-and-fetch.md)
- [../../../architecture/integrations/json-contract.md](../../../architecture/integrations/json-contract.md)

## Ordered Wave Sequence

- [wave-020.md](wave-020.md) - Note Lifecycle Baseline
- [wave-021.md](wave-021.md) - Replay and Patch Semantics
- [wave-022.md](wave-022.md) - Metadata and Search Closure

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
