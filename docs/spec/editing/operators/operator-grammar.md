# Operator Grammar

Operator syntax and composition.

## Overview

Operators are commands that act on
text defined by a motion or object.

## Basic Grammar

### Structure

The general form of an operator command is:

`[count] [register] operator [count] motion/text-object`

| Component           | Required | Description                                        |
|---------------------|----------|----------------------------------------------------|
| `[count]`           | No       | Repeat count (default 1); multiplies with operator |
| `[register]`        | No       | `"x` register specification for storage            |
| `operator`          | Yes      | The editing command (d, c, y, >, <, =, gq, etc.)  |
| `motion/text-object`| Yes      | Defines the text range the operator acts on        |

### Examples

| Input   | Effect                      |
|---------|-----------------------------|
| `d2w`   | Delete 2 words              |
| `3dw`   | Delete word 3 times         |
| `"ayy`  | Yank line to register a     |
| `gqap`  | Format paragraph            |
| `>i{`   | Indent inside braces        |
| `c$`    | Change to end of line       |

## Core Operators

### Delete

`d{motion}` deletes text defined by motion. Deleted text is stored in the
default register (`"`) unless a register is explicitly specified. `dd` deletes
the current line (linewise). `D` is equivalent to `d$` (delete to end of line).

### Change

`c{motion}` deletes text defined by motion and enters Insert mode. `cc` changes
the entire current line. `C` is equivalent to `c$`. `s` is equivalent to `cl`
(substitute character). `S` is equivalent to `cc`.

### Yank

`y{motion}` copies text to a register without deleting it. `yy` or `Y` yanks
the current line (linewise). The cursor does NOT move after yank; it remains at
the start of the yanked range.

### Put

`p` puts register text after the cursor (characterwise) or below the current
line (linewise). `P` puts text before the cursor or above the current line.
Count repeats the put operation. If the register contains linewise text, put
creates new lines; if characterwise, text is inserted inline.

## Case Operators

### Change Case

- `~` toggles the case of the character under the cursor and advances.
- `g~{motion}` toggles case over the range defined by motion.
- `gu{motion}` lowercases text over the range defined by motion.
- `gU{motion}` uppercases text over the range defined by motion.

### Examples

| Input   | Effect                  |
|---------|-------------------------|
| `g~iw`  | Toggle case of word     |
| `gUap`  | Uppercase paragraph     |
| `guaw`  | Lowercase word          |
| `~`     | Toggle char under cursor|

## Formatting Operators

### Format

- `gq{motion}` formats text to textwidth; cursor moves to end of formatted region.
- `gw{motion}` formats text to textwidth; cursor stays at original position.
- `=` reindents text according to the configured indent rules.

### Examples

| Input   | Effect                              |
|---------|-------------------------------------|
| `gqap`  | Format paragraph                    |
| `gqq`   | Format current line                 |
| `gwip`  | Format inner paragraph, keep cursor |
| `=i{`   | Reindent inside braces              |

## Shift Operators

### Indent

- `>{motion}` shifts lines right by shiftwidth.
- `<{motion}` shifts lines left by shiftwidth.
- `>>` indents the current line; `<<` dedents the current line.

### Examples

| Input   | Effect                  |
|---------|-------------------------|
| `>ap`   | Indent paragraph        |
| `<3j`   | Dedent 3 lines down     |
| `>>`    | Indent current line     |
| `>i{`   | Indent inside braces    |

## Filter Operators

### External Filter

`!{motion}` filters text through an external command. `!!` filters the current
line. The command receives the text range on stdin and the operator replaces the
original text with the command's stdout.

### Examples

| Input       | Effect                          |
|-------------|---------------------------------|
| `!ipsort`   | Sort paragraph lines            |
| `!!date`    | Replace line with date output   |
| `3!!fmt`    | Format 3 lines with fmt         |

## Operator Behavior

### Linewise

Operators that act on entire lines: `dd`, `yy`, `cc`, `>>`, `<<`, `==`, `gqq`.
When the motion is linewise (`j`, `k`, `G`, `gg`), the operator acts on whole
lines. Linewise text stored in registers is put as new lines by `p`/`P`.

### Characterwise

Operators with character precision: `dw`, `cw`, `yl`. When the motion is
characterwise (`w`, `e`, `b`, `f`, `t`, `h`, `l`), the operator acts on the
exact character range. Inclusive motions (e.g., `e`) include the last character;
exclusive motions (e.g., `w`) exclude it.

## Count Multiplication

### Before Operator

`3dw` applies count 3 to `dw`, deleting 3 words.

### After Operator

`d3w` applies count 3 to motion `w`, deleting 3 words (same result).

### Combined

`2d3w` multiplies the counts: 2 x 3 = 6 words deleted.

## Register Specification

### Syntax

`"{register}` is placed before the operator to direct storage: `"ayw` yanks
a word to register `a`.

### Examples

| Input   | Effect                            |
|---------|-----------------------------------|
| `"ayy`  | Yank line to register a           |
| `"ap`   | Put from register a               |
| `"+y`   | Yank to system clipboard          |
| `"_d`   | Delete to black hole (no storage) |
