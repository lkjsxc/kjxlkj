# Insert Abbreviations

Automatic text replacements triggered by typing.

## Overview

Abbreviations automatically expand typed shorthand into
longer text when followed by a non-keyword character.
They are defined with the `:abbreviate` (`:ab`) command.

## Definition Commands

### Create Abbreviation

| Command | Mode | Description |
|---------|------|-------------|
| `:abbreviate {lhs} {rhs}` | Insert + Command | Both modes |
| `:iabbrev {lhs} {rhs}` | Insert only | Insert mode only |
| `:cabbrev {lhs} {rhs}` | Command only | Command line only |

### Remove Abbreviation

| Command | Description |
|---------|-------------|
| `:unabbreviate {lhs}` | Remove from both modes |
| `:iunabbrev {lhs}` | Remove insert mode only |
| `:cunabbrev {lhs}` | Remove command line only |

### Clear All

| Command | Description |
|---------|-------------|
| `:abclear` | Clear all abbreviations |
| `:iabclear` | Clear insert mode only |
| `:cabclear` | Clear command line only |

## Trigger

### Trigger Characters

Abbreviations expand when a non-keyword character is
typed after the abbreviation text. Trigger characters
include: space, enter, tab, and punctuation.

### Keyword Characters

Keyword characters are letters, digits, and underscore.
These do NOT trigger expansion.

### Escape Prevention

Typing `<C-]>` immediately triggers abbreviation
expansion. `<Esc>` leaves insert mode without expanding.

## Types

### Full-id

All characters in `lhs` are keyword characters.
Example: `:iabbrev teh the`

### End-id

Last character of `lhs` is a keyword character but
earlier characters are not all keyword characters.
Example: `:iabbrev #i #include`

### Non-id

Last character of `lhs` is not a keyword character.
Example: `:iabbrev ;; <Esc>`

## Matching

### Case Sensitivity

Abbreviation matching is case-sensitive by default.
`:iabbrev Teh The` only matches `Teh`, not `teh`.

### Word Boundary

The abbreviation must be preceded by a non-keyword
character or be at the start of a line. This prevents
expansion inside words.

## Expression Abbreviations

### Dynamic Replacement

Using `<expr>` flag allows dynamic expansion:
`:iabbrev <expr> dt strftime("%Y-%m-%d")`

### Evaluation

The `rhs` is evaluated as a script expression each
time the abbreviation triggers. The result string
replaces the typed text.

## Special Keys

### Key Notation in RHS

| Notation | Effect |
|----------|--------|
| `<CR>` | Newline |
| `<Esc>` | Escape (leave insert) |
| `<C-o>` | Insert-normal command |
| `<BS>` | Backspace |

### Multi-line

Use `<CR>` in `rhs` to create multi-line expansions.

## Listing

### Show All

`:abbreviate` with no arguments lists all abbreviations.

### Output Format

Output shows mode indicator, lhs, and rhs:
`i  teh          the`
`!  btw          by the way`

### Filter

`:abbreviate {prefix}` lists only abbreviations starting
with `{prefix}`.

## Buffer-Local

### Local Abbreviations

`:iabbrev <buffer> lhs rhs` creates a buffer-local
abbreviation that only applies in the current buffer.

### Use Cases

Buffer-local abbreviations are useful for filetype-specific
expansions (e.g., language keywords, common patterns).

## No-remap

### Recursive

By default, abbreviations may trigger other abbreviations
recursively.

### Non-recursive

`:inoreabbrev lhs rhs` prevents recursive expansion.
Always prefer `noreabbrev` variants to avoid surprises.

## Undo

### Undo Expansion

After an abbreviation expands, pressing `<C-z>` or
`u` (after returning to normal mode) undoes the expansion
as a single undo unit.

### Inline Undo

`<C-w>` in insert mode may delete back through the
expanded text, but this is word-by-word, not expansion-aware.

## Configuration

### In Config File

Abbreviations are stored in config as:

`[abbreviations.insert]` section with `lhs = "rhs"` pairs.

### Loading

Abbreviations from config are loaded at startup.
Buffer-local abbreviations are applied via filetype
configuration.
