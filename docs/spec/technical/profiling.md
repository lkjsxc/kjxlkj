# Performance Profiling Guide

Guide for profiling and optimizing kjxlkj.

## Built-in Metrics

### Startup Time


Target: < 50ms for version output.

### Frame Time Logging


Logs render times to stderr or log file.

## CPU Profiling

### Using perf (Linux)


### Using Instruments (macOS)


### Using cargo-flamegraph


## Memory Profiling

### Using Valgrind/Massif


### Using heaptrack (Linux)


### Memory Usage Check


## Benchmarks

### Running Benchmarks


### Specific Benchmark


### Benchmark Comparison


## Common Bottlenecks

### Syntax Highlighting

- Large files with complex syntax
- Solution: Limit visible range highlighting

### LSP Communication

- Large completion lists
- Solution: Limit results, async loading

### File I/O

- Large file opening
- Solution: Memory-mapped files, streaming

### Rendering

- Complex UI with many elements
- Solution: Dirty-region tracking

## Performance Targets

| Operation | Target |
|-----------|--------|
| Startup | < 100ms |
| Keystroke latency | < 16ms |
| File open (1MB) | < 200ms |
| File open (100MB) | < 2s |
| Search (100k lines) | < 500ms |

## Profiling in Tests

