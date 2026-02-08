# Keybinding Hints (Which-Key)

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Display available keybindings after a prefix key.

## Overview

When a prefix key (like `<leader>` or `g`) is pressed and no immediate match exists, a floating panel appears showing available continuations with descriptions.

## Behavior

1. User presses a prefix key (e.g., `<leader>`).
2. After a short delay (`timeoutlen`), a panel appears.
3. The panel lists all bindings starting with that prefix.
4. User presses the next key to complete the binding.

## Display

| Column | Content |
|---|---|
| Key | The next key in the sequence |
| Description | The `desc` field from the binding definition |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `whichkey.enabled` | `true` | Enable which-key popup |
| `whichkey.delay` | `300` | Delay in ms before showing |
| `whichkey.max_width` | `40` | Maximum popup width |

## Groups

Bindings can be organized into named groups for the which-key display:

| Prefix | Group Name |
|---|---|
| `<leader>f` | "Find" |
| `<leader>g` | "Git" |
| `<leader>l` | "LSP" |
| `<leader>b` | "Buffer" |

## Related

- Key mappings: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)
- Keybinding DSL: [/docs/spec/ux/keybinding-dsl.md](/docs/spec/ux/keybinding-dsl.md)
