# kjxlkj

kjxlkj is a Neovim-inspired TUI text editor written in Rust.

This repository is optimized for machine (LLM) parsing and editing. This statement appears only here and in docs/README.md + docs/policy/README.md to avoid scattering it across leaf documents.

## Docs

- Canonical specifications: docs/spec/README.md
- Policies and invariants: docs/policy/README.md
- Documentation index: docs/README.md

## Implementation

- The canonical specification lives under `docs/` and is treated as normative.
- The Rust implementation lives under `src/crates/` as a Cargo workspace.

Build:

```bash
cargo build
```

Test:

```bash
cargo test
```
