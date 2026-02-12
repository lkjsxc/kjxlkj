# Command Execution Context

Back: [/docs/spec/commands/README.md](/docs/spec/commands/README.md)

This document defines which target is affected when an ex command executes.

## Core Rule

Unless explicitly defined as global, an ex command applies to the focused window
and the buffer bound to that window.

## Scope Classes

| Class | Meaning | Examples |
|---|---|---|
| `window-local` | affects only focused window/buffer binding | `:e`, `:w`, `:wq`, `:x`, `:q`, `:close`, `:read` |
| `buffer-local` | affects current buffer state, visible in all windows showing same buffer | `:setlocal`, write-modified state changes, text edits |
| `explicit-global` | affects all windows/tabs/buffers by design | `:qa`, `:wa`, `:wqa`, `:xa` |

## Normative Mapping

| Command | Scope | Required Behavior |
|---|---|---|
| `:e` | `window-local` | replace buffer binding in focused window only |
| `:w` | `window-local` + `buffer-local` | write focused buffer; do not retarget other windows |
| `:q` | `window-local` | close focused window only |
| `:wq`, `:x` | `window-local` | write focused buffer then close focused window |
| `:r` | `window-local` + `buffer-local` | insert into focused buffer at focused cursor/range |
| `:wa` | `explicit-global` | write all modified buffers |
| `:qa` | `explicit-global` | close all windows and exit |
| `:wqa`, `:xa` | `explicit-global` | write all modified buffers then exit |

## Chained Command Rule

For `:cmd1 | cmd2 | cmd3`:

1. each command runs against the current focused window at that step
2. if one command changes focus, later commands use the new focus
3. failure in a command stops subsequent commands unless command semantics define continuation

## Required Diagnostics

For command execution traces, emit:

- command text
- resolved command ID
- scope class
- focused window ID before and after execution
- target buffer ID before and after execution
- result (`ok`, `error`, `aborted`)

## Mandatory Verification

| ID | Scenario | Required Assertions |
|---|---|---|
| `CMD-01` | `:q` from split layout | exactly one focused window is closed |
| `CMD-02R` | `:e` and `:w` in one pane of two-pane layout | non-focused pane binding remains unchanged |
| `CMD-03` | `:wa` and `:qa` | global effect spans all relevant buffers/windows |
| `CMD-04` | chained `:e file | w` | second command uses post-`:e` focused buffer |

## Related

- syntax: [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md)
- file commands: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)
- quit commands: [/docs/spec/commands/quit-commands.md](/docs/spec/commands/quit-commands.md)
