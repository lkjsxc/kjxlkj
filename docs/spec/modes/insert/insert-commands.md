# Insert Mode Commands

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Editing commands available while in Insert mode. These commands do not leave Insert mode unless noted.

## Deletion commands (normative)

| Key | Action | Detail |
|---|---|---|
| `Backspace` / `Ctrl-h` | Delete character before cursor | Stops at line start (does not join lines unless `backspace` option includes `eol`) |
| `Delete` | Delete character under cursor | No-op at end of line |
| `Ctrl-w` | Delete word before cursor | Deletes backward to the start of the previous word (space-delimited) |
| `Ctrl-u` | Delete to start of line | Deletes from cursor to the first non-blank, or to column 0 if already at first non-blank |

## Cursor movement

| Key | Action |
|---|---|
| `Left` / `Ctrl-b` | Move cursor left one grapheme |
| `Right` / `Ctrl-f` | Move cursor right one grapheme |
| `Up` | Move cursor up one line (preserving desired column) |
| `Down` | Move cursor down one line (preserving desired column) |
| `Home` | Move cursor to column 0 |
| `End` | Move cursor past last character on line |

Arrow-key movement in Insert mode does NOT break the undo group. The undo group is broken only by `Esc` or after the `undobreak_pause` timeout (default 3 seconds).

## Special insertions

| Key | Action | Detail |
|---|---|---|
| `Enter` / `Ctrl-m` | Insert newline | Apply auto-indent based on the current `indentexpr` or `autoindent` setting |
| `Tab` | Insert tab or spaces | If `expandtab` is set, insert `shiftwidth` spaces. Otherwise insert literal `\t`. If completion popup is visible, accept selected item. |
| `Ctrl-t` | Increase indent | Add one `shiftwidth` of indentation at the start of the current line |
| `Ctrl-d` | Decrease indent | Remove one `shiftwidth` of indentation from the start of the current line |
| `Ctrl-j` | Insert newline (same as Enter) | Alias for `Enter` |

## Register access

| Key | Action | Detail |
|---|---|---|
| `Ctrl-r {reg}` | Insert register contents | Paste the contents of register `{reg}` at the cursor. Text is inserted as if typed (triggers abbreviations and mappings). |
| `Ctrl-r Ctrl-r {reg}` | Insert register literally | Paste register contents without triggering mappings or abbreviations. |
| `Ctrl-r =` | Expression register | Open a mini command-line to evaluate an expression. The result is inserted at the cursor. |
| `Ctrl-r Ctrl-o {reg}` | Insert register without auto-indent | Insert register contents literally, suppressing auto-indentation. |

## Insert-normal mode

| Key | Action |
|---|---|
| `Ctrl-o` | Execute one Normal-mode command, then return to Insert |

The single Normal-mode command sees the full Normal-mode keybinding table. After execution, mode returns to Insert. If the command changes mode (e.g., `Ctrl-o v` enters Visual), Insert mode is NOT restored until the user explicitly re-enters it.

## Exit commands

| Key | Action | Cursor adjustment |
|---|---|---|
| `Esc` | Return to Normal mode | Cursor moves left one grapheme (clamp to column 0) |
| `Ctrl-c` | Return to Normal mode | Same as `Esc` but does NOT trigger `InsertLeave` autocommand |
| `Ctrl-[` | Return to Normal mode | Alias for `Esc` |

## Undo behavior

All text typed in a single Insert-mode session (from entry to `Esc`) is one undo group. The undo group is broken by:

| Event | Effect |
|---|---|
| Cursor movement via arrow keys | Does NOT break undo group |
| `Ctrl-o` command | Breaks undo group before and after the command |
| Pause > `undobreak_pause` (3 seconds) | Breaks undo group at the pause point |

## Related

- Insert mode overview: [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)
- Insert navigation: [/docs/spec/modes/insert/insert-navigation.md](/docs/spec/modes/insert/insert-navigation.md)
- Insert mode entry: [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)

