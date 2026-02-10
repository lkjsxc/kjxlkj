# Workspace Manifest (Cargo)

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Normative requirements for reconstructing root Cargo workspace.

## Root Files

| Path | Requirement |
|---|---|
| `Cargo.toml` | must define workspace and all members |
| `Cargo.lock` | should be committed for reproducible builds |
| `.gitignore` | should ignore derived build artifacts |

## Workspace Settings

| Field | Requirement |
|---|---|
| `workspace.resolver` | must be `"2"` |
| `workspace.members` | must include all grouped crate paths below |

## Workspace Members

| Member Path | Crate |
|---|---|
| `src/crates/app/kjxlkj` | `kjxlkj` |
| `src/crates/core/kjxlkj-core` | `kjxlkj-core` |
| `src/crates/core/kjxlkj-core-types` | `kjxlkj-core-types` |
| `src/crates/core/kjxlkj-core-text` | `kjxlkj-core-text` |
| `src/crates/core/kjxlkj-core-edit` | `kjxlkj-core-edit` |
| `src/crates/core/kjxlkj-core-mode` | `kjxlkj-core-mode` |
| `src/crates/core/kjxlkj-core-undo` | `kjxlkj-core-undo` |
| `src/crates/core/kjxlkj-core-ui` | `kjxlkj-core-ui` |
| `src/crates/core/kjxlkj-core-state` | `kjxlkj-core-state` |
| `src/crates/platform/kjxlkj-host` | `kjxlkj-host` |
| `src/crates/platform/kjxlkj-input` | `kjxlkj-input` |
| `src/crates/platform/kjxlkj-render` | `kjxlkj-render` |
| `src/crates/platform/kjxlkj-services` | `kjxlkj-services` |
| `src/crates/services/kjxlkj-service-fs` | `kjxlkj-service-fs` |
| `src/crates/services/kjxlkj-service-git` | `kjxlkj-service-git` |
| `src/crates/services/kjxlkj-service-index` | `kjxlkj-service-index` |
| `src/crates/services/kjxlkj-service-lsp` | `kjxlkj-service-lsp` |
| `src/crates/services/kjxlkj-service-terminal` | `kjxlkj-service-terminal` |

## Workspace Package Defaults

| Field | Value |
|---|---|
| Edition | `2021` |
| Versioning | semver |
| License | `Apache-2.0` |

## Dependency Policy

Shared dependency versions should be defined in workspace scope for consistency.

| Dependency | Purpose |
|---|---|
| `tokio` | async runtime and supervision |
| `crossterm` | terminal IO and events |
| `ropey` | rope text storage |
| `unicode-segmentation` | grapheme segmentation |
| `unicode-width` | display width calculations |
| `serde` + `serde_json` | structured state and session IO |
| `thiserror` / `anyhow` | error propagation |
| `tracing` + `tracing-subscriber` | structured logging |

## Related

- Root layout: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Source layout: [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
