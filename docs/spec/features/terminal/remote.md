# Remote Terminal

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

Terminal connections to remote hosts.

## Overview

The terminal can spawn SSH sessions to remote hosts. The remote terminal operates identically to local terminals from the editor's perspective.

## Commands

| Command | Description |
|---|---|
| `:terminal ssh {host}` | Open terminal with SSH connection |
| `:terminal ssh -t {host} {cmd}` | Run command on remote host |

## Behavior

The SSH process is spawned as a regular terminal subprocess. The editor's terminal emulator handles all escape sequences from the remote host.

## File Editing

Remote file editing is not directly supported through the terminal. Use `:e scp://{host}/{path}` for remote file access (if network FS support is enabled).

## Configuration

SSH configuration uses the user's `~/.ssh/config` for host aliases, keys, etc.

## Related

- Terminal: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Network FS: [/docs/technical/network-fs.md](/docs/technical/network-fs.md)
