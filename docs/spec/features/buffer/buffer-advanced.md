# Advanced Buffer Management

Deep buffer control and strategies.

## Overview

Beyond basic open/close, buffers support typed behavior, lifecycle management, hidden policies, and batch operations.

## Buffer States (normative)

| State | In memory | In window | In list |
|---|---|---|---|
| Active | Yes | Yes | Yes |
| Hidden | Yes | No | Yes |
| Inactive (unloaded) | No | No | Yes |
| Wiped | No | No | No |

## Buffer Types (normative)

Set via the `buftype` option.

| `buftype` | Purpose | Writable | File-backed |
|---|---|---|---|
| (empty) | Normal file editing | Yes | Yes |
| `nofile` | Scratch/temporary | Yes | No |
| `nowrite` | Display only | Yes | No |
| `help` | Help documentation | No | Yes (read-only) |
| `quickfix` | Quickfix/location list | No | No |
| `terminal` | Terminal emulator buffer | No (special) | No |
| `prompt` | Input prompt | Restricted | No |

## Hidden Behavior (normative)

The `bufhidden` option controls what happens when a buffer has no window:

| Value | Behavior |
|---|---|
| (empty) | Use global `hidden` option |
| `hide` | Keep buffer loaded but hidden |
| `unload` | Unload buffer (free memory, keep in list) |
| `delete` | Delete buffer from list |
| `wipe` | Wipe buffer completely (remove from list) |

## Lifecycle Commands

| Command | Effect |
|---|---|
| `:badd {file}` | Add file to buffer list without loading |
| `:bunload {N}` | Unload buffer N (keep in list) |
| `:bdelete {N}` | Delete buffer N (remove from list) |
| `:bwipeout {N}` | Wipe buffer N completely |
| `:bufdo {cmd}` | Execute command in each listed buffer |

## Batch Operations

`:bufdo` iterates over all listed buffers, executing the given Ex command. The current buffer is saved/restored. Modified buffers without `hidden` set cause an error.

## Buffer-Local Options

| Option | Description |
|---|---|
| `buftype` | Buffer type (see above) |
| `bufhidden` | Hidden behavior |
| `buflisted` | Whether buffer appears in `:ls` |
| `swapfile` | Whether a swap file is created |
| `modified` | Read-only flag indicating unsaved changes |
| `modifiable` | Whether content can be modified |
| `readonly` | Whether buffer is read-only |

## Related

- Buffers: [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- Buffer listing: [/docs/spec/commands/buffer/buffer-listing.md](/docs/spec/commands/buffer/buffer-listing.md)
