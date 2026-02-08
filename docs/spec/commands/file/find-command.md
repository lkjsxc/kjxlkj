# Find Command

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

The `:find` command for locating and opening files by name.

## Overview

`:find {filename}` searches for a file in the configured path and opens it.

## Syntax

`:find[!] {filename}`

## Search Path

The `path` setting determines where `:find` looks:

| Component | Description |
|---|---|
| `.` | Current file's directory |
| (empty) | Current working directory |
| `**` | Recursive subdirectories |

Default: `path = [".", "**"]`

## Behavior

1. Search each path component for `{filename}`.
2. If found, open the file.
3. If multiple matches, open the first one.
4. If not found, emit an error.

## Wildcards

`:find *.rs` — opens the first `.rs` file found in the path.

## Tab Completion

`:find ` + `<Tab>` completes filenames found in the path.

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- File exploration: [/docs/spec/commands/file/file-exploration.md](/docs/spec/commands/file/file-exploration.md)
- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
