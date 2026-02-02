# Memory Optimization

kjxlkj is designed for efficient memory usage.

## Core Data Structures

### Rope-based Text Storage

Using `ropey` crate for text:
- O(log n) insertions and deletions
- Memory efficient for large files
- Copy-on-write for undo snapshots

### Memory Characteristics

| Operation | Memory Impact |
|-----------|---------------|
| Open 1MB file | ~1.5MB (rope overhead) |
| Undo snapshot | Incremental (shared nodes) |
| Selection | O(1) - just positions |
| Search results | Lazy iteration |

## Configuration

### Buffer Limits


### Cache Sizing


## Large File Handling

### Streaming Open

Files over threshold use streaming:

Large files:
- Partial syntax highlighting
- Limited undo history
- Disabled features (configurable)

### Memory-Mapped Files

For very large files (optional):

## Undo Memory Management

### Incremental Snapshots

Undo uses copy-on-write:
- Only changed rope nodes copied
- Typical undo step: < 1KB

### Undo Pruning

When memory pressure:
1. Oldest undo states pruned first
2. Merge adjacent small changes
3. Preserve branch points


## LSP Memory

### Completion Limits


### Cache Eviction

LRU cache for LSP responses:
- Hover info
- Signature help
- Symbol information

## Syntax Highlighting

### Visible Range Only

Only highlight visible + buffer lines:

### Parser Memory

Tree-sitter parsers per buffer:
- Reuse parsers when possible
- Unload for hidden buffers

## Monitoring Memory

### Debug Mode


Logs memory stats periodically.

### Runtime Check

`:memory` command shows current usage.

## Reducing Memory Usage

Tips for constrained environments:

