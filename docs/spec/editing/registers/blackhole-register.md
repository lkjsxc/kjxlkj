# Black Hole Register

Back: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

The black hole register (`"_`) discards text without affecting other registers.

## Overview

Deleting or changing text normally overwrites the unnamed register (`""`) and numbered registers. The black hole register provides a way to delete text without storing it anywhere.

## Usage

| Command | Effect |
|---|---|
| `"_dd` | Delete current line without storing it |
| `"_d{motion}` | Delete text covered by motion without storing |
| `"_c{motion}` | Change text without storing deleted text |
| `"_x` | Delete character without storing |

## Behavior

Text sent to the black hole register is permanently discarded. It does not appear in:

- The unnamed register (`""`)
- The numbered registers (`"0`-`"9`)
- The small delete register (`"-`)
- Any other register

## Use cases

| Scenario | Without `"_` | With `"_` |
|---|---|---|
| Delete a line then paste | The deleted text replaces the paste content | The paste content is preserved |
| Replace a word | `ciw` overwrites `""` | `"_ciw` preserves `""` |

The most common use case is replacing text: yank the replacement, then use `"_d` or `"_c` to remove the target text without overwriting the yank register, then paste.

## Implementation

When the black hole register is the target, the delete/change operation MUST skip all register writes. No `RegisterWrite` message is sent. The buffer mutation still occurs normally.

## Related

- Named registers: [/docs/spec/editing/registers/named-registers.md](/docs/spec/editing/registers/named-registers.md)
- Numbered registers: [/docs/spec/editing/registers/numbered-registers.md](/docs/spec/editing/registers/numbered-registers.md)
- Register commands: [/docs/spec/editing/registers/register-commands.md](/docs/spec/editing/registers/register-commands.md)
