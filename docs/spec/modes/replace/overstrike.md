# Overstrike Mode

Terminal-level overtype behavior and how kjxlkj handles overstruck files.

## Overview

Overstrike is a legacy terminal/printer mechanism where characters are combined by printing a character, backspace, then another character on the same position. Modern editors display the control sequences rather than rendering overstrike effects.

## Replace Mode vs Overstrike

| Feature | Replace Mode (`R`) | Overstrike |
|---|---|---|
| Scope | Editor command | Terminal/file format |
| Mechanism | Buffer character replacement | Backspace sequences in text |
| Display | Normal character rendering | Historically combined glyphs |
| Use case | Editing text | Legacy man pages, printer output |

## Handling Overstruck Files

When reading files containing backspace (0x08) sequences:

1. Display the raw control characters as `^H` in the buffer
2. Allow editing control characters directly
3. Provide a conversion command to strip overstrike sequences

### Conversion

`:set display+=uhex` shows the hex representation. Users can substitute overstrike sequences using `:%s/.\%x08//g` to strip them.

## Man Page Format

Traditional man pages use overstrike for formatting:

- **Bold**: `char + BS + char` (same character printed twice)
- **Underline**: `_ + BS + char` (underscore combined with character)

Modern systems use ANSI escape codes instead. kjxlkj reads man output processed through `col -b` or similar filters.

## Modern Alternative: ANSI Codes

kjxlkj supports ANSI SGR escape codes for terminal rendering (bold, underline, color). These are used in the built-in terminal emulator, not in normal buffers.

## Configuration

| Option | Effect |
|---|---|
| `display = "uhex"` | Show non-printable bytes as hex |
| `list = true` | Show whitespace and control chars |

## Related

- Replace mode: [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)
- Virtual replace: [/docs/spec/modes/replace/virtual-replace.md](/docs/spec/modes/replace/virtual-replace.md)
