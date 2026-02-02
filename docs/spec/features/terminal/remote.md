# SSH/Remote Editing

Editing files on remote systems.

## Methods

### 1. SSH + Local kjxlkj

Run kjxlkj locally, edit remote files via mount.


### 2. Remote kjxlkj

Run kjxlkj on remote system.


## SSHFS Setup

### Installation


### Mounting


### Recommended Options


### Unmounting


## Remote kjxlkj

### Installation on Remote


### Running


### Persistent Session


## Configuration Sync

### dotfiles


### Minimal Remote Config


## Performance

### Latency Mitigation


### Compression


## Clipboard

### SSH Forwarding


### OSC52 Clipboard


## Mosh

### Better Than SSH


### Benefits

- Handles disconnects
- Lower latency feel
- Local echo

## Port Forwarding

### LSP Over SSH


Run LSP server on remote, connect locally.

## Troubleshooting

### Slow Connection


### Broken Pipe

- Use mosh instead of SSH
- Add keep-alive options

### Display Issues


## Best Practices

1. Use SSHFS for occasional edits
2. Use remote kjxlkj for heavy work
3. Use tmux for persistent sessions
4. Enable OSC52 clipboard
5. Consider mosh for unreliable connections
