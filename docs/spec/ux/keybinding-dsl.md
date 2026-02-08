# Keybinding DSL

Back: [/docs/spec/ux/README.md](/docs/spec/ux/README.md)

Domain-specific language for defining key bindings in configuration.

## Overview

Key bindings are defined in TOML configuration using a structured format that specifies mode, key sequence, action, and optional conditions.

## Format

Each binding is a table entry under the `keys` section:

| Field | Type | Required | Description |
|---|---|---|---|
| `mode` | string | Yes | Mode(s): `"n"`, `"i"`, `"v"`, `"nv"`, etc. |
| `key` | string | Yes | Key sequence (e.g., `"<leader>w"`, `"<C-s>"`) |
| `action` | string | Yes | Command to execute |
| `desc` | string | No | Description for which-key display |
| `when` | string | No | Conditional expression |

## Key Notation

| Notation | Key |
|---|---|
| `<C-x>` | Ctrl + x |
| `<S-x>` | Shift + x |
| `<A-x>` / `<M-x>` | Alt/Meta + x |
| `<CR>` | Enter |
| `<Esc>` | Escape |
| `<Tab>` | Tab |
| `<Space>` | Space |
| `<leader>` | Leader key (configurable) |

## Conditions

The `when` field accepts expressions:

| Expression | True when |
|---|---|
| `filetype == "rust"` | File type is Rust |
| `modified` | Buffer is modified |
| `readonly` | Buffer is read-only |

## Related

- Key mappings: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)
- Configuration: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
