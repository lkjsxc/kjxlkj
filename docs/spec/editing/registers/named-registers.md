# Named Registers

Alphabetic registers (a-z, A-Z) for storing and retrieving text.

## Overview

| Register | Access | Behavior |
|----------|--------|----------|
| `a`-`z` | Read/Write | Overwrite content |
| `A`-`Z` | Write | Append to lowercase |

## Using Named Registers

### Yanking to Register


### Deleting to Register


### Pasting from Register


## Append Mode (Uppercase)

Using uppercase appends instead of overwrites:


## Insert Mode Access

| Key | Action |
|-----|--------|
| `Ctrl-R a` | Insert register a contents |
| `Ctrl-R Ctrl-R a` | Insert literally (no remapping) |
| `Ctrl-R Ctrl-O a` | Insert without indent |
| `Ctrl-R Ctrl-P a` | Insert with indent fixed |

## Command Line Access


## Visual Mode


## Viewing Register Contents


## Register Persistence


## Common Workflows

### Multi-paste


### Collecting Text


### Replace Without Losing


## Linewise vs Characterwise

Registers remember how text was yanked:


## Expression Register Combination


## API Reference


## Configuration


## See Also

- [numbered-registers.md](numbered-registers.md) - Registers 0-9
- [special-registers.md](special-registers.md) - Special registers
- [register-commands.md](register-commands.md) - Register commands
