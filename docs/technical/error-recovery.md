# Error Recovery Strategies

How kjxlkj handles and recovers from errors.

## Philosophy

- Never lose user data
- Fail gracefully
- Provide recovery options
- Log for debugging

## Error Categories

### Recoverable

| Error | Recovery |
|-------|----------|
| LSP timeout | Retry with backoff |
| Network error | Use cached data |
| Parse error | Show partial result |
| Missing file | Create or prompt |

### Non-Recoverable

| Error | Handling |
|-------|----------|
| Out of memory | Save and exit |
| Disk full | Emergency save |
| Permission denied | Read-only mode |

## Buffer Recovery

### Unsaved Changes


### Backup Strategy


### Swap Files


Recovered on next open.

## Crash Recovery

### Crash Handler


### Recovery on Restart


## LSP Error Handling

### Timeout


### Server Crash


Automatic restart with exponential backoff.

## File System Errors

### Permission Denied


### Disk Full


### Missing Directory


## Network Errors

### Clipboard


### Remote Files

Automatic retry with status indication.

## User Notification

### Severity Levels

| Level | Display |
|-------|---------|
| Info | Statusline |
| Warning | Popup, auto-dismiss |
| Error | Modal dialog |
| Critical | Force action |

## Logging

### Error Log

Location: `~/.local/share/kjxlkj/error.log`

### Format


## Testing Recovery

### Simulate Errors


### Chaos Testing

Randomly inject errors in development builds.

## Best Practices

1. Always have undo available
2. Never corrupt on partial write
3. Atomic file operations
4. Preserve user intent
