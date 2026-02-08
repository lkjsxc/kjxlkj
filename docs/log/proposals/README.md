# Proposals

Back: [/docs/log/README.md](/docs/log/README.md)

## Purpose

Store improvement proposals that are not yet canonical specification.

Each proposal MUST:

- Link to the defining spec documents (or the spec gaps)
- Define acceptance criteria and a test strategy
- Identify user-visible impacts and whether limitations must be updated

## Active proposals

| Proposal | Topic |
|---|---|
| [reconstruction-wave-improvements.md](reconstruction-wave-improvements.md) | Future improvement ideas (rope diffing, LSP streaming, WASM plugins, etc.) |
| [performance-regression-harness.md](performance-regression-harness.md) | CPU/latency regression harness and gating strategy |
| [profiling-workflow.md](profiling-workflow.md) | Repeatable profiling workflow and regression triage |
| [anti-mvp-measures.md](anti-mvp-measures.md) | Measures to prevent minimal/scaffold-only implementations |
| [deep-wiring-checklist.md](deep-wiring-checklist.md) | Per-crate module/function wiring inventory (core, render, input, terminal) |
| [deep-wiring-checklist-2.md](deep-wiring-checklist-2.md) | Per-crate module/function wiring inventory (binary, facade, host, services) |
| [terminal-emulator-detail.md](terminal-emulator-detail.md) | Terminal emulator full-scratch implementation detail proposal |

## Archived

Previous proposals for insert-newline, long-lines, file-explorer-mvp, and docs-only-baseline-reset were deleted after their requirements were promoted to canonical specs.
