# File Type Detection

Automatic detection and per-filetype settings.

## Detection Order (normative)

1. **Modeline** — `vim:` or `kjxlkj:` modeline in first/last 5 lines
2. **Shebang** — `#!/usr/bin/env python3` → filetype `python`
3. **Filename** — exact match (`Makefile` → `make`, `Dockerfile` → `dockerfile`)
4. **Extension** — `.rs` → `rust`, `.py` → `python`, `.ts` → `typescript`
5. **Content analysis** — first-line heuristics as fallback

## Built-in Extension Mappings

| Extension | Filetype |
|---|---|
| `.rs` | `rust` |
| `.py` | `python` |
| `.js` | `javascript` |
| `.ts` | `typescript` |
| `.tsx` | `typescriptreact` |
| `.c`, `.h` | `c` |
| `.cpp`, `.hpp` | `cpp` |
| `.go` | `go` |
| `.md` | `markdown` |
| `.toml` | `toml` |
| `.json` | `json` |
| `.yaml`, `.yml` | `yaml` |
| `.lua` | `lua` |
| `.sh`, `.bash` | `sh` |
| `.html` | `html` |
| `.css` | `css` |

## Per-Filetype Settings

Each filetype may configure:

| Setting | Example |
|---|---|
| `tabstop` | 4 for Rust, 2 for YAML |
| `expandtab` | true for most, false for Makefiles |
| `shiftwidth` | Matches tabstop |
| `textwidth` | 80 or 100 |
| `commentstring` | `// %s` for Rust, `# %s` for Python |
| `formatter` | `rustfmt`, `prettier` |
| `lsp.server` | `rust-analyzer`, `pyright` |

## Commands

| Command | Action |
|---|---|
| `:set filetype={ft}` | Override filetype for current buffer |
| `:set filetype?` | Display detected filetype |

## Custom Filetypes

Users can register new filetypes in TOML config by specifying extension, filename, or content patterns and the associated settings.

## Related

- Syntax highlighting: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)
- LSP: [/docs/spec/features/lsp/code-actions.md](/docs/spec/features/lsp/code-actions.md)
