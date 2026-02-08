# Alternate File

Back: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

Quick switching between the current and alternate (previous) buffer.

## Overview

The alternate file is the buffer that was most recently active before the current one. It provides a fast way to toggle between two files.

## Basic Usage

| Key | Command | Description |
|---|---|---|
| `<C-^>` / `<C-6>` | `:e #` | Switch to alternate file |
| `{N}<C-^>` | `:e #{N}` | Switch to buffer number N |

## How It Works

The alternate file is updated whenever you switch buffers:

1. Buffer A is active. Alternate is empty.
2. Open buffer B. Alternate becomes A.
3. Open buffer C. Alternate becomes B.
4. Press `<C-^>`. Switch to B, alternate becomes C.

## Alternate File Register

The alternate file path is stored in the `#` register.

| Register | Content |
|---|---|
| `%` | Current file path |
| `#` | Alternate file path |

These can be used in commands: `:e #` edits the alternate file.

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `alternate.patterns` | array | `[]` | Pattern rules for source/test switching |

## Alternate Patterns

User-defined patterns allow switching between related files:

| From | To | Pattern |
|---|---|---|
| `src/foo.rs` | `tests/foo_test.rs` | `src/*.rs` → `tests/*_test.rs` |
| `lib/bar.js` | `test/bar.test.js` | `lib/*.js` → `test/*.test.js` |
| `include/x.h` | `src/x.c` | `include/*.h` → `src/*.c` |

## Commands

| Command | Description |
|---|---|
| `:e #` | Edit alternate file |
| `:sp #` | Split and edit alternate file |
| `:vs #` | Vertical split and edit alternate file |

## Fallback Behavior

If the alternate file does not exist on disk, the editor offers to create it (if `alternate.create_missing` is `true`). Otherwise, an error is emitted.

## Related

- Buffer management: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)
- Buffer list: [/docs/spec/features/buffer/buffer-list.md](/docs/spec/features/buffer/buffer-list.md)
