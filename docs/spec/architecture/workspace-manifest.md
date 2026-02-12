# Workspace Manifest (Cargo)

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Normative requirements for reconstructing workspace manifests.

## Root Files

| Path | Requirement |
|---|---|
| `Cargo.toml` | defines workspace resolver and members |
| `Cargo.lock` | committed for reproducibility |
| `.gitignore` | excludes derived build artifacts |

## Workspace Settings

| Field | Requirement |
|---|---|
| `workspace.resolver` | MUST be `"2"` |
| `workspace.package.edition` | MUST be `"2021"` |
| `workspace.members` | MUST include canonical crate paths |

## Shared Dependencies

| Dependency | Purpose |
|---|---|
| `actix-web` | HTTP server |
| `actix-web-actors` | WebSocket actor support |
| `tokio` | async runtime |
| `sqlx` | async PostgreSQL access and migrations |
| `serde`, `serde_json` | schema serialization |
| `tracing`, `tracing-subscriber` | diagnostics |
| `thiserror`, `anyhow` | error handling |
| `uuid`, `time` | identifiers and timestamps |

## Related

- Crate topology: [crates.md](crates.md)
- Root layout: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- Source layout: [source-layout.md](source-layout.md)
