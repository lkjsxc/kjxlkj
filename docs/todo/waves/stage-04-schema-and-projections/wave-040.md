# Wave 040: Automation Rules and Validation

Back: [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] implement automation rule CRUD APIs -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] validate trigger/condition/action deterministically -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] enforce workspace role checks on rule mutation -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)

## Verification Tasks

- [ ] run `API-AUTO-01` -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] run invalid-rule and forbidden-path checks -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)

## Evidence Placeholder

- [ ] `Check: automation rule lifecycle, deterministic validation, and forbidden-path integration coverage` -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] `Result: pass` -> [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-04-wave-040-automation-rules.md](/docs/log/audits/2026-02-13-stage-04-wave-040-automation-rules.md)`
