# Replace Mode

Replace mode overwrites existing text rather than inserting.

## Entry

| Key | From | Action |
|-----|------|--------|
| `R` | Normal | Enter Replace mode (multi-character) |
| `r` | Normal | Single character replace (stays in Normal) |
| `gr` | Normal | Virtual replace (respects virtual columns for tabs) |
| `gR` | Normal | Virtual Replace mode (continuous) |

## Behavior (normative)

| Aspect | Behavior |
|--------|----------|
| Typing | Overwrites character at cursor, advances cursor by one grapheme |
| Width-2 grapheme | Replacing a CJK character with an ASCII character fills the vacated cell with a space. Replacing an ASCII character with a CJK character overwrites the next character as well if needed. |
| Backspace | Restores the original character and moves cursor back. A stack of original characters MUST be maintained for the duration of the Replace session. |
| End of line | When cursor is at EOL, typed characters append (no overwrite) |
| Tab | Overwrites with tab character (or spaces if `expandtab` is set) |
| Newline (`Enter`) | Inserts newline, breaks line. Characters after cursor move to new line. |
| Count | `3R` enters Replace mode; on exit, the replacement is repeated 3 times. |

## Cursor display (normative)

In Replace mode, the cursor MUST be displayed as a full block (or underline if block is not available) to visually distinguish from Insert mode's thin bar.

## Original character stack (normative)

The Replace mode session maintains a stack of replaced characters:

1. Each typed character pushes the original character it overwrote onto the stack.
2. Backspace pops from the stack and restores that character.
3. The stack is discarded when Replace mode is exited.
4. If Backspace is pressed when the stack is empty (cursor at the start position of the Replace session), no action is taken.

## Exit

| Key | Action |
|-----|--------|
| `Esc` | Return to Normal mode |
| `Ctrl-[` | Return to Normal mode |
| `Ctrl-c` | Return to Normal mode |

## Undo behavior (normative)

- The entire Replace-mode session (from `R` to `Esc`) is one undo unit.
- Undo restores all original characters exactly.
- Backspace within the session is NOT recorded as a separate undo operation.

## Single-character replace (`r`) detail (normative)

1. Wait for the next character input.
2. Replace the grapheme under the cursor with the typed character.
3. If a count is given (`3ra`), replace the next 3 characters with `a`.
4. Cursor stays on the last replaced character (does not advance past it).
5. If the replacement character is `Enter`, split the line at the cursor.

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Undo: [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)
- Mode transitions: [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- CJK width rules: [/docs/technical/unicode.md](/docs/technical/unicode.md)
