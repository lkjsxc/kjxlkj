# Network File Systems

Working with files on network file systems.

## Supported Systems

| System | Support |
|--------|---------|
| NFS | Full |
| SMB/CIFS | Full |
| SSHFS | Full |
| AFP | Basic |

## Considerations

### Latency

Network operations are slower than local.


### Reliability

Network may disconnect during edits.

## Auto-Save

### Enabled by Default


### On Focus Lost


## Conflict Detection

### File Changed Externally


### Check Frequency


## Caching

### Read Cache


### Write-Through

Writes always go directly to disk.

## Error Handling

### Timeout


### Disconnection


## Performance Tips

### Local Working Copy


### Reduce Requests


## NFS Specific

### Lock Files


### Hard Links

Undo files may use hard links.

## SMB Specific

### Windows Shares


## SSHFS

### Mount Options


### Integration

SSHFS appears as local filesystem.

## Offline Mode

### When Disconnected


### Sync on Reconnect

Queued writes applied when connection restored.

## Backup Strategy

### Local Backups


### Before Overwrite

Always backup before saving to network.

## Best Practices

1. Enable auto-save
2. Use local backups
3. Watch for external changes
4. Handle disconnects gracefully
5. Consider latency in workflows
