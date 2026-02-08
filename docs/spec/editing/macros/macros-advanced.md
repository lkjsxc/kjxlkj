# Advanced Macros

Back: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)

Advanced macro techniques and edge cases.

## Overview

Beyond basic macro recording and playback, macros support recursive invocation, editing, appending, and interaction with other features like visual mode and counts.

## Appending to a Macro

`q{A-Z}` — recording with an uppercase register letter appends to the existing macro in that register.

`qa` records macro a. Later, `qA` appends additional commands to macro a.

## Recursive Macros

A macro can call itself: record a macro in register `a` that ends with `@a`. The macro repeats until an error occurs (e.g., end of file).

Example: `qadd@aq` — deletes a line and recursively repeats until no lines remain.

## Editing Macros

Macros are stored in registers as text. To edit:

1. `"ap` — paste the macro text.
2. Edit the text.
3. `"ayy` — yank the edited text back into register `a`.

## Macro with Count

`10@a` — execute macro `a` 10 times. Stops early if the macro encounters an error.

## Last Macro

`@@` replays the last executed macro. Useful for quickly repeating a just-run macro.

## Visual Mode

Select lines with `V`, then `:normal @a` — executes macro `a` on each selected line.

## Macro Registers

Macros are stored in named registers (`a`-`z`). They share storage with yank/delete registers.

## Session Persistence

Macro registers are saved in the session file.

## Error Behavior

If any command in a macro fails (e.g., search finds no match), macro execution stops immediately.

## Related

- Macros: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)
- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
