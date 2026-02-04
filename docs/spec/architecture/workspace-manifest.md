# Workspace Manifest (Cargo)

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
Normative requirements for reconstructing the root Cargo workspace.

## Files (root)

| Path | Requirement |
|------|-------------|
| `Cargo.toml` | MUST define the workspace and its members |
| `Cargo.lock` | SHOULD be committed for reproducible builds |
| `.gitignore` | SHOULD ignore build artifacts (`/target`) |

## Workspace settings

The root manifest (`Cargo.toml`) MUST define:

| Field | Requirement |
|-------|-------------|
| `workspace.resolver` | MUST be `"2"` |
| `workspace.members` | MUST include all crates under `src/crates/` listed below |

## Workspace members

The workspace MUST include these members (paths are relative to repo root):

| Member path | Crate |
|-------------|-------|
| `src/crates/kjxlkj` | `kjxlkj` |
| `src/crates/kjxlkj-core` | `kjxlkj-core` |
| `src/crates/kjxlkj-core-types` | `kjxlkj-core-types` |
| `src/crates/kjxlkj-core-text` | `kjxlkj-core-text` |
| `src/crates/kjxlkj-core-edit` | `kjxlkj-core-edit` |
| `src/crates/kjxlkj-core-mode` | `kjxlkj-core-mode` |
| `src/crates/kjxlkj-core-undo` | `kjxlkj-core-undo` |
| `src/crates/kjxlkj-core-ui` | `kjxlkj-core-ui` |
| `src/crates/kjxlkj-core-state` | `kjxlkj-core-state` |
| `src/crates/kjxlkj-host` | `kjxlkj-host` |
| `src/crates/kjxlkj-input` | `kjxlkj-input` |
| `src/crates/kjxlkj-render` | `kjxlkj-render` |
| `src/crates/kjxlkj-services` | `kjxlkj-services` |
| `src/crates/kjxlkj-service-lsp` | `kjxlkj-service-lsp` |
| `src/crates/kjxlkj-service-git` | `kjxlkj-service-git` |
| `src/crates/kjxlkj-service-index` | `kjxlkj-service-index` |
| `src/crates/kjxlkj-service-fs` | `kjxlkj-service-fs` |
| `src/crates/kjxlkj-service-terminal` | `kjxlkj-service-terminal` |

## Workspace package defaults

The workspace SHOULD define shared package metadata (used by member crates):

| Field | Value |
|-------|-------|
| Edition | `2021` |
| Versioning | Semver (`0.1.0` initial is acceptable) |
| License | `Apache-2.0` |

## Dependency policy

The workspace SHOULD define shared dependency versions to keep the implementation consistent across crates.

Recommended baseline set (version ranges are intentionally broad):

| Dependency | Purpose |
|------------|---------|
| `tokio` | Async runtime and service supervision |
| `crossterm` | Terminal IO and events |
| `ropey` | Rope text storage |
| `unicode-segmentation` | Word/grapheme segmentation helpers |
| `unicode-width` | Display width calculations |
| `serde` + `serde_json` | Headless scripts and structured IO for tests |
| `thiserror` / `anyhow` | Error types and propagation |
| `tracing` + `tracing-subscriber` | Structured logging |

## Related

- Root layout allowlist: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- Crate topology: [crates.md](crates.md)
