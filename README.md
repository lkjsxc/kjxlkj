# kjxlkj

`kjxlkj` is a Neovim-inspired terminal editor with a documentation-first contract.

## All in Docs

Project contract:

- `/docs/` is the canonical system definition.
- Source code, CI, and packaging artifacts are derived outputs.
- The repository may intentionally exist in a docs-only baseline and be reconstructed from documentation.

Canonical explanation:

- [docs/overview/all-in-docs.md](docs/overview/all-in-docs.md)

## Canonical Docs

- Documentation index: [docs/README.md](docs/README.md)
- Policies: [docs/policy/README.md](docs/policy/README.md)
- Specifications: [docs/spec/README.md](docs/spec/README.md)
- Current verified status: [docs/reference/CONFORMANCE.md](docs/reference/CONFORMANCE.md)
- Known gaps: [docs/reference/LIMITATIONS.md](docs/reference/LIMITATIONS.md)
- Reconstruction contract: [docs/todo/RECONSTRUCTION_PROMPT.md](docs/todo/RECONSTRUCTION_PROMPT.md)

## Repository States

- Docs-only baseline: canonical docs are present; derived artifacts may be absent.
- Reconstructed implementation: workspace and automation artifacts are regenerated from docs.

Current state: docs-only baseline active.

## Reconstructing the Implementation

Follow:

- [docs/todo/RECONSTRUCTION_PROMPT.md](docs/todo/RECONSTRUCTION_PROMPT.md)
- [docs/todo/README.md](docs/todo/README.md)
- [docs/todo/current/README.md](docs/todo/current/README.md)

When reconstructed, the Rust workspace is expected under `src/crates/` with
grouped crate roots:

- `src/crates/app/`
- `src/crates/core/`
- `src/crates/platform/`
- `src/crates/services/`

## Build and Verification (When Workspace Exists)

Build:

```bash
cargo build
```

Format check:

```bash
cargo fmt --all -- --check
```

Lint:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Test:

```bash
cargo test --workspace
```

CI contract:

- [docs/reference/CI.md](docs/reference/CI.md)

## Docker (When Artifacts Exist)

If `Dockerfile` exists:

```bash
docker build -t kjxlkj:dev .
docker run --rm -it kjxlkj:dev
```

Container guidance:

- [docs/guides/DOCKER.md](docs/guides/DOCKER.md)
