# Grep Integration

Project-wide search using external grep tools.

## Commands (normative)

| Command | Action |
|---|---|
| `:grep {pattern}` | Search with external grep, populate quickfix |
| `:lgrep {pattern}` | Search, populate location list |
| `:vimgrep {pattern} {files}` | Internal search (Vim regex syntax) |
| `:grepadd {pattern}` | Append results to existing quickfix |

## Quickfix Navigation

| Key | Action |
|---|---|
| `:copen` | Open quickfix window |
| `]q` / `:cnext` | Next result |
| `[q` / `:cprev` | Previous result |
| `:cc {n}` | Go to result number n |

## Grep Program Configuration

| Option | Description |
|---|---|
| `grepprg` | External program (default: `grep -rn`) |
| `grepformat` | Output parsing format (default: `%f:%l:%m`) |

### Recommended: ripgrep

Set `grepprg = "rg --vimgrep --smart-case"` and `grepformat = "%f:%l:%c:%m"` for fast, smart-case project search with column numbers.

## Grepformat Tokens

| Token | Meaning |
|---|---|
| `%f` | Filename |
| `%l` | Line number |
| `%c` | Column number |
| `%m` | Message/content |

## Async Grep

Grep runs asynchronously by default. Results populate the quickfix list incrementally as they arrive. The editor remains responsive during search.

## Keybindings

| Key | Action |
|---|---|
| `<leader>gg` | Grep word under cursor |
| `<leader>gw` | Grep whole word under cursor |
| `<leader>gv` | Grep visual selection |

## :vimgrep vs :grep

| Feature | `:grep` | `:vimgrep` |
|---|---|---|
| Speed | Fast (external tool) | Slower (internal) |
| Pattern syntax | Tool-specific (ripgrep, PCRE) | Vim regex |
| Multi-line | Depends on tool | Supported |

## Related

- Search patterns: [/docs/spec/editing/search/search-patterns.md](/docs/spec/editing/search/search-patterns.md)
- Star search: [/docs/spec/editing/search/star-search.md](/docs/spec/editing/search/star-search.md)
