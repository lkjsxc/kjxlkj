# Named Registers

Alphabetic registers (a-z, A-Z) for storing and retrieving text.

## Register Behavior (normative)

| Register | Access | Behavior |
|---|---|---|
| `"a` – `"z` | Read/Write | Overwrite contents |
| `"A` – `"Z` | Write-only | Append to corresponding lowercase register |

## Usage

### Yank to Register

`"ay{motion}` yanks the text described by `{motion}` into register `a`.

### Delete to Register

`"ad{motion}` deletes text and stores it in register `a`.

### Paste from Register

`"ap` pastes the contents of register `a` after the cursor. `"aP` pastes before.

## Append Mode (Uppercase)

Using an uppercase letter appends text instead of overwriting. `"Ayy` appends the current line to what is already in `"a`. This is useful for collecting multiple text fragments.

## Insert Mode Access

| Key | Action |
|---|---|
| `<C-r>a` | Insert register `a` contents |
| `<C-r><C-r>a` | Insert literally (no remapping) |
| `<C-r><C-o>a` | Insert without auto-indent |
| `<C-r><C-p>a` | Insert with auto-indent fixed |

## Command Line Access

`<C-r>a` on the command line inserts register `a` contents into the command.

## Visual Mode

In visual mode, `"ay` yanks the selection to register `a`. `"ap` replaces the selection with register `a`.

## Viewing Contents

`:registers a` shows the content of register `a`. `:registers` shows all registers.

## Persistence

Named registers are saved across sessions when session persistence is enabled. See [/docs/spec/editing/marks/mark-persistence.md](/docs/spec/editing/marks/mark-persistence.md).

## Linewise vs Characterwise

Registers store the type (characterwise, linewise, blockwise) alongside the text. Paste behavior depends on this stored type.

## Common Workflows

- **Multi-paste**: Yank to `"a`, paste repeatedly with `"ap` (unlike default register, `"a` is not overwritten by subsequent deletes)
- **Collecting text**: Use `"Ay` to append yanked lines to register `a`
- **Replace without losing**: Yank replacement text to `"a`, visually select target, `"ap` replaces selection while default register gets the old text

## Related

- Numbered registers: [/docs/spec/editing/registers/numbered-registers.md](/docs/spec/editing/registers/numbered-registers.md)
- Registers overview: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
