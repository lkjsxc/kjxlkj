# Undo File Persistence

Persistent undo across sessions.

## Overview

Save undo history to disk for
recovery after restart.

## Enable

### Configuration


### Command


## Undo Directory

### Location


### Auto Create

Created automatically.

## Undo File Names

### Format


### Example


## How It Works

### On Save

Undo history written to disk.

### On Open

Undo history loaded from disk.

### Seamless

Continue undoing after restart.

## Undo Levels

### Limit History


### Unlimited


## File Size

### Maximum


### Over Limit

Undo not persisted for large files.

## Commands

### Check Status


### Clear Undo


## Undo Tree

### Visual


### Navigation


## Related Commands

### Undo/Redo


### Jump in History


## Privacy

### Sensitive Files

Undo files contain content history.

### Skip Patterns


## Security

### Permissions


### Encryption

Not encrypted by default.

## Cleanup

### Manual


### Age-Based


### Script


## Troubleshooting

### Undo Not Working

1. Check `:set undofile?`
2. Check directory permissions
