# Text Objects

Operate on logical units of text.

## Overview

Text objects allow operations on semantic units
like words, sentences, paragraphs, and code blocks.

## Syntax

Text objects use a two-character prefix: `i` (inner) or `a` (around), followed
by the object type. `i` selects content only; `a` includes delimiters/whitespace.
Text objects cannot be used as standalone motions -- they require an operator or
visual mode. Grammar: `{operator}{a|i}{object}`.

## Built-in Objects

### Word Objects

| Object | Description |
|--------|-------------|
| `w` | Word |
| `W` | WORD (space-separated) |

Given `hello_world foo` with cursor on `e`:
- `diw` deletes `hello_world` (just the word)
- `daw` deletes `hello_world ` (word + trailing space)
- `diW` same as `diw` here (no adjacent punctuation)
- `ciw` deletes word, enters insert mode at word position

### Block Objects

| Object | Description |
|--------|-------------|
| `(` `)` | Parentheses |
| `[` `]` | Brackets |
| `{` `}` | Braces |
| `<` `>` | Angle brackets |

Given `fn(a, b)` with cursor on `a`:
- `di(` or `di)` deletes `a, b`, leaving `fn()`
- `da(` deletes `(a, b)`, leaving `fn`
- `ci[` on `arr[idx]` deletes `idx`, enters insert inside `arr[]`
- Blocks nest: `di{` finds innermost enclosing `{}`

### Quote Objects

| Object | Description |
|--------|-------------|
| `"` | Double quotes |
| `'` | Single quotes |
| `` ` `` | Backticks |

Given `let s = "hello world";` with cursor on `w`:
- `di"` deletes `hello world`, leaving `let s = "";`
- `da"` deletes `"hello world"`, leaving `let s = ;`
- `ci"` deletes content inside quotes, enters insert between them

### Other Objects

| Object | Description |
|--------|-------------|
| `s` | Sentence |
| `p` | Paragraph |
| `t` | HTML/XML tag |

- `dis` -- delete inner sentence (text only, keep surrounding space)
- `dap` -- delete paragraph including trailing blank line
- `dit` -- delete content between HTML tags: `<p>text</p>` becomes `<p></p>`
- `dat` -- delete entire tag pair: `<p>text</p>` is removed

## Tree-sitter Objects

Tree-sitter objects use language-aware parsing. Prefix: `if`/`af` for function,
`ic`/`ac` for class, `ia`/`aa` for argument, `ix`/`ax` for comment.

### Function

- `daf` -- delete entire function including signature and body
- `dif` -- delete function body only (keep signature and braces)
- `vaf` -- visually select whole function for inspection

### Class

- `dac` -- delete entire class/struct definition
- `dic` -- delete class body only (keep `class Name {` and `}`)
- `vic` -- select class body for refactoring

### Argument

- `daa` -- delete argument and separator (comma + space)
- `dia` -- delete argument value only, keep comma
- `cia` -- change an argument: delete and enter insert mode

### Comment

- `dax` -- delete comment including leading whitespace
- `dix` -- delete comment text only (keep `//` or `/* */` delimiters)

## Seeking

### Forward

Objects search forward if cursor not inside a matching pair:
- `di"` with cursor before any quote searches forward for the next quoted string
- `di(` with cursor outside parens finds next `(...)` on the same line
- Seeking only works within the current line for quote and block objects
- Paragraph and sentence objects always operate relative to cursor position

## Configuration

Text object behavior is configured through these settings:
- `textobj_seek_range` (default: current line) -- how far to seek for objects
- Tree-sitter objects require a language parser to be loaded for the buffer
- Custom objects are registered via the `register_textobj(key, finder_fn)` API

## Custom Objects

### Entire File

`ie` / `ae` -- select entire buffer content:
- `dae` -- delete all text in the buffer
- `yie` -- yank entire file content (no trailing newline with `i`)
- Implemented as range from first to last line

### Line

`il` / `al` -- select current line:
- `dil` -- delete line content (keep newline)
- `dal` -- delete line content and newline (like `dd`)
- `yil` -- yank line content without newline character

## Visual Mode

### Select Object

In visual mode, pressing a text object selects it:
- `viw` -- select current word
- `vi(` -- select content inside parentheses
- `vaB` -- select around braces including the braces

### Extend Selection

Repeating a text object in visual mode expands to the next enclosing scope:
- `vi(` then `i(` again -- expand to the outer parentheses pair
- `vib` then `ab` -- expand from inner to around the block
- This allows incremental selection expansion

## Object Motion

### Around Objects

`a` objects include surrounding context useful for deletion:
- `daw` -- removes word and one adjacent space (no double-space left behind)
- `da"` -- removes quotes and their content
- `dap` -- removes paragraph and one adjacent blank line
- Rule: trailing whitespace/delimiter is preferred; leading is used at end of line

## Tips

1. Use `i` for content, `a` for delimiters
2. Tree-sitter objects are language-aware
3. Objects work with all operators
4. Visual mode to preview selection

See also: [/docs/design/editing/operators-detailed.md](/docs/design/editing/operators-detailed.md),
[/docs/design/editing/motions-detailed.md](/docs/design/editing/motions-detailed.md)
