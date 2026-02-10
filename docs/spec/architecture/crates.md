# Crates

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

The implementation is a Cargo workspace under `src/crates/`.

## Topology Requirements

| Requirement | Value |
|---|---|
| Target crate count | 18 |
| Directory fan-out target | around 12 direct children |
| Source file length limit | 200 lines |
| Grouped crate roots | `app`, `core`, `platform`, `services` |

## Workspace Members by Group

| Group | Crate | Path |
|---|---|---|
| app | `kjxlkj` | `src/crates/app/kjxlkj` |
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
| platform | `kjxlkj-services` | `src/crates/platform/kjxlkj-services` |
| services | `kjxlkj-service-fs` | `src/crates/services/kjxlkj-service-fs` |
| services | `kjxlkj-service-git` | `src/crates/services/kjxlkj-service-git` |
| services | `kjxlkj-service-index` | `src/crates/services/kjxlkj-service-index` |
| services | `kjxlkj-service-lsp` | `src/crates/services/kjxlkj-service-lsp` |
| services | `kjxlkj-service-terminal` | `src/crates/services/kjxlkj-service-terminal` |

## Decomposition Rules

| Rule | Requirement |
|---|---|
| Split before overflow | files trending toward 200 lines split in same wave |
| Dispatch decomposition | large dispatch logic split by mode/command/service/UI |
| Test decomposition | test files split by concern before exceeding limits |
| Fan-out balancing | if directory grows beyond around 12 children, create domain subdirs |

## Reconstruction Contract

- TODO item closes only when behavior is user-reachable
- touched crates must include deterministic tests
- crate completion must satisfy [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)

## Related

- Source layout blueprint: [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- Workspace manifest policy: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
