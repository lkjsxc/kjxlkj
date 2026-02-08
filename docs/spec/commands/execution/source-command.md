# Source Command

Back: [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)

Execute ex commands from a file.

## Syntax (normative)

| Command | Action |
|---|---|
| `:source {file}` | Read and execute each line of `{file}` as an ex command |
| `:source %` | Re-source the current buffer (useful for editing config) |
| `:source` (no arg) | Source the default init file (`~/.config/kjxlkj/init.kjxlkj`) |

## File resolution (normative)

| Path form | Resolution |
|---|---|
| Absolute path | Used as-is |
| Relative path | Resolved from cwd |
| `~/{path}` | Expanded to user home directory |
| `%` | Current buffer file path |

## Startup sourcing (normative)

On startup, the editor sources:

1. Built-in defaults (compiled into binary)
2. `~/.config/kjxlkj/config.toml` (TOML config; parsed, not sourced as commands)
3. `~/.config/kjxlkj/init.kjxlkj` (command file; sourced line by line)
4. `.kjxlkj.toml` (project-local TOML config)

## Command file format

Each line is one ex command. Comments start with `"`. Empty lines are ignored. Lines MUST NOT contain leading `:` characters.

## Error handling (normative)

| Situation | Behavior |
|---|---|
| File not found | Display error: "Can't open file: {file}" |
| Command error | Stop sourcing at the failing line; display the error |
| `silent! source {file}` | Source silently; suppress errors |

## Related

- Execute command: [/docs/spec/commands/execution/execute-command.md](/docs/spec/commands/execution/execute-command.md)
- Script files: [/docs/spec/scripting/script-files.md](/docs/spec/scripting/script-files.md)
- Startup sequence: [/docs/spec/architecture/startup.md](/docs/spec/architecture/startup.md)
