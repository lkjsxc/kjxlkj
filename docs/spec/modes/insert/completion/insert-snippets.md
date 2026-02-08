# Insert Snippets

Back: [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md)

Snippet expansion and navigation in insert mode.

## Overview

Snippets are templates with tabstops, placeholders, and transformations. When a snippet is expanded, the user can tab through tabstops to fill in values.

## Trigger

| Source | Trigger |
|---|---|
| Completion menu | Select snippet item, press `<CR>` |
| Manual | Type trigger word, press `<Tab>` (if configured) |
| LSP | LSP server provides snippet completions |
| Command | `:InsertSnippet {name}` |

## Tabstops

| Syntax | Description |
|---|---|
| `$1`, `$2`, ... | Tabstop positions (in order) |
| `$0` | Final cursor position |
| `${1:default}` | Tabstop with default text |
| `${1\|choice1,choice2\|}` | Tabstop with choices |

## Navigation

| Key | Action |
|---|---|
| `<Tab>` | Jump to next tabstop |
| `<S-Tab>` | Jump to previous tabstop |
| `<Esc>` | Exit snippet mode |
| `<CR>` | Confirm and stay at current tabstop |

## Placeholder Behavior

When jumping to a tabstop with a placeholder, the placeholder text is selected (visual-select). Typing replaces the placeholder.

## Mirror

`${1}` used multiple times mirrors the text — editing at one location updates all.

## Transformations

`${1/pattern/replacement/flags}` — applies regex transformation to the tabstop value.

## Snippet Definitions

Snippets are defined in TOML configuration:

| Field | Description |
|---|---|
| `prefix` | Trigger text |
| `body` | Template with tabstops |
| `description` | Shown in completion menu |
| `scope` | File types where this snippet applies |

## Related

- Completion: [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md)
- Snippets: [/docs/spec/features/editing/snippets.md](/docs/spec/features/editing/snippets.md)
