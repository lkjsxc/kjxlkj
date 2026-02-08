# Operator-Pending Mappings

Custom operator targets.

## Overview

Operator-pending mappings define custom motions and text
objects for operators. They execute in operator-pending mode
(after `d`, `c`, `y`, etc.) and define a region for the
operator to act on.

## Operator-Pending Mode

### Entry

Triggers after operators like `d`, `c`, `y`, `>`, `<`, `=`.
The mapping runs in operator-pending context and must define
a selection (via visual mode or cursor movement).

## Basic Mappings

### Configuration

In TOML config under `[keys.operator]`:
each key maps to a command or key sequence that defines
the motion or text object.

### Usage

After typing an operator, the operator-pending mapping
keys are available. `dil` (delete inner line) if `il`
is defined as an omap.

## Custom Motions

### Inner Line

`il` selects the line content without leading/trailing
whitespace and without the newline. Implementation:
jump to first non-blank, enter visual mode, select to
last non-blank character.

Usage: `dil` deletes inner line content (preserves indent).

### Around Line

`al` selects the entire line content (column 0 to end).
Not the same as `dd` which is linewise; `dal` is charwise.

Usage: `dal` deletes entire line content but preserves
the line itself.

## Text Objects

### Inner/Around Pattern

Custom text objects follow the `i{char}` / `a{char}` convention.
The mapping defines the selection region using visual mode.

### Custom Pairs

User-defined delimiter pairs. For example, `i|` to select
text between pipe characters. Configuration defines
the opening and closing characters.

## Numeric Arguments

### Preserve Count

Operator-pending mappings receive the count from the
operator. `d3n` passes count 3 to the `n` omap.
The mapping should use the count to extend the selection.

## Function-Based

### Complex Logic

Operator-pending mappings can call script functions that
inspect buffer content and set the visual selection
programmatically.

### Tree-Sitter Based

Language-aware text objects use tree-sitter queries to
find the nearest enclosing node of a given type and
select it.

## Common Text Objects

### Inner Entire File

`ie` selects the entire buffer content from first to last
line (charwise). Usage: `die` deletes entire buffer.
`yie` yanks entire buffer.

### Inner Indentation

`ii` selects all contiguous lines with the same or greater
indentation as the current line. `ai` includes the line
above and below the indentation block.

## With Visual Mode

### Paired Definitions

Operator-pending mappings should have matching visual mode
mappings defined under `[keys.visual]` so that both
`vie` and `die` work.

Both `vie` and `die` use the same selection logic.

## Search-Based

### To Next Match

`d/pattern<CR>` is a built-in: delete from cursor to the
next match of pattern. Custom omap `n` can use the last
search pattern to define the motion target.

### To Character

Custom motions to specific characters or patterns can be
defined as operator-pending mappings.

## Indent Level

### Same Indentation

`ii` selects consecutive lines at the current indent level.
Blank lines do not break the selection. Useful for
Python code blocks.

## URL Text Object

### Select URL

`iu` selects the URL under the cursor. Detection uses
regex matching `https?://[^\s>)]+`.

## Number Text Object

### Select Number

`in` selects the number under or after the cursor.
Supports integers, floating point, hex (`0x...`), and
binary (`0b...`).

## Sentence/Paragraph

### Enhanced

Built-in `is`/`as` (sentence) and `ip`/`ap` (paragraph)
can be overridden with tree-sitter-aware versions for
more accurate selection in code (vs. prose).
