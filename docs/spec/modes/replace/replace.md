# Replace Mode

Replace mode overwrites existing text rather than inserting.

## Entry

| Key | Action |
|-----|--------|
| `R` | Enter Replace mode |
| `r` | Single character replace (stays in Normal) |

## Behavior

| Aspect | Behavior |
|--------|----------|
| Typing | Overwrites character at cursor, advances cursor |
| Backspace | Restores original character, moves cursor back |
| End of line | Appends characters (no overwrite) |
| Tab | Overwrites with tab character |
| Newline | Inserts newline, breaks line |

## Exit

| Key | Action |
|-----|--------|
| `Esc` | Return to Normal mode |
| `Ctrl-c` | Return to Normal mode |

## Undo Behavior

- Multiple replacements coalesced into single undo unit
- Undo restores original text exactly
- Backspace within session not recorded as separate operation

## Related

- Cursor semantics: [docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Undo: [docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)
- Mode transitions: [docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
