# Remote Editing

Edit files on remote systems.

## Overview

kjxlkj supports editing files over SSH connections
and within remote containers using a thin remote
protocol.

## SSH Integration

### Direct Command

`kjxlkj scp://host/path/to/file` opens a remote file.
This uses SSH under the hood for transport.

### Authentication

SSH authentication uses the system SSH agent, key files
from `~/.ssh/`, or interactive password prompts.

### Configuration

Remote hosts can be configured in the editor:
- `ssh_command = "ssh"` — SSH binary path
- `ssh_args = ["-o", "StrictHostKeyChecking=no"]`
- `remote_timeout = 30` — Connection timeout in seconds

## Remote Protocol

### Architecture

The remote protocol runs a small helper binary on the
remote machine that handles file I/O and sends results
back over the SSH channel.

### File Operations

| Operation | Description |
|-----------|-------------|
| Read file | Transfer file contents to local |
| Write file | Send buffer contents to remote |
| List directory | Get directory listing |
| File info | Get size, permissions, modified time |
| Watch file | Monitor for external changes |

### Caching

Remote file contents are cached locally in
`~/.cache/kjxlkj/remote/`. The cache is invalidated
when the remote file's modification time changes.

## Container Support

### Docker

`kjxlkj docker://container_id/path` edits files inside
Docker containers using `docker exec`.

### Devcontainer

Integration with devcontainer.json for VS Code-style
development containers.

## LSP Over Remote

### Remote LSP

LSP servers run on the remote machine. The remote helper
forwards LSP messages between the local editor and the
remote language server.

### Latency

LSP operations have additional latency due to network
round trips. The editor handles this asynchronously
without blocking the UI.

## Terminal Over Remote

### Remote Shell

`:terminal ssh host` opens a remote terminal session.
The terminal runs on the remote machine.

## Limitations

### Performance

Large files may be slow to open/save over slow
connections. The editor shows progress indicators.

### Binary Files

Binary file editing is not supported over remote
connections.

### Concurrent Edits

No real-time collaboration support. File locks are
advisory only.
