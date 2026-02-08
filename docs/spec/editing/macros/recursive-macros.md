# Recursive Macros

Back: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)

A recursive macro is one that invokes itself during playback, creating a loop.

## Mechanism (normative)

A macro in register `a` is recursive if its recorded sequence ends with `@a`. When played back, it repeats until a motion or command fails (e.g., `j` fails at the last line).

## Stop conditions (normative)

Recursive playback terminates when:

| Condition | Example |
|---|---|
| Motion fails | `j` at the last line of the buffer |
| Search fails | `n` when no more matches exist |
| Command errors | `:s/pat/rep/` when `pat` is not found |
| User interrupts | `Ctrl-C` during macro playback |
| Maximum recursion depth | Default limit: 1000 iterations (prevents runaway) |

## Example: process every line

To number every line:

1. Move to first line: `gg`
2. Start recording: `qa`
3. Perform the action: `I1. <Esc>`
4. Move to next line: `j`
5. Call self: `@a`
6. Stop recording: `q`
7. Play: `@a`

The macro stops when `j` fails at the last line.

## Counted vs recursive

Using a count (`100@a`) is simpler and safer than recursion when the iteration count is known. Recursion is preferred when the count depends on buffer content.

## Error handling

On error during recursive playback, the macro stops immediately. The buffer state reflects all changes made before the error. Undo reverts the entire playback sequence (one `u` undoes all iterations).

## Related

- Macros: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)
- Register macros: [/docs/spec/editing/macros/register-macros.md](/docs/spec/editing/macros/register-macros.md)
- Advanced macros: [/docs/spec/editing/macros/macros-advanced.md](/docs/spec/editing/macros/macros-advanced.md)

