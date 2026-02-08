# Mode Transitions

Back: [/docs/spec/modes/README.md](/docs/spec/modes/README.md)

Mode transitions are deterministic and never block on services.

## Mode enum (normative)

The mode state machine uses this enum:

| Mode | Variant | Description |
|---|---|---|
| Normal | `Normal` | Navigation, operator composition, command entry. |
| Insert | `Insert` | Text entry with completion and auto-pairs. |
| Replace | `Replace` | Overwrite character by character. |
| Visual | `Visual(VisualKind)` | Selection: `Char`, `Line`, or `Block`. |
| Command | `Command(CommandKind)` | Ex command (`:`) or search (`/`, `?`). |
| OperatorPending | `OperatorPending(Operator)` | Waiting for motion/text-object after operator key. |
| TerminalInsert | `TerminalInsert` | Forwarding input to PTY (terminal window focused). |
| InsertNormal | `InsertNormal` | Single normal-mode command from insert mode (`Ctrl-O`). |

## Core rules

| Rule | Description |
|---|---|
| Escape returns to Normal | `Esc` from any mode returns to Normal. |
| Double escape cancels | `Esc Esc` resets all pending state. |
| Entry from Normal only | Mode entry keys work only in Normal mode. |
| Deterministic | Same (mode, input) pair always produces the same transition. |
| Non-blocking | No transition may block on external IO. |

## Full transition table

| From | To | Trigger | Side effects |
|---|---|---|---|
| Normal | Insert | `i` | Cursor stays. |
| Normal | Insert | `a` | Cursor moves right one grapheme. |
| Normal | Insert | `I` | Cursor moves to first non-blank. |
| Normal | Insert | `A` | Cursor moves past last grapheme (end-inclusive). |
| Normal | Insert | `o` | Insert new line below, cursor to new line. |
| Normal | Insert | `O` | Insert new line above, cursor to new line. |
| Normal | Visual(Char) | `v` | Anchor set at cursor. |
| Normal | Visual(Line) | `V` | Anchor set at cursor line. |
| Normal | Visual(Block) | `Ctrl-v` | Block anchor set at cursor. |
| Normal | Command(Ex) | `:` | Open command line. |
| Normal | Command(Search) | `/` | Open forward search. |
| Normal | Command(Search) | `?` | Open backward search. |
| Normal | Replace | `R` | Enter replace mode. |
| Normal | OperatorPending(d) | `d` | Await motion/text-object for delete. |
| Normal | OperatorPending(c) | `c` | Await motion for change. |
| Normal | OperatorPending(y) | `y` | Await motion for yank. |
| Normal | OperatorPending(>) | `>` | Await motion for indent. |
| Normal | OperatorPending(<) | `<` | Await motion for dedent. |
| Normal | OperatorPending(=) | `=` | Await motion for reindent. |
| Normal | OperatorPending(gq) | `gq` | Await motion for format. |
| Normal | TerminalInsert | `i` (in terminal window) | Start forwarding keys to PTY. |
| Insert | Normal | `Esc` | Commit undo checkpoint, adjust cursor left. |
| Insert | InsertNormal | `Ctrl-O` | Execute one normal command, then return. |
| InsertNormal | Insert | After single command | Resume insert mode. |
| Visual(*) | Normal | `Esc` | Clear selection. |
| Visual(*) | Normal | Operator applied | Execute operator on selection, return to Normal. |
| Visual(Char) | Visual(Line) | `V` | Switch visual sub-mode. |
| Visual(Char) | Visual(Block) | `Ctrl-v` | Switch visual sub-mode. |
| Visual(Line) | Visual(Char) | `v` | Switch visual sub-mode. |
| Visual(Line) | Visual(Block) | `Ctrl-v` | Switch visual sub-mode. |
| Visual(Block) | Visual(Char) | `v` | Switch visual sub-mode. |
| Visual(Block) | Visual(Line) | `V` | Switch visual sub-mode. |
| Command(*) | Normal | `Esc` | Close command line. |
| Command(Ex) | Normal | `Enter` | Execute command, close command line. |
| Command(Search) | Normal | `Enter` | Execute search, close command line. |
| Replace | Normal | `Esc` | End replace mode. |
| OperatorPending(*) | Normal | `Esc` | Cancel pending operator. |
| OperatorPending(*) | Normal | Motion received | Execute operator + motion, return to Normal. |
| OperatorPending(c) | Insert | Motion received | Delete range, enter insert. |
| TerminalInsert | Normal | `Ctrl-\` `Ctrl-n` | Exit terminal input. |

## Double-operator (linewise)

When an operator key is pressed twice (e.g., `dd`, `yy`, `cc`), the operator applies to the current line (linewise):

| Keys | Effect |
|---|---|
| `dd` | Delete current line. |
| `yy` | Yank current line. |
| `cc` | Change current line (delete + enter Insert). |
| `>>` | Indent current line. |
| `<<` | Dedent current line. |
| `==` | Reindent current line. |
| `gqq` | Format current line. |

## Undo boundaries

| Transition | Undo effect |
|---|---|
| Normal -> Insert | Begin undo group. |
| Insert -> Normal | End undo group (one `u` undoes entire insert session). |
| Insert (after pause > 3s) | Break undo group, start new one. |

## Invariants

1. No transition MAY block on external IO.
2. Transitions MUST be deterministic based on (current_mode, input_key).
3. Pending operator state clears on mode change to Normal.
4. Jumplist updated on certain transitions (search, `:` commands that move cursor).
5. Status line MUST reflect current mode name immediately after transition.

## Related

- Mode overview: [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Replace mode: [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)
- Normal mode: [/docs/spec/modes/normal.md](/docs/spec/modes/normal.md)
- Visual mode: [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
- Terminal mode: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)
