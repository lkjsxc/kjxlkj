# Snippet Support

Code snippets for rapid text insertion.

## Overview

Snippets are templates that expand into larger
code blocks with placeholder values.

## Defining Snippets

### Configuration

Snippets are defined in JSON or TOML files placed in the snippet
directories. Each snippet MUST have a `prefix`, `body`, and MAY
have `description` and `scope` fields.

```json
{
  "Match Arm": {
    "prefix": "match",
    "body": [
      "match ${1:expr} {",
      "    ${2:pattern} => ${3:value},",
      "    $0",
      "}"
    ],
    "description": "Match expression",
    "scope": "rust"
  }
}
```

The `scope` field is a comma-separated list of language
identifiers. If omitted, the snippet applies to all file types.

## Triggering Snippets

### Tab Expansion

In insert mode, type a snippet prefix and press `<Tab>`. The
editor MUST scan loaded snippets for a matching prefix. If
exactly one match exists, it MUST expand immediately. If
multiple prefixes match, a completion menu MUST appear listing
all candidates with their descriptions.

### Explicit Command

`:SnippetExpand {prefix}` -- expand the named snippet at the
cursor position from normal or command mode.
`:SnippetList` -- show all snippets available for the current
buffer's filetype.

## Placeholders

### Syntax

| Syntax | Description |
|--------|-------------|
| `$1` | First placeholder |
| `${1:default}` | With default |
| `${1|a,b,c|}` | Choice list |
| `$0` | Final cursor |

### Navigation

| Key | Action |
|-----|--------|
| `<Tab>` | Next placeholder |
| `<S-Tab>` | Previous placeholder |
| `<Esc>` | Exit snippet |

## Linked Placeholders

### Same Value

Multiple occurrences of the same tab stop number (e.g. `$1`)
MUST be linked. All instances MUST update in real time.

Editing `$1` updates all instances.

## Transformations

### Case Conversion

Transform syntax: `${N/regex/replacement/flags}`.
Built-in case modifiers for replacement strings:

- `${1:/upcase}` -- convert capture to UPPER CASE
- `${1:/downcase}` -- convert capture to lower case
- `${1:/capitalize}` -- capitalize first letter
- `${1:/camelcase}` -- convert to camelCase
- `${1:/pascalcase}` -- convert to PascalCase

### Regex Replace

Transformations use Rust `regex` crate syntax. Capture groups
are referenced as `$1`, `$2`, etc. in the replacement string.

Example: `${1/(.*)_(.*)/$2_$1/}` swaps words around `_`.

## Built-in Snippets

### Rust

| Prefix | Description |
|--------|-------------|
| `fn` | Function |
| `impl` | Implementation |
| `struct` | Struct |
| `enum` | Enum |
| `test` | Test function |

### JavaScript

| Prefix | Description |
|--------|-------------|
| `func` | Function |
| `arrow` | Arrow function |
| `class` | Class |
| `import` | Import statement |

## Custom Snippet Files

### Directory Structure

```
~/.config/kjxlkj/snippets/
  global.json     # all file types
  rust.json       # Rust-only
  python.json     # Python-only
  javascript.json # JavaScript-only
```

Language identifiers MUST match the editor's internal filetype
names (same identifiers used in `[language]` config sections).

### Global Snippets

Apply to all file types.
Defined in `global.json`. If a global snippet prefix conflicts
with a language-specific one, the language-specific snippet MUST
take priority.

## LSP Snippets

### From Language Server

LSP servers can provide snippets in completions.

Completion items with `insertTextFormat: 2` (Snippet) MUST be
parsed using LSP snippet syntax and expanded through the same
snippet engine. LSP snippets MUST support the same tab stops,
placeholders, choices, and variables as user-defined snippets.

## Variables

### Built-in

| Variable | Value |
|----------|-------|
| `$TM_FILENAME` | Current filename |
| `$TM_FILEPATH` | Full path |
| `$TM_LINE_NUMBER` | Line number |
| `$CURRENT_DATE` | Today's date |

### Example

```json
{
  "File Header": {
    "prefix": "header",
    "body": ["$LINE_COMMENT File: $TM_FILENAME", "$LINE_COMMENT $CURRENT_YEAR-$CURRENT_MONTH-$CURRENT_DATE", "$0"],
    "description": "File header comment"
  }
}
```

## Configuration

In `~/.config/kjxlkj/config.toml`:

```toml
[snippets]
enable = true
tab_trigger = true
dirs = ["~/.config/kjxlkj/snippets"]
```

- `enable` -- master toggle (default `true`)
- `tab_trigger` -- `<Tab>` triggers expansion (default `true`)
- `dirs` -- additional snippet directories to search

## Best Practices

1. Use descriptive prefixes
2. Add descriptions
3. Keep snippets focused
4. Use placeholders effectively
