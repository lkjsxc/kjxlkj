# Swap File Management

Crash recovery and swap files.

## Overview

Swap files protect against data loss from crashes. A swap file is created when a buffer is loaded and deleted on normal exit. If a crash occurs, the swap file enables recovery.

## Swap File Location (normative)

| Option | Default | Description |
|---|---|---|
| `directory` | `~/.local/share/kjxlkj/swap//` | Comma-separated list of directories for swap files |

When `//` is appended to a directory path, the swap file name encodes the full path of the original file (replacing `/` with `%`), preventing collisions.

## Swap File Naming

Swap files are named `{encoded_path}.swp`. If the directory option uses `//`, the encoded path replaces path separators with `%`. Example: `/home/user/file.rs` becomes `%home%user%file.rs.swp`.

## Enable / Disable

| Scope | Option | Effect |
|---|---|---|
| Global | `swapfile` (boolean) | Default for new buffers |
| Per-buffer | `:setlocal noswapfile` | Disable for one buffer |

## Write Timing (normative)

| Trigger | Condition |
|---|---|
| Timer | Every `updatetime` milliseconds (default 4000) of inactivity |
| Character count | After `updatecount` characters typed (default 200) |
| Explicit | On `:preserve` command |

## Swap Detection on Open (normative)

When opening a file, if a swap file already exists:

1. Display a warning dialog showing: swap file path, process ID that created it, last modified time.
2. Present choices:

| Choice | Action |
|---|---|
| (O)pen Read-Only | Open the file read-only, leave swap alone |
| (E)dit anyway | Open normally, create a new swap file |
| (R)ecover | Apply the swap file's changes to recover content |
| (D)elete it | Delete the stale swap file and open normally |
| (Q)uit | Abort opening this file |
| (A)bort | Abort the entire startup |

## Recovery Process

When the user chooses (R)ecover:

1. Read the swap file to reconstruct the buffer state at the time of the crash.
2. Mark the buffer as modified (since the recovered content differs from the file on disk).
3. The user should review the content, `:w` to save, then delete the swap file manually or via `:e` (which detects the swap is now stale).

## Automatic Cleanup

On normal exit (`:q`, `:wq`, `:x`), the swap file for each buffer is deleted. Swap files from crashed sessions remain until manually deleted or recovered.

## Multiple Editor Protection

The swap file's existence signals that another editor instance may have the file open. The swap file stores the creating process's PID. If the PID is still running, the editor can warn accordingly.

## Related

- Session management: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Buffers: [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
