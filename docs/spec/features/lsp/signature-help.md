# Signature Help

Function signature display while typing.

## Trigger (normative)

Signature help appears automatically when typing trigger characters inside a function call:

| Trigger | Context |
|---|---|
| `(` | Function/method call opened |
| `,` | Moving to next argument |
| `<` | Generic type arguments (language-dependent) |

Manual trigger: `<C-s>` or `:SignatureHelp`.

## Display

The signature popup shows above or below the cursor:

- Function name and full parameter list
- Current parameter highlighted (bold or different color)
- Parameter documentation (if available from LSP)

### Multiple Overloads

When a function has multiple signatures (overloads), the popup shows `1/N` and allows cycling:

| Key | Action |
|---|---|
| `<C-j>` | Next overload |
| `<C-k>` | Previous overload |
| `<Esc>` | Dismiss |

## Parameter Highlighting

The parameter corresponding to the cursor's argument position is highlighted. As you type `,` to advance to the next argument, the highlight moves.

## Auto-Close

The popup closes when:

- Cursor moves outside the function call parentheses
- Closing `)` is typed after all parameters
- `<Esc>` is pressed
- Switching to normal mode

## Configuration

| Option | Default | Description |
|---|---|---|
| `signature_help` | `true` | Enable automatic signature display |
| `signature_help_trigger` | `true` | Auto-trigger on `(` and `,` |

## Position

The popup position adapts based on available space. If there is not enough room above the cursor, it appears below. Never overlaps the cursor line.

## Related

- Completion: [/docs/spec/features/lsp/completion.md](/docs/spec/features/lsp/completion.md)
- Hover: [/docs/spec/features/lsp/hover.md](/docs/spec/features/lsp/hover.md)
