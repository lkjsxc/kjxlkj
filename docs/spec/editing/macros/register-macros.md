# Register Macros

Recording and playing back keystroke sequences.

## Overview

Macros record sequences of keystrokes into registers
and replay them. Record with `q{reg}`, play with `@{reg}`.

## Recording

### Start Recording

`q{a-z}` starts recording into the specified register.
The statusline shows `recording @{reg}`.

### Stop Recording

`q` stops recording. The keystrokes are stored in the
register as a string.

### Append Recording

`q{A-Z}` appends to an existing register (uppercase).
`qA` appends new keystrokes to register `a`.

## Playback

### Execute Macro

`@{a-z}` executes the macro stored in the register.
`@@` repeats the last executed macro.

### With Count

`5@a` executes macro `a` five times.
`100@@` repeats the last macro 100 times.

### Recursive

A macro can call itself: `qa...@aq` creates a
recursive macro. It stops when an error occurs (e.g.,
motion fails at end of file).

## Register Contents

### View

`:registers a` shows the contents of register `a`.
The stored keystrokes are displayed with special key
notation (e.g., `^[` for Escape).

### Edit

`:let @a = "dd"` sets register `a` to the string `dd`.
This effectively creates a macro without recording.

### Append

`:let @A = "p"` appends `p` to register `a`.

## Error Handling

### Stop on Error

Macro playback stops when a command fails. For example,
if `j` is called on the last line, the macro stops.

### Continue on Error

No built-in way to ignore errors during macro playback.
Design macros to avoid errors or use conditional logic.

## Integration

### With :normal

`:%normal! @a` applies macro `a` to every line.
This is the primary way to apply macros to ranges.

### With :global

`:g/pattern/normal! @a` applies a macro to matching lines.

### With Visual Selection

Select lines, then `:normal! @a` applies the macro
to each selected line independently.

## Multiple Macros

### Chaining

`:let @a = @b . @c` combines two macros.

### Calling

Within a macro, `@b` calls another macro. This allows
building complex macros from simpler building blocks.

## Best Practices

### Start Position

Begin recording with the cursor at a predictable
position (e.g., `0` or `^` for start of line).

### End Position

End the macro with the cursor positioned for the next
iteration (e.g., `j0` to move to the next line start).

### Idempotent

Design macros to work correctly when run multiple
times on the same input.
