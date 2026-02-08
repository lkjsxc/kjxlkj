# Numbered Registers

Back: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

Automatic registers `"0` through `"9` that store recent yank and delete history.

## Register `"0` (yank register)

The yank register stores the text from the most recent yank operation (`y{motion}`, `Y`, `yy`). It is NOT overwritten by delete or change operations.

| Operation | Updates `"0`? |
|---|---|
| `yy` (yank line) | Yes |
| `yw` (yank word) | Yes |
| `dd` (delete line) | No |
| `dw` (delete word) | No |
| `cw` (change word) | No |

## Registers `"1` through `"9` (delete history)

These registers form a queue of the 9 most recent delete operations that contain at least one full line.

| Register | Content |
|---|---|
| `"1` | Most recent delete/change text (linewise or multi-line) |
| `"2` | Previous delete (shifted from `"1`) |
| `"3` | Previous delete (shifted from `"2`) |
| ... | ... |
| `"9` | Oldest stored delete |

### Shift behavior

When a new linewise delete occurs:

1. `"9` content is discarded
2. `"8` moves to `"9`
3. `"7` moves to `"8`
4. ... and so on
5. `"1` moves to `"2`
6. The new delete text goes to `"1`

### Small deletes

Deletes of less than one line (e.g., `dw` within a line) go to the small delete register `"-` instead of the numbered registers, unless a specific register was specified.

## Unnamed register `""`

The unnamed register always points to the last used register. After a yank, `""` contains the yanked text (same as `"0`). After a delete, `""` contains the deleted text (same as `"1`).

## Paste from numbered registers

| Command | Description |
|---|---|
| `"0p` | Paste from yank register (most recent yank) |
| `"1p` | Paste most recent delete |
| `"2p` | Paste second most recent delete |

## Use case

The most common pattern: yank text with `y`, delete the target with `d` (which goes to `"1`), then paste the yanked text with `"0p` (which was not overwritten by the delete).

## Related

- Named registers: [/docs/spec/editing/registers/named-registers.md](/docs/spec/editing/registers/named-registers.md)
- Black hole register: [/docs/spec/editing/registers/blackhole-register.md](/docs/spec/editing/registers/blackhole-register.md)
- Register commands: [/docs/spec/editing/registers/register-commands.md](/docs/spec/editing/registers/register-commands.md)
