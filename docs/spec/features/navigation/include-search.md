# Include/Define Search

Search for include/import statements and symbol definitions.

## Include Search Keybindings

| Key | Action |
|---|---|
| `[i` | Display first line matching word under cursor in included files |
| `]i` | Display next match after cursor position |
| `[I` | List all matching lines |
| `]I` | List matches after cursor position |

## Define Search Keybindings

| Key | Action |
|---|---|
| `[d` | Display first definition of word under cursor |
| `]d` | Display next definition after cursor |
| `[D` | List all definitions |
| `]D` | List definitions after cursor |

## Commands

| Command | Action |
|---|---|
| `:isearch {pattern}` | Search included files for pattern |
| `:ilist {pattern}` | List matches in included files |
| `:dsearch {pattern}` | Search for definition |
| `:dlist {pattern}` | List definitions |
| `:ijump {pattern}` | Jump to match in included file |
| `:djump {pattern}` | Jump to definition |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `include` | `"^\\s*#\\s*include"` | Pattern to detect include lines |
| `define` | `"^\\s*#\\s*define"` | Pattern to detect definitions |
| `path` | `".,,,"` | Search path for included files |

These patterns are language-dependent. Set per-filetype for best results:

| Language | Include Pattern | Define Pattern |
|---|---|---|
| C/C++ | `^\s*#\s*include` | `^\s*#\s*define` |
| Rust | `^\s*use\s` | `^\s*\(pub\s\+\)\?\(fn\|struct\|enum\|type\)` |
| Python | `^\s*\(import\|from\)` | `^\s*\(def\|class\)` |

## Search Scope

Include search follows the include chain: it finds files referenced by the current file's include/import lines, then searches those files recursively. The `path` option controls where to look for included files.

## LSP Integration

When an LSP server is available, prefer LSP-based "go to definition" (`<C-]>`) over include/define search for more accurate results.

## Related

- Tags: [/docs/spec/features/navigation/tags.md](/docs/spec/features/navigation/tags.md)
- Search: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
