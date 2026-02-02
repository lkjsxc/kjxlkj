# Large File Handling

Strategies for editing large files efficiently.

## Definition

| Size | Category |
|------|----------|
| <1MB | Small |
| 1-10MB | Medium |
| 10-100MB | Large |
| >100MB | Very Large |

## Automatic Detection


## Optimizations Applied

### For Large Files

| Feature | Change |
|---------|--------|
| Syntax | Disabled/limited |
| LSP | Disabled |
| Undo history | Reduced |
| Line numbers | Relative disabled |

### Configuration


## Lazy Loading

### Chunked Reading

Files loaded in chunks, not entirely in memory.


### Virtual Scrolling

Only visible lines rendered.

## Memory Management

### Expected Usage

| File Size | RAM |
|-----------|-----|
| 10MB | ~30MB |
| 100MB | ~150MB |
| 1GB | ~1.5GB |

### Memory Limits


## Search Optimization

### Large File Search


### Incremental Search

Results shown as found, not after complete scan.

## Syntax Highlighting

### Disabled for Large


### Partial Highlighting

Only visible region highlighted.

## Line Limits

### Very Long Lines


### Display


## Undo Optimization

### Reduced History


### Checkpoint Strategy

Fewer checkpoints for large edits.

## File Opening

### Progress Indicator


### Async Loading

UI responsive during load.

## Recommended Workflow

1. Use grep for searching
2. Split large files when possible
3. Use streaming tools (sed, awk)
4. Consider alternative viewers (less)

## Commands

| Command | Description |
|---------|-------------|
| `:LargeFileMode` | Toggle optimizations |
| `:set wrap` | Enable line wrap |
| `:syntax off` | Disable syntax |

## Warning Dialog


## Performance Tips

1. Disable syntax highlighting
2. Use narrower viewport
3. Close unused buffers
4. Avoid global operations

## Limits

### Tested Maximum

- 1GB file: Opens, basic editing
- 100k line files: Full functionality
- 1M line files: Limited functionality
