# kjxlkj

kjxlkj is a Neovim-inspired TUI text editor written in Rust.

## “All in Docs” (project contract)

This repository follows an **“All in Docs”** philosophy:

- The documentation under [`/docs/`](docs/README.md) is the **project** and the **source of truth**.
- Everything else (code, build files, CI, generated artifacts) is a **derived** implementation detail.
- The documentation is written so that **even if everything except `/docs/` disappeared**, a complete, working system can be reconstructed from the docs.

This repo is also optimized for **machine (LLM) parsing and editing**. Canonical explanation: [`docs/overview/all-in-docs.md`](docs/overview/all-in-docs.md).

## Docs

- Documentation index: [docs/README.md](docs/README.md)
- Canonical specifications: [docs/spec/README.md](docs/spec/README.md)
- Policies and invariants: [docs/policy/README.md](docs/policy/README.md)
- Reconstruction prompt (Copilot/Claude): [docs/todo/RECONSTRUCTION_PROMPT.md](docs/todo/RECONSTRUCTION_PROMPT.md)

## Implementation

- The canonical specification lives under `docs/` and is treated as normative.
- The Rust implementation lives under `src/crates/` as a Cargo workspace.

Toolchain:

- Rust stable is expected. When present, `rust-toolchain.toml` pins the toolchain + components for reproducible builds/CI (see `docs/policy/ROOT_LAYOUT.md`).

Build:

```bash
cargo build
```

Format:

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

Docker:

Docker support is a target derived artifact. If `Dockerfile` exists, build/run with:

```bash
docker build -t kjxlkj:dev .
docker run --rm -it kjxlkj:dev
```
