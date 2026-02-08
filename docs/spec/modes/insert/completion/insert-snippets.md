# Snippet Expansion

Template-based code insertion.

## Overview

Snippets expand short triggers into larger code templates.

## Basic Usage

### Trigger

Type a snippet prefix in insert mode and press `<Tab>`. The editor
MUST check if the word before the cursor matches a known snippet
prefix. If matched, the prefix MUST be deleted and replaced with
the expanded snippet body.

### Example

Typing `fn` then `<Tab>` in a Rust buffer expands to a function template: `fn ${1:name}(${2:params}) { $0 }`. Cursor lands at first placeholder.

## Snippet Structure

### Basic Snippet

A snippet is a JSON object with a name key containing `prefix`, `body` (array of strings), `description`, and optional `scope` (language ID).

Parts:

| Field | Purpose |
|---|---|
| `prefix` | Trigger text |
| `body` | Expanded content (array of lines) |
| `description` | Shown in completion menu |
| `scope` | Comma-separated language IDs; omit for all languages |
| `$1, $2` | Tab stops |
| `$0` | Final cursor |

## Tab Stops

### Navigation

| Key | Action |
|-----|--------|
| `<Tab>` | Jump to next tab stop |
| `<S-Tab>` | Jump to previous tab stop |
| `<Esc>` | Exit snippet, keep text, return to normal mode |

While navigating, the editor MUST remain in a snippet-active
insert sub-mode. Active tab stops MUST be visually highlighted.

### Order

Visit in numerical order: `$1` -> `$2` -> `$3` -> `$0`.

### Final Position

`$0` is final cursor position. When reached, the snippet session
MUST end and regular insert mode MUST resume.

## Placeholders

### With Default

Syntax: `${N:default_text}`. Placeholder text MUST be pre-filled
and selected so typing replaces it immediately.

### Example

`"body": "let ${1:variable} = ${2:value};"` expands to
`let variable = value;` with `variable` selected.

### Overwrite

Type to replace placeholder text.

## Choice Placeholders

### Syntax

`${N|choice1,choice2,choice3|}` -- the editor MUST display a
dropdown menu listing each choice when the tab stop is active.

### Behavior

Shows dropdown with choices. Selecting a choice MUST insert that
text and advance to the next tab stop.

### Example

`"body": "let ${1:name}: ${2|i32,u32,f64,String|} = $0;"`
At `$2`, a dropdown offers `i32`, `u32`, `f64`, `String`.

## Variable Substitution

### Built-in Variables

| Variable | Value |
|----------|-------|
| `$TM_FILENAME` | Current file name (e.g. `main.rs`) |
| `$TM_FILEPATH` | Full file path |
| `$TM_DIRECTORY` | Directory of current file |
| `$TM_FILENAME_BASE` | File name without extension |
| `$TM_LINE_NUMBER` | Current line number (1-based) |
| `$TM_SELECTED_TEXT` | Currently selected text or empty |
| `$CLIPBOARD` | Clipboard contents |
| `$CURRENT_YEAR` | Four-digit year |
| `$CURRENT_MONTH` | Two-digit month |
| `$CURRENT_DATE` | Two-digit day |
| `$BLOCK_COMMENT_START` | Language block comment start |
| `$LINE_COMMENT` | Language line comment prefix |

Variables MUST resolve at expansion time. Unknown variables
MUST be replaced with an empty string.

### Example

`"body": "// $TM_FILENAME | $CURRENT_YEAR-$CURRENT_MONTH-$CURRENT_DATE\n$0"`

## Mirror Placeholders

### Same Value

Use the same tab stop number in multiple positions. All
occurrences of `$1` MUST mirror text entered at the first `$1`.

### Behavior

Both update simultaneously.

### Example

`"body": "struct ${1:Name} {\n}\nimpl ${1:Name} {\n    $0\n}"`
Typing `Point` fills both `struct Point` and `impl Point`.

## Transform Placeholders

### Syntax

`${N/regex/replacement/flags}` -- applies a regex transform to
tab stop N. Flags: `i` (case-insensitive), `g` (global).
Replacement supports `${1:/upcase}`, `${1:/downcase}`,
`${1:/capitalize}` for capture groups.

### Example

`"body": "${1:name} -> ${1/(.*)/${1:/upcase}/}"`
Typing "Hello" produces: `Hello -> HELLO`

## Snippet File Format

### JSON

Top-level keys are snippet names. Each value is an object with `prefix`, `body` (array of strings), and `description`. Example: a snippet named `"For Loop"` with prefix `"for"`, body `["for ${1:item} in ${2:iter} {", "    $0", "}"]`, and description `"For-in loop"`.

### TOML

Each snippet is a TOML table under `[snippets.{name}]` with `prefix`, `body` (string using `\n` for newlines), and `description`.

## File Location

Snippet files MUST be discovered (highest priority first):

| Priority | Path | Scope |
|---|---|---|
| 1 | `.kjxlkj/snippets/{language}.json` | project-local |
| 2 | `~/.config/kjxlkj/snippets/{language}.json` | per-language |
| 3 | `~/.config/kjxlkj/snippets/global.json` | all file types |
| 4 | LSP completions with `insertTextFormat: 2` | from language server |

Project-local snippets override global ones on prefix conflict.
