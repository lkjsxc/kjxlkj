# kjxlkj Quick Start Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)

Practical first-run guide for the currently reconstructed surface.

## Before You Start

Check current status first:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

If the repo is docs-only (no workspace artifacts), reconstruct first:

- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/checklists/README.md](/docs/todo/checklists/README.md)

## Build and Run

1. Install Rust stable (or pinned toolchain if `rust-toolchain.toml` exists).
2. Build with `cargo build`.
3. Run with `cargo run -- <optional-file-path>`.

If Docker artifacts exist, see:

- [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)

## Common First Commands

Use these only if current conformance marks the command surface as supported.

| Workflow | Typical command |
|---|---|
| Open file | `:e {file}` |
| Write file | `:w` or `:w {file}` |
| Write + quit | `:wq` or `:x` |
| Quit | `:q` or `:q!` |

## Expected Gaps

Feature availability is reconstruction-dependent and must be read from ledgers.

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Related

- Docs index: [/docs/README.md](/docs/README.md)
- Keybinding target: [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
