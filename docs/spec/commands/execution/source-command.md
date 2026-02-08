# Source Command

Back: [docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)

Execute ex commands from a file.

## Overview

The `:source` command reads a file and executes each
line as an ex command. This is the primary mechanism
for loading configuration and running batch command
scripts.

## Syntax

### Basic Usage

`:source {file}` reads the file and executes each
non-blank, non-comment line as an ex command.

### Source Current File

`:source %` sources the current buffer's file.
`:source` with no argument sources the default
configuration file (`~/.config/kjxlkj/config.toml`).

## File Format

### Command Lines

Each line is treated as an independent ex command.
Lines are executed sequentially.

### Comments

Lines starting with `"` (double quote) are treated
as comments and skipped.

### Line Continuation

A line ending with `\` continues on the next line.
The backslash and newline are replaced with a space.

### Blank Lines

Empty lines and lines containing only whitespace
are silently ignored.

## TOML Configuration

### Config File Processing

The primary configuration file uses TOML format,
not ex command format. When `:source` encounters
a `.toml` file, it parses it as TOML configuration
and applies the settings.

### Setting Application

TOML settings are applied immediately. Changed
settings take effect on the next render cycle.

## Execution Behavior

### Error Handling

If a command fails during sourcing, the error is
reported but subsequent commands continue executing.
All errors are collected and displayed after the
file is fully processed.

### Error Format

Each error shows the filename, line number, and
the error message:
`Error on line 42 of config.toml: unknown option "foo"`

### Nested Source

`:source` can be called from within a sourced file.
Nesting depth is limited to 10 to prevent infinite
recursion. Exceeding the limit produces an error.

## Scope

### Option Scope

Options set during `:source` affect global scope
unless explicitly scoped with `:setlocal`.

### Mapping Scope

Mappings defined during `:source` are global unless
`<buffer>` qualifier is used.

## Startup Sourcing

### Automatic Source

At startup, the editor automatically sources:
1. `~/.config/kjxlkj/config.toml` (main config)
2. `.kjxlkj.toml` in the workspace root (if exists)

### Skip Init

`--clean` flag skips all automatic configuration
sourcing, starting with default settings.

## Profiling

### Source Timing

`:verbose source {file}` reports the time taken
to process each command in the file.

## Related

- Configuration: [docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
- Execute command: [docs/spec/commands/execution/execute-command.md](/docs/spec/commands/execution/execute-command.md)
- Startup: [docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
