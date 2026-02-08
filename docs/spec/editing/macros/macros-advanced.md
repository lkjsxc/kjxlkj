# Advanced Macros

Complex macro techniques and patterns.

## Overview

Advanced macro techniques beyond basic recording
and playback. Covers editing macros, conditional
execution, and macro debugging.

## Editing Macros

### Via Register

1. `"ap` — paste macro contents from register `a`
2. Edit the pasted text as normal text
3. `0"ay$` — yank the edited line back into register `a`

### Via :let

`:let @a = "0dwj"` sets register `a` directly.
Special characters use escape sequences.

## Conditional Patterns

### Search-Based Stop

A macro that uses `/pattern` will stop when no match
is found. This acts as a conditional break.

### Count-Based

`10@a` runs macro `a` exactly 10 times. If the macro
errors before completion, remaining iterations stop.

## Recursive Macros

### Self-Calling

`qa...@aq` — the macro calls itself at the end. It
continues until an error occurs (natural termination).

### Terminate Conditions

Common termination triggers:
- Motion fails (end/start of file)
- Search finds no match
- Explicitly recorded `<C-c>` (not recommended)

## Parallel Application

### On Multiple Lines

`:%normal! @a` applies macro to every line independently.
Each line starts fresh; errors on one line do not stop
processing of subsequent lines.

### On Selection

Visual select lines, then `:'<,'>normal! @a`.

### On Pattern Matches

`:g/pattern/normal! @a` applies only to matching lines.

## Macro Composition

### Sequential

`:let @c = @a . @b` concatenates macros `a` and `b`
into macro `c`.

### Nested Calls

Within macro `a`, call `@b` to execute macro `b` as
a subroutine. Execution returns to macro `a` after
`b` completes.

## Debugging

### Step Through

There is no built-in macro debugger. Debugging strategy:
1. Paste macro with `"ap`
2. Read each command in the sequence
3. Execute commands one at a time manually
4. Identify the failing step

### Common Issues

- Forgetting to exit insert mode (`<Esc>`)
- Cursor position not matching expectations
- Searching for absent patterns
- Off-by-one in count-based loops

## Persistence

### In Config

Frequently used macros can be stored in config:
`[macros]` section, `a = "0dwj"`, etc.

### Session

Macros in registers persist across sessions via the
session file when session saving is enabled.

## Performance

### Large Repetitions

`10000@a` may be slow. For very large repetitions,
consider using `:g` with `:s` or `:normal` instead.

### Screen Updates

During macro playback, screen updates are suppressed
until the macro completes. This improves performance
significantly.

## Safety

### Undo

The entire macro execution (all iterations) can be
undone with a single `u`.

### Backup

Before running a destructive macro on many lines,
save the file first or use `:earlier {time}`.
