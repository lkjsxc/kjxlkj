# Advanced Editing
Composite workflows built from deterministic primitives.

## Dot repeat (normative)

The `.` command repeats the last change. A "change" is defined as:

| Change type | What is recorded |
|---|---|
| Operator + motion | The operator, the motion, and the count |
| Operator + text-object | The operator and the text-object |
| Insert session | The enter-insert command (e.g., `i`, `a`, `o`) plus all typed text |
| Replace session | The `R` command plus all replacement characters |
| Single-char commands | `x`, `X`, `r{c}`, `~`, `J`, etc. |

### Dot repeat rules

1. `.` replays the last change at the current cursor position.
2. If `.` is given a count, the count overrides the original count.
3. If no count is given, the original count is used.
4. The register used in the original change is reused.
5. Dot repeat MUST be stable even if async services update the UI between the original change and the repeat.
6. Entering and exiting Insert mode without typing anything does NOT update the dot-repeat register.

## Macros (normative)

### Recording

| Key | Action |
|---|---|
| `q{reg}` | Start recording into register `{reg}` (a-z). Statusline shows `recording @{reg}`. |
| `q` | Stop recording (when already recording) |

### Playback

| Key | Action |
|---|---|
| `@{reg}` | Play macro in register `{reg}` |
| `@@` | Replay the last played macro |
| `{count}@{reg}` | Play macro `{count}` times |

### Macro recording detail

- Macro recording captures user intents (key sequences), not raw terminal bytes.
- The recorded key sequence is stored in the specified register as a string.
- `"qp` pastes the macro content for editing.
- `"qy` yanks text into register `q`, effectively creating a macro from buffer text.
- Recursive macros are allowed (a macro can invoke `@{reg}` including itself). A recursion depth limit of 1000 MUST be enforced to prevent infinite loops.

### Macro and dot repeat interaction

- Playing a macro counts as a single change for dot repeat. After `@a`, pressing `.` replays `@a`.
- However, the individual changes within a macro each update the `"` register. The final state of `"` after macro playback reflects the last change in the macro.

## Multiple cursors

Canonical specification: [/docs/spec/features/editing/multicursor.md](/docs/spec/features/editing/multicursor.md)

## Related

- Operators: [/docs/spec/editing/operators/operators.md](/docs/spec/editing/operators/operators.md)
- Registers: [/docs/spec/editing/registers/registers.md](/docs/spec/editing/registers/registers.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
