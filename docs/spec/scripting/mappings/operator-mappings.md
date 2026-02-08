# Operator Mappings

Back: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

User-defined operators that use operator-pending mode for motion/text-object input.

## Overview

Operator mappings define custom operators that follow the Vim operator grammar: the user presses the operator key, then a motion or text object to define the range. The operator function receives the range and performs its action.

## Defining operator mappings

| Command | Description |
|---|---|
| `:omap {lhs} {rhs}` | Create a mapping in operator-pending mode |
| `:ounmap {lhs}` | Remove an operator-pending mapping |

Operator-pending mappings define what happens between the operator key and the motion resolution.

## Custom operators

To define a full custom operator (not just an operator-pending mapping), the user defines a normal-mode mapping that enters operator-pending mode and then processes the resulting range.

| Step | Description |
|---|---|
| 1 | Map a key in normal mode to a function |
| 2 | The function sets `operatorfunc` to the callback |
| 3 | The function returns `g@` to enter operator-pending mode |
| 4 | After the user provides a motion, the callback receives the range |

## Operatorfunc

The `operatorfunc` option stores the name of the function to call when `g@{motion}` completes. The function receives the motion type (`char`, `line`, or `block`) and operates on the range `'[` to `']`.

## Motion types

| Type | Description |
|---|---|
| `char` | Characterwise motion: operate on characters from start to end |
| `line` | Linewise motion: operate on complete lines |
| `block` | Blockwise motion: operate on a rectangular block |

## Count handling

Counts entered before the operator are multiplied with counts entered before the motion. The operator function receives the final computed range.

## Visual mode operators

An operator mapping can also be triggered from Visual mode, in which case the range is the visual selection rather than a motion.

## Examples

| Operator | Description |
|---|---|
| Sort lines operator | Custom `gs` that sorts lines in the motion range |
| Comment operator | Custom `gc` that toggles comments on the range |
| Titlecase operator | Custom `gU` variant that titlecases the range |

## Related

- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)
- Operator-pending mode: [/docs/spec/editing/operators/operator-pending.md](/docs/spec/editing/operators/operator-pending.md)
- Mapping modes: [/docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)
