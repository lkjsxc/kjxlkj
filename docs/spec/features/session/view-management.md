# View Management

Save and restore view states.

## Overview

Views capture window-specific state:
cursor position, folds, options.

## mkview Command

### Save View


### Default Location


## loadview Command

### Load View


## View Contents

### What's Saved

| Element | Saved |
|---------|-------|
| Cursor position | ✓ |
| Folds (manual) | ✓ |
| Fold options | ✓ |
| Local options | ✓ |
| Local mappings | Optional |
| Scroll position | ✓ |

### Not Saved

| Element | Reason |
|---------|--------|
| Window size | Session handles |
| Buffer content | File handles |
| Global options | Config handles |

## Automatic Views

### Save on Leave


### Load on Enter


### Configuration


## View Options

### viewoptions


### Default Options


## View Directory

### Configuration


### Per-Project


## Fold Persistence

### Manual Folds


### Marker Folds

Marker folds are in file content,
automatically persistent.

### Expression Folds

Cannot persist; recalculated on load.

## Cursor Restoration

### Last Position


### Implementation


## Scroll Restoration

### Restore Scroll


### Center After Restore


## Multiple Views

### Numbered Slots


### Named Views


## Session vs View

### Session

- All windows/tabs
- All buffers
- Global state
- Workspace-level
