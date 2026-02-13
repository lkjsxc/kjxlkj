# Wave 032: Graph Explorer and Responsive Shell

Back: [/docs/todo/waves/stage-03-single-container-runtime/README.md](/docs/todo/waves/stage-03-single-container-runtime/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] implement backlink graph explorer with scope-aware filters
- [x] implement return-context preserving navigation
- [x] enforce single responsive component-tree behavior
- [x] add compact-screen menu toggle to collapse/restore upper navigation areas
- [x] ensure note title rename propagates to list and related surfaces
- [x] keep default editor chrome minimal (no required inline version/save/delete controls)

## Verification Tasks

- [x] run `E2E-04`, `E2E-06`, `E2E-07`, `E2E-08`
- [x] run 320px interaction boundary checks
- [x] run `E2E-12`, `E2E-13`, `E2E-14`

## Evidence Placeholder

- [x] `Check: responsive shell + graph/context + autosave/title propagation + minimal chrome coverage`
- [x] `Result: pass`
- [x] `Proof: [/docs/log/audits/2026-02-13-stage-03-wave-032-graph-responsive-shell.md](/docs/log/audits/2026-02-13-stage-03-wave-032-graph-responsive-shell.md)`
