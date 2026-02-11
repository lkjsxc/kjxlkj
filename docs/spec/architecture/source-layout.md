# Source Layout Blueprint

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Blueprint for reconstruction topology and module sizing.

## Goals

- keep source directories near 12 direct children
- split source files before they exceed 200 lines
- keep runtime-critical paths decomposed by responsibility

## Canonical Top-Level Tree

| Path | Target Direct Children | Purpose |
|---|---:|---|
| `src/` | 1-4 | source root |
| `src/crates/` | 4 | grouped domain roots |
| `src/crates/app/` | 2-6 | app binary and test harness crates |
| `src/crates/core/` | 8-12 | core state/edit/mode/text/ui crates |
| `src/crates/platform/` | 3-8 | host/input/render integration crates |
| `src/crates/services/` | 6-12 | service crates and service supervisor |

## Required Grouped Crate Paths

| Path | Required Entries |
|---|---|
| `src/crates/app/` | `kjxlkj`, `kjxlkj-test-harness` |
| `src/crates/core/` | `kjxlkj-core`, `kjxlkj-core-types`, `kjxlkj-core-text`, `kjxlkj-core-edit`, `kjxlkj-core-mode`, `kjxlkj-core-undo`, `kjxlkj-core-ui`, `kjxlkj-core-state` |
| `src/crates/platform/` | `kjxlkj-host`, `kjxlkj-input`, `kjxlkj-render` |
| `src/crates/services/` | `kjxlkj-services`, `kjxlkj-service-explorer`, `kjxlkj-service-fs`, `kjxlkj-service-git`, `kjxlkj-service-index`, `kjxlkj-service-lsp`, `kjxlkj-service-terminal` |

## High-Risk Internal Layout Targets

| Path | Target Direct Children | Minimum Required Split |
|---|---:|---|
| `src/crates/core/kjxlkj-core-state/src/` | 8-12 | editor state, window tree, focus, session codec, tests |
| `src/crates/core/kjxlkj-core-mode/src/` | 8-12 | normal, insert, command, visual, replace, resolver |
| `src/crates/platform/kjxlkj-input/src/` | 8-12 | decode, normalize, ime gate, mapping resolver, trace |
| `src/crates/platform/kjxlkj-render/src/` | 8-12 | wrap, cursor, grid, paint diff, diagnostics |
| `src/crates/services/kjxlkj-service-explorer/src/` | 8-12 | tree state, fs ops, reveal, refresh, tests |
| `src/crates/services/kjxlkj-service-terminal/src/` | 8-12 | PTY, parser, screen model, lifecycle, tests |
| `src/crates/app/kjxlkj-test-harness/src/` | 6-12 | PTY transport, script runner, dump parser, frame oracle |

## Overflow Procedure

| Trigger | Required Action |
|---|---|
| direct children > 12 | create focused subdirectory and rebalance by domain |
| source file > 200 lines | extract cohesive module and keep thin facade |
| mixed responsibilities | separate state mutation, parsing, routing, and IO side effects |

## Implementation-Time Rule

When implementing new behavior, split early instead of waiting for overflow.
If a file crosses 160 lines and growth is expected, preemptively extract modules.

## Verification Rules

- every structural split preserves behavior and public contract
- moved logic receives deterministic tests in the same change
- topology checks are mandatory in hardening and release gates

## Related

- Crate inventory: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Workspace members: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
