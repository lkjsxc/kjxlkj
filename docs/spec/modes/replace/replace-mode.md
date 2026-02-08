# Replace Mode Detail

Back: [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)

Replace mode overwrites characters one at a time as the user types.

## Entry and exit

| Key | From | Action |
|---|---|---|
| `R` | Normal | Enter Replace mode at cursor position |
| `Esc` | Replace | Return to Normal mode |
| `r{char}` | Normal | Replace single character (does not enter Replace mode) |

## Overstrike behavior (normative)

In Replace mode, each typed character replaces the character at the cursor position:

1. The character under the cursor is replaced with the typed character.
2. The cursor advances one grapheme to the right.
3. If the cursor is at the end of the line, new characters are appended (like Insert mode).
4. Each replacement is a separate change in the undo record.

## Backspace behavior

Backspace in Replace mode MUST restore the original character:

| Condition | Behavior |
|---|---|
| Within replaced region | Move cursor left, restore original character |
| Before replaced region | Move cursor left (no restore; original was never changed) |
| At line start | No action (stay at position 0) |

The implementation MUST maintain a stack of replaced characters to support correct backspace restoration.

## Tab and wide character handling

| Scenario | Behavior |
|---|---|
| Replacing a tab with a regular character | Tab is removed; single character placed at cursor; subsequent characters shift |
| Replacing a regular character with a tab | Regular character removed; tab inserted at cursor; display width changes |
| Replacing width-1 with width-2 (CJK) | Width-1 character removed; width-2 character replaces it; subsequent characters shift |
| Replacing width-2 with width-1 | Width-2 character removed; width-1 character placed; padding cell consumed |

## Count with R

`{count}R{text}Esc` replaces text and then repeats the replacement `count - 1` additional times after the first replacement.

## Newline in replace mode

Pressing Enter in Replace mode inserts a newline at the cursor position (like Insert mode), breaking the current line.

## Virtual replace mode

`gR` enters Virtual Replace mode, which respects screen columns rather than character positions. See [/docs/spec/modes/replace/virtual-replace.md](/docs/spec/modes/replace/virtual-replace.md).

## Undo

When leaving Replace mode via Esc, all replacements made since entering Replace mode form a single undo group.

## Related

- Replace overview: [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)
- Overstrike: [/docs/spec/modes/replace/overstrike.md](/docs/spec/modes/replace/overstrike.md)
- Virtual replace: [/docs/spec/modes/replace/virtual-replace.md](/docs/spec/modes/replace/virtual-replace.md)
- Mode transitions: [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
