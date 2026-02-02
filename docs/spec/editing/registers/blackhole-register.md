# Black Hole Register

The `_` register that discards all content written to it.

## Purpose

Delete or change text without affecting any registers:
- Preserves unnamed register ("")
- Preserves numbered registers (1-9)
- Preserves yank register (0)

## Basic Usage

### Delete Without Saving


### Change Without Saving


## Common Use Cases

### Replace Without Losing Yank


Without black hole:

### Clean Delete for Macros


### Bulk Replace


## Behavior Details

### Reading From Black Hole

Reading `_` always returns empty string:


### In Insert Mode


### In Expressions


## Comparison Table

| Operation | Without `_` | With `_` |
|-----------|-------------|----------|
| `dd` | Saves to "", 1-9 | |
| `"_dd` | | Nothing saved |
| `dw` | Saves to "", - | |
| `"_dw` | | Nothing saved |
| `cw` | Saves to "", -, 1-9 | |
| `"_cw` | | Nothing saved |

## Configuration


## Best Practices

### When to Use

1. Replacing text with yanked content
2. Recording clean macros
3. Deleting without history pollution
4. Multi-step replacements

### When Not Needed

1. Simple deletions you might undo
2. When you want register history
3. One-off operations

## Keybinding Suggestions

Some users map common operations to always use black hole:


## Visual Mode


## With Operators

Black hole works with all delete/change operators:

| Normal | Black Hole |
|--------|------------|
| `d{motion}` | `"_d{motion}` |
| `c{motion}` | `"_c{motion}` |
| `x` | `"_x` |
| `X` | `"_X` |
| `s` | `"_s` |
| `S` | `"_S` |

## API Reference


## Why "_"?

The underscore character was chosen because:
- Visually suggests "nothing" or "blank"
- Not used by other registers
- Easy to type with shift key
- Mnemonic: "underline" = below/nothing

## See Also

- [special-registers.md](special-registers.md) - Other special registers
- [named-registers.md](named-registers.md) - Named registers
- [numbered-registers.md](numbered-registers.md) - Numbered registers
