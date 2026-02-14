# Wave 041: Automation Run Engine and Events

Back: [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] implement rule trigger evaluation and run state machine -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] enforce idempotent run execution per triggering event -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] emit WS automation events and audit logs -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)

## Verification Tasks

- [ ] run `API-AUTO-02` and `E2E-09` -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] run run-idempotency boundary tests -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)

## Evidence Placeholder

- [ ] `Check: automation run lifecycle/state machine, run-idempotency, and workspace-event replay coverage` -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] `Result: pass` -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-04-wave-041-automation-runs.md](/docs/log/audits/2026-02-13-stage-04-wave-041-automation-runs.md)`
