# Recursive Macros

Back: [docs/spec/editing/macros/README.md](docs/spec/editing/macros/README.md)

Macros that invoke themselves to create loops.

## Overview

A recursive macro calls itself during playback,
repeating its actions until an error stops execution.
This is a powerful pattern for batch editing.

## Basic Pattern

### Recording

1. `qa` - Start recording into register `a`
2. Perform the editing action
3. Move to the next target
4. `@a` - Call self (recursion step)
5. `q` - Stop recording

### Execution

`@a` starts the macro. Each iteration performs the
action and then calls `@a` again. The loop continues
until an error occurs (e.g., search fails, end of
file reached).

## Termination Conditions

### Error-Based Termination

The most common termination: a motion or search
command fails, producing an error that aborts
the macro chain. Examples:

| Cause | Typical Command |
|-------|-----------------|
| No more matches | `/pattern` fails |
| End of file | `j` at last line |
| No more finds | `f{char}` on empty line |
| Pattern not found | `:substitute` with no match |

### Count-Based

Instead of making the macro recursive, use a count:
`100@a` repeats the macro 100 times. This is safer
but requires knowing the iteration count.

### Search-In-Macro

A common pattern: include a search (`/pattern<CR>`)
in the macro. When no more matches exist, the search
fails and the macro stops.

## Safety Mechanisms

### Error Stops Execution

When any command in a macro produces an error, the
entire macro chain stops. This prevents runaway
recursion from corrupting the buffer.

### Undo Safety

Each complete macro invocation (including all
recursive calls) is grouped as a single undo unit.
A single `u` undoes ALL changes from the entire
recursive macro execution.

### Maximum Recursion Depth

The editor limits recursive macro depth to prevent
stack overflow. Default limit: 1000 iterations.
Exceeding the limit produces an error:
"recursive macro limit exceeded".

## Advanced Patterns

### Conditional Recursion

Use search patterns to create conditional logic:
1. Search for target pattern
2. If found, perform action and recurse
3. If not found, search fails and macro stops

### Multi-Register Chains

Macros in different registers can call each other:
- Register `a`: perform action, call `@b`
- Register `b`: move to next target, call `@a`

This alternating pattern can implement complex
multi-step batch operations.

### Pre-Clear Register

Before recording a recursive macro, clear the target
register: `:let @a = ""`. This prevents stale
content from the previous recording from interfering.

## Debugging

### Step-by-Step Testing

Test the macro body (without the recursive `@a` call)
first. Run it once manually. Then add the recursion.

### Checking Register Content

`:registers a` shows what is recorded in register a.
Verify the content includes the self-call at the end.

## Performance

### Screen Update Suppression

During recursive macro execution, the editor
suppresses screen updates. The display is refreshed
only after the entire macro chain completes. This
provides significant speedup for large batch edits.

## Related

- Macro recording: [docs/spec/editing/macros/README.md](docs/spec/editing/macros/README.md)
- Advanced macros: [docs/spec/editing/macros/macros-advanced.md](docs/spec/editing/macros/macros-advanced.md)
- Registers: [docs/spec/editing/registers/README.md](docs/spec/editing/registers/README.md)
