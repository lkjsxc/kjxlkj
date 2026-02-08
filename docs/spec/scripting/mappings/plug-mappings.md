# Plug Mappings

Back: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Internal mapping names used for extensibility.

## Overview

`<Plug>` mappings are virtual key sequences that serve as named interfaces for functionality. They connect user keybindings to internal operations without exposing implementation details.

## How It Works

A `<Plug>` mapping is defined internally:

`nnoremap <Plug>(CommentToggle) ...internal logic...`

Users then map a key to the plug name:

`nmap gc <Plug>(CommentToggle)`

## Built-in Plug Mappings

| Plug | Default Key | Description |
|---|---|---|
| `<Plug>(CommentToggle)` | `gc` | Toggle comment |
| `<Plug>(SurroundAdd)` | `ys` | Add surround |
| `<Plug>(SurroundDelete)` | `ds` | Delete surround |
| `<Plug>(SurroundChange)` | `cs` | Change surround |
| `<Plug>(JumpForward)` | `<C-i>` | Jump list forward |
| `<Plug>(JumpBackward)` | `<C-o>` | Jump list backward |

## Naming Convention

Plug mapping names use PascalCase in parentheses: `<Plug>(ActionName)`.

## Remapping

Users can remap plug mappings to any key without affecting internal logic.

## Related

- Key mappings: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)
- Mapping modes: [/docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)
