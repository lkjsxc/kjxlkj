# Overstrike Mode

Terminal-level overtype behavior.

## Overview

Overstrike is a terminal/display
concept where characters overlay
existing characters.

## Historical Context

### Origin

Older terminals and printers
used overstrike for:
- Bold (print char twice)
- Underline (char + underscore)
- Strikethrough

### Modern Relevance

Less common today, but still
relevant for some use cases.

## Replace vs Overstrike

### Replace Mode


Replaces character in buffer.

### Overstrike

Terminal overwrites display
position, may combine glyphs.

## Terminal Behavior

### Overstrike Sequence


### Modern Terminals

Most don't support true overstrike.
Use ANSI escape codes instead.

## Simulation

### In Editor

Editor doesn't have true overstrike,
but replace mode is similar.

### Configuration


## Use Cases

### Fixed-Width Output

Generating output for legacy
printers or displays.

### Text-Based Formatting

Creating "bold" or "underline"
in plain text via overstriking.

## Implementation

### How It Works

1. Print character
2. Backspace
3. Print same/different char

### Example Output


## In kjxlkj

### Handling Overstruck Files

When reading files with
backspace sequences:


### Conversion


Shows control characters.

## Related Concepts

### Replace Mode

Use `R` for overtype editing.
Most similar behavior.

### Unicode Combining

Modern combining characters:

More compatible than overstrike.

## Man Page Format

### Traditional man

Uses overstrike for formatting:

### Reading man Files


May help visualize.

## Terminal Settings

### ANSI Codes Preferred

Modern terminals use:

### Configuration


## Compatibility

### Terminals Supporting

Few modern terminals:
- Some TTY emulators
- Historical terminal emulators

### Terminals Not Supporting

Most modern:
- xterm (modern mode)
- iTerm2
- Windows Terminal
- Alacritty

## Editor Behavior

### Display


### Editing

Control characters shown,
can be edited directly.

