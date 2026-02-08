# Alternate File (#)

Switch between related files.

## Overview

The alternate file (`#`) provides quick access to the
previously edited file. It enables fast toggling between
two buffers.

## Basic Usage

### Toggle Alternate

`<C-^>` (or `<C-6>` on some terminals) switches between
the current file and the alternate file.

### With Count

`{count}<C-^>` switches to buffer number `{count}`.
This overrides the alternate file for the jump.

## How It Works

### Automatic Assignment

When you switch buffers, the previous buffer becomes
the alternate file. The alternate is tracked per-window.

### Flow Example

1. Open `main.rs` (current: main.rs, alt: none)
2. Open `lib.rs` (current: lib.rs, alt: main.rs)
3. `<C-^>` switches to `main.rs` (current: main.rs, alt: lib.rs)
4. `<C-^>` switches to `lib.rs` (current: lib.rs, alt: main.rs)

## Alternate File Register

### Check Alternate

`:echo @#` displays the alternate file path.
`<C-g>` shows it in the file info line.

### In Commands

`#` expands to the alternate file path in ex commands:
`:e #` opens the alternate file.
`:sp #` opens it in a split.

## Configuration

### Keybindings

| Key | Action |
|-----|--------|
| `<C-^>` | Toggle alternate (default) |
| `<Leader>a` | Toggle alternate (common remap) |

## Alternate Patterns

### Source/Test Switching

Configurable patterns in TOML under `[alternate]`:
rules map source files to their test files and vice versa.

| Source Pattern | Alternate Pattern |
|----------------|-------------------|
| `src/{}.rs` | `tests/{}_test.rs` |
| `src/{}.ts` | `src/{}.test.ts` |
| `lib/{}.rb` | `spec/{}_spec.rb` |

### Header/Source

For C/C++ projects:

| Source | Alternate |
|--------|-----------|
| `src/{}.c` | `include/{}.h` |
| `src/{}.cpp` | `include/{}.hpp` |

## Related Files

### Multiple Alternates

When alternate patterns produce multiple candidates,
`:AlternateList` shows them in a picker. The first
existing file is used as the default alternate.

### Cycle Related

`:AlternateNext` cycles through related files in order.
`:AlternatePrev` cycles backward.

## Commands

### Alternate Command

`:b #` opens the alternate buffer.
`:e #` edits the alternate file (reloads from disk).

### Split Alternate

`:sb #` opens the alternate in a horizontal split.
`:vert sb #` opens in a vertical split.

## Fallback Behavior

### Create If Missing

When `alternate.create_if_missing = true`, if the
alternate file does not exist, a new buffer with that
path is created (unsaved).

### Prompt

When `alternate.create_if_missing = false` (default),
attempting to switch to a non-existent alternate shows
"No alternate file".

## Project-Local Patterns

### Per-Project

Alternate patterns in `.kjxlkj.toml` at the project root
override global patterns.

## Functions

### Get Alternate

`alternate({path})` returns the computed alternate file
path based on configured patterns.

### Set Alternate

The alternate file is set automatically. To override:
`:let @# = "path/to/file"`.

## Multiple Alternates

### Alternate List

When multiple alternates exist (e.g. source has both
unit test and integration test), the list is stored
internally and accessible via `:AlternateList`.

### Select Alternate

`:Alternate {n}` selects the nth alternate from the list.
