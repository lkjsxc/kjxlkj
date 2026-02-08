# Argument List Commands

Back: [/docs/spec/commands/buffer/README.md](/docs/spec/commands/buffer/README.md)

Manage the argument list (files passed on the command line).

## Overview

The argument list is the set of files specified when launching the editor. Commands allow navigating, modifying, and operating on this list.

## Navigation

| Command | Description |
|---|---|
| `:next` | Edit next file in argument list |
| `:prev` | Edit previous file |
| `:first` | Edit first file |
| `:last` | Edit last file |
| `:argument {N}` | Edit Nth file |

## Modify List

| Command | Description |
|---|---|
| `:args {files}` | Set argument list to `{files}` |
| `:argadd {file}` | Add file to argument list |
| `:argdelete {pattern}` | Remove files matching pattern |
| `:args` | Display current argument list |

## Apply Command to All

| Command | Description |
|---|---|
| `:argdo {cmd}` | Execute `{cmd}` on each file in arglist |
| `:argdo update` | Save all modified files |

## Related

- Buffer navigation: [/docs/spec/commands/buffer/buffer-navigation.md](/docs/spec/commands/buffer/buffer-navigation.md)
- Buffer commands: [/docs/spec/commands/buffer/README.md](/docs/spec/commands/buffer/README.md)
