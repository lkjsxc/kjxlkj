# Workspace Manifest Policy

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Normative requirements for derived runtime manifests.

## Root Files (When Runtime Snapshot Exists)

| Path | Requirement |
|---|---|
| `Cargo.toml` | Rust workspace resolver and members |
| `Cargo.lock` | committed for Rust reproducibility |
| `package.json` | frontend workspace/package manifest |
| `package-lock.json` | npm workspace lockfile reproducibility |
| `tsconfig.json` | TypeScript compiler settings with `strict: true` |

## Rust Workspace Settings

| Field | Requirement |
|---|---|
| `workspace.resolver` | MUST be `"2"` |
| `workspace.package.edition` | MUST be `"2021"` or newer approved edition |
| members | MUST include required backend crates |

## Frontend TypeScript Settings

| Field | Requirement |
|---|---|
| `compilerOptions.strict` | MUST be `true` |
| `compilerOptions.noImplicitAny` | MUST be `true` |
| `allowJs` | MUST be `false` |

## Dependency Direction Rule

- frontend packages MUST depend on typed shared contracts, not ad-hoc JSON shapes
- backend crates MUST expose typed DTO contracts used by API docs

## Related

- Source layout: [source-layout.md](source-layout.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- Root layout: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
