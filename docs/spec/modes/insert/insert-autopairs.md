# Insert Auto-Pairs

Automatic bracket and quote pairing.

## Overview

When enabled, typing an opening delimiter automatically
inserts the matching closing delimiter and positions the
cursor between them.

## Supported Pairs

### Default Pairs

| Open | Close | Name |
|------|-------|------|
| `(` | `)` | Parentheses |
| `[` | `]` | Brackets |
| `{` | `}` | Braces |
| `"` | `"` | Double quotes |
| `'` | `'` | Single quotes |
| `` ` `` | `` ` `` | Backticks |

### Language-Specific

| Language | Additional Pairs |
|----------|-----------------|
| HTML/XML | `<` / `>` (in tag context) |
| Rust | `\|` / `\|` (closures) |
| Python | `"""` / `"""` (docstrings) |

## Behavior

### Insert Opening

Typing `(` inserts `()` with cursor between them.
Typing `{` after `<CR>` also adds indented newline.

### Skip Closing

If the cursor is before a closing delimiter that matches
an already-paired opener, typing that closer moves the
cursor past it instead of inserting a duplicate.

### Delete Pair

When `<BS>` is pressed between an empty pair (`()`),
both delimiters are deleted.

### Wrap Selection

In visual mode, typing an opening delimiter wraps
the selection: `(selection)`.

## Smart Behavior

### Context Awareness

Auto-pairs do not activate inside:
- String literals (no auto-pair for `'` inside `"..."`)
- Comments (language-dependent)
- After `\` (escaped character)

### Word Boundary

For quote characters (`"`, `'`, `` ` ``), auto-pairing
only occurs when:
- After whitespace or opening bracket
- At start of line
- NOT after word characters (avoids pairing in contractions)

## Configuration

| Option | Default | Description |
|--------|---------|-------------|
| `auto_pairs` | `true` | Enable/disable globally |
| `auto_pairs_map` | (see above) | Custom pair definitions |

### Per-Filetype

Auto-pairs can be configured per filetype in language
settings to add or remove pairs.

### Disable Specific

To disable a specific pair, set its value to `false`
in the auto_pairs_map configuration.

## Multi-Line

### Brace Newline

Typing `{<CR>` produces:
```
{
  |
}
```
Where `|` is cursor position, indented appropriately.

### Smart Indent

The closing delimiter is placed at the same indent
level as the opening delimiter's line.

## Undo

### Single Undo

Auto-pair insertion is part of the same undo group
as the character typed. `u` removes both delimiters.

## Integration

### With Completion

When auto-completion inserts a function name followed
by `(`, auto-pairs adds the closing `)`.

### With Snippets

Snippet expansion may include pairs. Auto-pairs
defers to snippet handling in that case.
