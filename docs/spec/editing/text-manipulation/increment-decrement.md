# Increment/Decrement

Number and value manipulation under cursor.

## Basic Operations (normative)

| Key | Action |
|---|---|
| `<C-a>` | Increment number under/after cursor by 1 |
| `{count}<C-a>` | Increment by count |
| `<C-x>` | Decrement number under/after cursor by 1 |
| `{count}<C-x>` | Decrement by count |

The cursor searches forward on the current line for the first number if not already on one.

## Supported Number Formats

| Format | Prefix | Example | Detection |
|---|---|---|---|
| Decimal | (none) | `42`, `-7` | Default |
| Hexadecimal | `0x` / `0X` | `0xff` | When `nrformats` contains `hex` |
| Octal | `0` | `077` | When `nrformats` contains `octal` |
| Binary | `0b` / `0B` | `0b1010` | When `nrformats` contains `bin` |
| Unsigned | (none) | `255` | When `nrformats` contains `unsigned` |

Default `nrformats`: `bin,hex`.

## Negative Numbers

- `-5` + `<C-a>` → `-4`
- `-1` + `2<C-a>` → `1` (crosses zero)

## Visual Mode

With a visual selection containing numbers:

| Key | Action |
|---|---|
| `<C-a>` | Increment all numbers in selection by 1 |
| `<C-x>` | Decrement all numbers in selection by 1 |
| `g<C-a>` | Sequentially increment (1st +1, 2nd +2, etc.) |
| `g<C-x>` | Sequentially decrement |

Sequential increment is useful for creating numbered lists from repeated zeros.

## Alphabetic Increment

When `nrformats` contains `alpha`:

- `a` + `<C-a>` → `b`
- `z` + `<C-a>` → wraps or stops depending on config

## Related

- Text manipulation: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
