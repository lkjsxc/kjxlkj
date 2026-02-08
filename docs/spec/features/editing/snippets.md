# Snippets

Back: [/docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md)

Template-based text expansion with tabstops and placeholders.

## Overview

Snippets are code templates composed of static text, tabstops, placeholders, choices, and transformations. They integrate with the completion system and can be triggered by typing a prefix.

## Snippet Format

Snippets use LSP snippet syntax (a subset of TextMate snippet syntax).

## Tabstops

| Syntax | Description |
|---|---|
| `$1` | First tabstop |
| `$2` | Second tabstop |
| `$0` | Final cursor position |
| `${1:default}` | Tabstop with default text |

## Placeholders

`${1:name}` — the text `name` is pre-filled and selected. Typing replaces it.

## Choices

`${1|option1,option2,option3|}` — presents a choice menu at the tabstop.

## Variables

| Variable | Value |
|---|---|
| `$TM_FILENAME` | Current filename |
| `$TM_FILEPATH` | Full file path |
| `$TM_DIRECTORY` | Directory of current file |
| `$TM_LINE_NUMBER` | Current line number |
| `$TM_SELECTED_TEXT` | Selected text (visual mode) |
| `$CLIPBOARD` | Clipboard content |
| `$CURRENT_YEAR` | Current year |
| `$UUID` | Generated UUID |

## Transformations

`${1/pattern/replacement/flags}` — regex transform applied to tabstop value.

## Configuration

Snippets are defined in TOML files under `snippets/`:

| Field | Type | Description |
|---|---|---|
| `prefix` | string | Trigger text |
| `body` | string or array | Template lines |
| `description` | string | Shown in completion menu |
| `scope` | string | Comma-separated file types |

## Related

- Insert snippets: [/docs/spec/modes/insert/completion/insert-snippets.md](/docs/spec/modes/insert/completion/insert-snippets.md)
- Completion: [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md)
