# Normal Command

Back: [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)

Execute normal mode keystrokes from the command line.

## Overview

`:normal {keys}` feeds `{keys}` into the editor as if typed in normal mode. This is powerful for scripting repetitive edits.

## Basic Syntax

`:normal[!] {keys}`

The `!` modifier bypasses user mappings.

## Simple Examples

| Command | Effect |
|---|---|
| `:normal dd` | Delete current line |
| `:normal A;` | Append `;` to current line |
| `:normal 0i// ` | Insert `// ` at start of current line |

## With Range

| Command | Effect |
|---|---|
| `:%normal A;` | Append `;` to every line |
| `:10,20normal dd` | Delete lines 10-20 |
| `:'<,'>normal @a` | Execute macro `a` on visual selection |

## Bang Modifier

`:normal! {keys}` ignores user-defined mappings and uses default key meanings.

Always use `!` for robust, predictable behavior independent of user configuration.

## Special Keys

Use `\<keyname>` notation or `execute` for special keys:

| Example | Description |
|---|---|
| `:exe "normal! \<C-a>"` | Increment number |
| `:exe "normal! /pattern\<CR>"` | Search for pattern |
| `:exe "normal! O\<Esc>"` | Insert blank line above |

## Common Uses

| Pattern | Effect |
|---|---|
| `:%normal! I# ` | Comment all lines (prepend `# `) |
| `:%normal! A,` | Append `,` to all lines |
| `:%normal! $x` | Remove last character of every line |
| `:%normal! ==` | Re-indent every line |

## With Global

| Command | Effect |
|---|---|
| `:g/TODO/normal! dd` | Delete all lines containing TODO |
| `:g/^$/normal! O// blank` | Insert comment above blank lines |

## Macros

`:normal @a` replays macro `a` on each line in the range.

| Command | Effect |
|---|---|
| `:%normal @q` | Run macro q on all lines |
| `:g/pattern/normal @a` | Run macro a on matching lines |

## Motion Commands

| Command | Effect |
|---|---|
| `:normal! gg=G` | Re-indent entire file |
| `:normal! /foo` | Search for foo |

## Insert Mode

`:normal! ifoo` enters insert mode, types `foo`, but does NOT exit insert mode (use `\<Esc>` via `:execute` to add the escape).

## Visual Mode

`:normal! viw` enters visual mode and selects a word. Combined with operators: `:normal! viwd` selects a word and deletes it.

## Operators

`:normal! d$` deletes to end of line. `:normal! gUiw` uppercases current word. All operator-motion combinations work.

## Related

- Execute: [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)
- Global command: [/docs/spec/commands/substitute/global-command.md](/docs/spec/commands/substitute/global-command.md)
- Macros: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)
