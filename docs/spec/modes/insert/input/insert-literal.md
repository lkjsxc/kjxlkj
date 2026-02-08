# Insert Literal (Input Layer)

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

Low-level input processing for literal character insertion.

## Overview

This documents the input layer behavior when `<C-v>` is pressed in insert mode, including numeric code states and display.

## Input Processing

After `<C-v>`:

1. If next input is a digit, enter numeric code input mode.
2. If next input is `o`, `x`, `u`, or `U`, enter the appropriate radix mode.
3. Otherwise, insert the literal byte of the next key.

## Numeric Input States

| State | Trigger | Format | Max digits |
|---|---|---|---|
| Decimal | `0`-`9` | `{0-255}` | 3 digits |
| Octal | `o` | `o{0-377}` | 3 digits |
| Hex byte | `x` | `x{00-FF}` | 2 digits |
| Unicode 4-digit | `u` | `u{0000-FFFF}` | 4 digits |
| Unicode 8-digit | `U` | `U{00000000-7FFFFFFF}` | 8 digits |

## Display During Input

While entering numeric code, the status area shows the partial input (e.g., `^V123`).

## Error Handling

If the numeric value exceeds the valid range, the character is not inserted and a bell is emitted.

## Related

- Insert literal overview: [/docs/spec/modes/insert/insert-literal.md](/docs/spec/modes/insert/insert-literal.md)
- Insert special chars: [/docs/spec/modes/insert/input/insert-special-chars.md](/docs/spec/modes/insert/input/insert-special-chars.md)
