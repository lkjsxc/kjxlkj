# Read-Only Registers

Registers that can only be read, not written directly.

## List of Read-Only Registers

| Register | Name | Content |
|----------|------|---------|
| `".` | Last insert | Last inserted text |
| `"%` | Filename | Current buffer filename |
| `"#` | Alternate | Alternate buffer filename |
| `":` | Last command | Last ex command |
| `"/` | Last search | Last search pattern |

## Last Insert Register (.)

Contains text from the most recent insert mode session.

### Content


### Usage


### Limitations

- Read-only: `:let @. = "x"` has no effect
- Updated only on insert mode exit
- Single insert session only (not cumulative)

## Filename Register (%)

Contains the current buffer's filename.

### Content Examples


### Usage


### Path Modifiers

Use `expand()` for path manipulation:

| Expression | Result |
|------------|--------|
| `@%` | Relative filename |
| `expand("%:p")` | Full path |
| `expand("%:h")` | Directory |
| `expand("%:t")` | Filename only |
| `expand("%:r")` | Without extension |
| `expand("%:e")` | Extension only |

## Alternate File Register (#)

Contains the alternate (previous) buffer filename.

### Usage


### When Updated

- Changes when switching buffers
- `:b#` uses this register
- `Ctrl-^` toggles between current and alternate

## Last Command Register (:)

Contains the most recently executed ex command.

### Content


### Usage


### Note

- Does not include the `:` prefix
- Updated after each command execution
- Useful for complex command repetition

## Last Search Register (/)

Contains the last search pattern.

### Content


### Usage


### Setting (Special Case)

Unlike other read-only registers, `@/` can be set:


## Reading in Different Contexts

### Normal Mode


### Insert Mode


### Command Line


## Why Read-Only?

These registers reflect editor state:
- Content determined by editor actions
- Writing would break expected behavior
- Some have special write semantics (`/`)

## Configuration


## API Reference

