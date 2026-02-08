# Mode Configuration

Back: [/docs/spec/modes/README.md](/docs/spec/modes/README.md)

Mode configuration affects presentation, not editing semantics.

## Requirements

- Configuration MUST NOT change editing semantics.
- Cursor style and line number settings are UI concerns only.
- Mode transitions remain deterministic regardless of configuration.

## Configurable elements (normative)

### Cursor shape per mode

| Mode | Default shape | DECSCUSR code | Description |
|---|---|---|---|
| Normal | Block (blinking) | `\e[1 q` | Full cell block, blinks |
| Insert | Bar (blinking) | `\e[5 q` | Thin vertical bar, blinks |
| Visual | Block (steady) | `\e[2 q` | Full cell block, steady |
| Replace | Underline (blinking) | `\e[3 q` | Underline cursor, blinks |
| Command | Bar (blinking) | `\e[5 q` | Same as Insert |
| OperatorPending | Half-block (steady) | `\e[2 q` | Block, steady (to indicate pending) |

If the terminal does not support `DECSCUSR`, the cursor shape request MUST be silently ignored.

### Cursor blink

| Setting | Default | Description |
|---|---|---|
| `guicursor` | Mode-specific | Controls cursor shape and blink per mode |

### Line number display

| Setting | Default | Description |
|---|---|---|
| `number` | `true` | Show absolute line numbers |
| `relativenumber` | `false` | Show relative line numbers |
| Both `true` | N/A | Hybrid: absolute for current line, relative for others |
| Both `false` | N/A | No line numbers displayed |

### Mode indicator

| Setting | Default | Description |
|---|---|---|
| `showmode` | `true` | Show mode name in command line area |

Mode indicator text:

| Mode | Display text |
|---|---|
| Normal | (empty; no indicator shown) |
| Insert | `-- INSERT --` |
| Visual (char) | `-- VISUAL --` |
| Visual (line) | `-- VISUAL LINE --` |
| Visual (block) | `-- VISUAL BLOCK --` |
| Replace | `-- REPLACE --` |
| Command | (empty; command line is shown instead) |
| OperatorPending | (empty; behaves like Normal) |
| TerminalInsert | `-- TERMINAL --` |
| TerminalNormal | `-- TERMINAL --` |

## Invariants (normative)

1. Keybindings execute identically regardless of cursor style configuration.
2. Line number format does not affect navigation commands (`:5` goes to line 5 regardless of display format).
3. Mode indicator is always visible in the statusline when `showmode` is true.
4. Cursor shape changes MUST be emitted on every mode transition.
5. On editor exit, cursor shape MUST be restored to the terminal's default (`\e[0 q`).

## Related

- UX theming: [/docs/spec/ux/theming.md](/docs/spec/ux/theming.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Mode transitions: [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
