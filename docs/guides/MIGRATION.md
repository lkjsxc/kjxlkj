# Neovim Migration Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)
Guidance for Neovim users transitioning to the kjxlkj surface described by the conformance ledger.

## What is similar

kjxlkj is intentionally Vim-like:

- modal editing (Normal / Insert / Visual / Command / Replace)
- operator + motion model (`d{motion}`, `c{motion}`, `y{motion}`)
- text objects, registers, marks, macros (subset; see conformance)

For the authoritative “what exists right now” ledger, see:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## What is different

### No plugin ecosystem

kjxlkj has no plugin system. Features are either built-in (when implemented) or not available yet.

### No Neovim Lua config

kjxlkj does not execute Neovim configuration. Persistent configuration/key remapping is a target but is not implemented yet.

### Documentation contract

See: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)

## Practical mapping

### Editing workflow

- Use Normal/Insert/Visual just as in Vim.
- Use the command line (`:`) for file and Ex commands.
- Use `.` to repeat the last change.

### Files

- Open a file: launch with a file path argument, or use `:e {file}`.
- Write: `:w {file}` (and `:wq` / `:x` to write then quit).
- Quit: `:q` (or `:q!` to force).

### Search and replace

- Search: `/` and `?`, then `n`/`N`.
- Substitute: `:s/pat/repl/` and `:s/pat/repl/g`.
- Global: `:g/pat/d` and `:v/pat/d` (subset).

## What you will miss (for now)

These are specified as targets but are not implemented yet:

- LSP client features
- git UI/integration
- syntax highlighting and diagnostics UI
- built-in file explorer and fuzzy finder UIs
- splits/windows/tabs and multi-buffer management
- persistent configuration and key remapping

See:

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Recommended reading

- Docs index: [/docs/README.md](/docs/README.md)
- Target keybindings spec: [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
