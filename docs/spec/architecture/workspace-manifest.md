# Workspace Manifest (Cargo)

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Normative requirements for reconstructed Rust workspace manifests.

## Required Files

- `Cargo.toml`
- `Cargo.lock`

## Required Settings

- `workspace.resolver = "2"`
- `workspace.package.edition = "2021"`
- workspace members include all canonical crates from [crates.md](crates.md)

## Related

- Crate topology: [crates.md](crates.md)
- Source layout: [source-layout.md](source-layout.md)
