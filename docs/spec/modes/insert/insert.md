# Insert mode
Insert mode handles direct text entry and completion UI integration.

## Requirements
- Inserted text becomes core transactions (coalesced into one undo unit per session).
- Completion suggestions come from services (LSP/snippets/etc.) and are versioned.
- Leaving insert with `Esc` always returns to Normal.

## Entry points (normative)

| Key | Action | Cursor position |
|---|---|---|
| `i` | Insert before cursor | Before current grapheme |
| `I` | Insert at first non-blank | Column of first non-blank character |
| `a` | Append after cursor | After current grapheme |
| `A` | Append at end of line | After last character on line |
| `o` | Open line below | New line below, enter Insert |
| `O` | Open line above | New line above, enter Insert |
| `gi` | Insert at last insert position | Position stored in `'^` mark |
| `gI` | Insert at column 1 | Column 0 of current line |
| `c{motion}` | Change | After deleting the motion range |
| `s` | Substitute character | After deleting character under cursor |
| `S` / `cc` | Substitute line | After deleting current line content |
| `C` | Change to end of line | After deleting to EOL |

## In-mode keys (normative)

| Key | Action |
|---|---|
| Printable char | Insert at cursor position |
| `Backspace` / `Ctrl-h` | Delete character before cursor |
| `Delete` | Delete character after cursor |
| `Ctrl-w` | Delete word before cursor |
| `Ctrl-u` | Delete to start of current insert (or start of line) |
| `Ctrl-t` | Increase indent by `shiftwidth` |
| `Ctrl-d` | Decrease indent by `shiftwidth` |
| `Ctrl-j` / `Enter` | Insert newline |
| `Ctrl-r {reg}` | Insert contents of register `{reg}` |
| `Ctrl-a` | Re-insert last insert text |
| `Ctrl-o` | Execute one Normal-mode command then return to Insert |
| `Ctrl-n` | Next completion item |
| `Ctrl-p` | Previous completion item |
| `Ctrl-v` | Insert literal character (next key is inserted verbatim) |
| `Ctrl-k {c1}{c2}` | Insert digraph |
| `Tab` | Insert tab or trigger indentation (configurable) |

## Exit (normative)

| Key | Action |
|---|---|
| `Esc` | Return to Normal mode |
| `Ctrl-[` | Return to Normal mode (equivalent to Esc) |
| `Ctrl-c` | Return to Normal mode (no abbreviation expansion) |

## Undo granularity (normative)

- All text entered in a single Insert-mode session is one undo unit.
- `Ctrl-u` and `Ctrl-w` within Insert mode create sub-boundaries; the delete itself is part of the current undo unit but can be separately undone if followed by more typing.
- `Ctrl-o {command}` creates a separate undo unit for the Normal-mode command.

## Cursor display (normative)

In Insert mode, the cursor MUST be displayed as a thin vertical bar (if the terminal supports `DECSCUSR`), positioned between characters. For terminals that do not support cursor shape changes, render the cursor as a block on the character where text will be inserted.

## Completion (normative)

See [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md) for the completion popup specification.

## Related

- Undo granularity: [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)
- LSP completion: [/docs/spec/features/lsp/lsp.md](/docs/spec/features/lsp/lsp.md)
- Mode transitions: [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- Registers: [/docs/spec/editing/registers/registers.md](/docs/spec/editing/registers/registers.md)
