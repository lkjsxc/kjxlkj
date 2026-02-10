# Source Layout Blueprint

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Blueprint for reconstruction source topology.

## Goals

- keep source directories around 12 direct children
- keep source files below 200 lines
- make runtime wiring traceable by domain grouping

## Canonical Directory Tree

| Path | Target Direct Children | Notes |
|---|---:|---|
| `src/` | 1-4 | implementation root |
| `src/crates/` | 4 | grouped crate domains |
| `src/crates/app/` | 1-4 | shipped binary crates |
| `src/crates/core/` | 8-12 | core model and editing crates |
| `src/crates/platform/` | 4-8 | host/input/render/supervision crates |
| `src/crates/services/` | 5-8 | external integration service crates |

## Crate Grouping (normative)

| Group Path | Required Members |
|---|---|
| `src/crates/app/` | `kjxlkj` |
| `src/crates/core/` | `kjxlkj-core`, `kjxlkj-core-types`, `kjxlkj-core-text`, `kjxlkj-core-edit`, `kjxlkj-core-mode`, `kjxlkj-core-undo`, `kjxlkj-core-ui`, `kjxlkj-core-state` |
| `src/crates/platform/` | `kjxlkj-host`, `kjxlkj-input`, `kjxlkj-render`, `kjxlkj-services` |
| `src/crates/services/` | `kjxlkj-service-fs`, `kjxlkj-service-git`, `kjxlkj-service-index`, `kjxlkj-service-lsp`, `kjxlkj-service-terminal` |

## Binary Crate Layout (`src/crates/app/kjxlkj/src`)

| Module | Responsibility |
|---|---|
| `main.rs` | startup handoff only |
| `app.rs` | runtime construction and join |
| `channels.rs` | typed channel wiring |
| `services.rs` | service spawn and shutdown hooks |
| `signals.rs` | process signal orchestration |
| `cli.rs` | command-line arguments |

## High-Risk Module Layout Targets

| Path | Direct Children Target | Notes |
|---|---:|---|
| `src/crates/core/kjxlkj-core-state/src/` | 10-12 | use `ops/` and `tests/` subdirs |
| `src/crates/core/kjxlkj-core-edit/src/` | 8-12 | split operators/motions/tests |
| `src/crates/core/kjxlkj-core-mode/src/` | 8-12 | split dispatch and per-mode logic |
| `src/crates/platform/kjxlkj-render/src/` | 8-12 | use `paint/` subdir |
| `src/crates/platform/kjxlkj-input/src/` | 8-12 | decode/mapping/IME separation |
| `src/crates/services/kjxlkj-service-terminal/src/` | 8-12 | parser/screen/PTY separation |

## Overflow Procedure

| Trigger | Required Action |
|---|---|
| direct children > 12 | create domain subdirectory and move related files |
| file length > 200 | extract focused module and keep thin facade |
| mixed concerns | split by state mutation, dispatch, and IO side effects |

## Verification Rules

- every topology split preserves API and behavior
- moved logic receives deterministic tests in same change
- TODO topology items link this file directly

## Related

- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Workspace members: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
