# Source Layout Blueprint

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Blueprint for reconstruction topology and module sizing.

## Goals

- keep source directories around 12 direct children
- keep source files at or below 200 lines
- preserve traceable domain wiring for runtime-critical behavior

## Canonical Directory Targets

| Path | Target Direct Children | Purpose |
|---|---:|---|
| `src/` | 1-4 | top-level source root |
| `src/crates/` | 4 | grouped domain roots |
| `src/crates/app/` | 2-6 | binary and harness crates |
| `src/crates/core/` | 8-12 | core model/edit/mode/state crates |
| `src/crates/platform/` | 3-8 | host/input/render integrations |
| `src/crates/services/` | 6-12 | external service crates and supervisor |

## Canonical Tree Shape

| Path | Expected Child Entries |
|---|---|
| `src/crates/app/` | `kjxlkj`, `kjxlkj-test-harness` |
| `src/crates/core/` | `kjxlkj-core`, `kjxlkj-core-types`, `kjxlkj-core-text`, `kjxlkj-core-edit`, `kjxlkj-core-mode`, `kjxlkj-core-undo`, `kjxlkj-core-ui`, `kjxlkj-core-state` |
| `src/crates/platform/` | `kjxlkj-host`, `kjxlkj-input`, `kjxlkj-render` |
| `src/crates/services/` | `kjxlkj-services`, `kjxlkj-service-explorer`, `kjxlkj-service-fs`, `kjxlkj-service-git`, `kjxlkj-service-index`, `kjxlkj-service-lsp`, `kjxlkj-service-terminal` |

## High-Risk Module Layout Targets

| Path | Target Direct Children | Decomposition Guidance |
|---|---:|---|
| `src/crates/core/kjxlkj-core-state/src/` | 8-12 | split `tree`, `split`, `layout`, `editor`, `window`, `tests/` |
| `src/crates/core/kjxlkj-core-mode/src/` | 8-12 | split per mode and per prefix family |
| `src/crates/platform/kjxlkj-input/src/` | 8-12 | separate decode, normalization, IME gate, mapping resolver |
| `src/crates/platform/kjxlkj-render/src/` | 8-12 | separate wrap, cursor, paint, diff modules |
| `src/crates/services/kjxlkj-service-terminal/src/` | 8-12 | separate PTY, parser, screen, lifecycle, tests |
| `src/crates/services/kjxlkj-service-explorer/src/` | 8-12 | separate state, fs actions, reveal, refresh, tests |

## Overflow Procedure

| Trigger | Required Action |
|---|---|
| direct children > 12 | create focused subdirectory and rebalance by domain |
| source file > 200 lines | extract cohesive modules and keep facade thin |
| mixed responsibilities | split state mutation, command parsing, and IO side effects |

## Verification Rules

- every structural split preserves behavior and public contract
- moved logic receives deterministic tests in same change
- topology checks are part of phase-5 hardening gate

## Related

- Crate inventory: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Workspace members: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
