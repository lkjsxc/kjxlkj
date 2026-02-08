# Source Command

Back: [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)

Execute commands from a file.

## Overview

`:source {file}` reads a file and executes each line as an ex command. This is the primary mechanism for loading configuration.

## Syntax

`:source[!] {file}`

## Behavior

1. Read the file line by line.
2. Execute each line as an ex command.
3. Stop on error (unless `!` is used).

## Configuration Loading

At startup, the editor sources the main configuration file. Additional files can be sourced explicitly.

## Reload Configuration

`:source $MYVIMRC` — re-read the main configuration file to apply changes.

## With Bang

`:source! {file}` — continue executing even if a line produces an error.

## Related

- Execute: [/docs/spec/commands/execution/execute-command.md](/docs/spec/commands/execution/execute-command.md)
- Startup: [/docs/spec/architecture/startup.md](/docs/spec/architecture/startup.md)
- Configuration: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
