# Crates

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

The implementation is a Cargo workspace rooted at `src/crates/`.

## Topology Requirements

| Requirement | Value |
|---|---|
| Target crate count | 20 |
| Group roots | `app`, `core`, `platform`, `services` |
| Directory fan-out target | around 12 direct children |
| Source file limit | each source file should remain <=200 lines |

## Workspace Members by Group

| Group | Crate | Path |
|---|---|---|
| app | `kjxlkj` | `src/crates/app/kjxlkj` |
| app | `kjxlkj-test-harness` | `src/crates/app/kjxlkj-test-harness` |
| core | `kjxlkj-core` | `src/crates/core/kjxlkj-core` |
| core | `kjxlkj-core-types` | `src/crates/core/kjxlkj-core-types` |
| core | `kjxlkj-core-text` | `src/crates/core/kjxlkj-core-text` |
| core | `kjxlkj-core-edit` | `src/crates/core/kjxlkj-core-edit` |
| core | `kjxlkj-core-mode` | `src/crates/core/kjxlkj-core-mode` |
| core | `kjxlkj-core-undo` | `src/crates/core/kjxlkj-core-undo` |
| core | `kjxlkj-core-ui` | `src/crates/core/kjxlkj-core-ui` |
| core | `kjxlkj-core-state` | `src/crates/core/kjxlkj-core-state` |
| platform | `kjxlkj-host` | `src/crates/platform/kjxlkj-host` |
| platform | `kjxlkj-input` | `src/crates/platform/kjxlkj-input` |
| platform | `kjxlkj-render` | `src/crates/platform/kjxlkj-render` |
| services | `kjxlkj-services` | `src/crates/services/kjxlkj-services` |
| services | `kjxlkj-service-explorer` | `src/crates/services/kjxlkj-service-explorer` |
| services | `kjxlkj-service-fs` | `src/crates/services/kjxlkj-service-fs` |
| services | `kjxlkj-service-git` | `src/crates/services/kjxlkj-service-git` |
| services | `kjxlkj-service-index` | `src/crates/services/kjxlkj-service-index` |
| services | `kjxlkj-service-lsp` | `src/crates/services/kjxlkj-service-lsp` |
| services | `kjxlkj-service-terminal` | `src/crates/services/kjxlkj-service-terminal` |

## Decomposition Rules

| Rule | Requirement |
|---|---|
| split before overflow | if file trends toward 200 lines, extract focused modules early |
| fan-out balancing | if directory exceeds around 12 children, create domain subdirectories |
| test partitioning | split tests by concern before directories exceed fan-out target |
| IO separation | keep state mutation, dispatch, and external IO in separate modules |

## Reconstruction Contract

- TODO closure requires user-reachable behavior
- touched crates must add deterministic tests
- topology and module splits must satisfy [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)

## Related

- Source layout: [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- Workspace manifest: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
