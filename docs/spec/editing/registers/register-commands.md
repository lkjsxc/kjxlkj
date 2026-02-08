# Register Commands

Ex commands for viewing and manipulating registers.

## Viewing Registers

### :registers / :reg

Display register contents. Without arguments, shows all
non-empty registers. With arguments, shows only the
specified registers: `:reg a b 0 "+`.

### :display / :di

Alias for `:registers`. Identical behavior.

## Output Format

| Column | Content |
|--------|---------|
| Type | `c` (characterwise), `l` (linewise), `b` (blockwise) |
| Name | Register character (`"`, `0`-`9`, `a`-`z`, etc.) |
| Content | First 50 characters of register value |

Control characters are shown in caret notation (`^M`).
Newlines in linewise registers are shown as `^J`.

## Pasting Registers

### :put

`:put {reg}` pastes register contents on a new line
below the current line. `:put! {reg}` pastes above.
Always pastes linewise regardless of register type.

### Range with :put

`:{line}put {reg}` pastes after the specified line.
`:0put a` pastes register `a` before line 1.
`:$put a` pastes after the last line.

## Setting Registers

### :let @{reg}

Set register contents directly with the expression
command: `:let @a = "hello world"`. The register
type becomes characterwise. Use `\n` for newlines.

### Appending

`:let @A = " more"` appends to register `a`.
Uppercase register name always appends.

## Executing Registers

### :@{reg}

Execute register contents as ex commands, one line
at a time. `:@a` runs the content of register `a`.
`:@@` re-executes the last executed register.

### Example

Store `:%s/foo/bar/g` in register `a`, then `:@a`
performs the substitution.

## Register Operations

### Clear Register

Set to empty string: `:let @a = ""`.
Recording an empty macro also clears: `qa` then `q`.

### Clear All Named Registers

No built-in command. Script: iterate `a`-`z` and
set each to `""`.

### Copy Between Registers

`:let @b = @a` copies register `a` into `b`.

## Interactive Register Selection

In normal mode, typing `"` then pausing shows a popup
listing all non-empty registers with previews.
The `which-key` timeout applies.

## Register in Substitution

Use register content in a substitution replace string
with `\=@{reg}`: `:%s/pattern/\=@a/g` replaces with
the contents of register `a`.

## Register in Expressions

In the expression register (`<C-r>=`), registers are
accessed as `@a`, `@"`, `@+`, etc. The `@` prefix
returns the register contents as a string.

## Macros and Registers

Macros are stored in registers. `qa` records into `a`,
`q` stops recording, `@a` replays. To edit a macro:
1. `:let @a = "` then paste, edit, close quote
2. Or `"ap` to paste, edit the text, then `"ayy` to re-yank

## Configuration

The `clipboard` option controls `"*` and `"+` behavior.
Set `clipboard = "unnamedplus"` to make all yank/delete
operations use the system clipboard.

## Keybindings

| Key | Action |
|-----|--------|
| `"` | Prefix for register selection |
| `<C-r>{reg}` | Paste register in insert/cmdline mode |
| `<C-r><C-r>{reg}` | Paste register literally (no mapping) |
| `<C-r><C-o>{reg}` | Paste register literally, fix indent |

## API Reference

| Function | Return |
|----------|--------|
| `getreg({name})` | Register contents as string |
| `setreg({name}, {value})` | Set register |
| `getregtype({name})` | Register type (`v`, `V`, `<C-v>`) |
