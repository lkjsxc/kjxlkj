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

- [x] implement automation rule CRUD APIs
- [x] validate trigger/condition/action deterministically
- [x] enforce workspace role checks on rule mutation

## Verification Tasks

- [x] run `API-AUTO-01`
- [x] run invalid-rule and forbidden-path checks

## Evidence Placeholder

- [x] `Check: automation rule lifecycle, deterministic validation, and forbidden-path integration coverage`
- [x] `Result: pass`
- [x] `Proof: [/docs/log/audits/2026-02-13-stage-04-wave-040-automation-rules.md](/docs/log/audits/2026-02-13-stage-04-wave-040-automation-rules.md)`
