# Literal Insert (Input Context)

Insert characters by numeric code or literally.

## Overview

This covers the input mechanics of literal character
insertion via `<C-v>` sequences. For the full literal
insertion specification including digraphs, see the
main insert-literal document.

## Input Processing

### Bypass Mappings

`<C-v>` bypasses all key mappings for the next keypress.
The raw key event is captured and inserted as-is.

### Bypass Abbreviations

`<C-v>` prevents abbreviation expansion for the
following character.

## Numeric Input States

### State Machine

1. Idle: waiting for `<C-v>`
2. Prefix: reading mode prefix (none, `o`, `x`, `u`, `U`)
3. Digits: accumulating digit characters
4. Complete: character inserted, return to idle

### Digit Limits

| Mode | Max Digits | Value Range |
|------|-----------|-------------|
| Decimal | 3 | 0-255 |
| Octal (`o`) | 3 | 0-377 (0-255) |
| Hex (`x`) | 2 | 0x00-0xFF |
| Unicode (`u`) | 4 | U+0000-U+FFFF |
| Full Unicode (`U`) | 8 | U+00000000-U+0010FFFF |

### Early Completion

If the maximum digits are reached, insertion happens
immediately. If fewer digits are typed followed by a
non-digit, the accumulated value is used.

## Display During Input

### Status Feedback

While entering digits, the command area shows:
- `^` initially
- `^o177` (example) as digits are typed

### Cursor Behavior

The cursor remains at the insertion point. A special
indicator shows the pending literal state.

## Error Handling

### Invalid Code Point

Values above U+10FFFF are rejected. Surrogate code
points (U+D800-U+DFFF) are rejected.

### Overflow

If the decimal value exceeds 255, only the last 3
digits are used (wraps).

## Special Cases

### Null Character

`<C-v>000` inserts a null byte (0x00). This is valid
in the buffer but may cause issues with some operations.

### Tab in expandtab Mode

`<C-v><Tab>` inserts a literal tab character even when
`expandtab` is enabled. Normal `<Tab>` would insert spaces.
