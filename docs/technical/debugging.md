# Debugging Techniques

Debugging kjxlkj during development.

## Logging

### Enable Debug Logging


### Specific Modules


### Log Levels

| Level | Use |
|-------|-----|
| error | Failures |
| warn | Issues |
| info | Events |
| debug | Details |
| trace | Everything |

## Log Output

### To File


### Structured Logs


## GDB/LLDB

### Build with Debug Info


### Run in GDB


### LLDB (macOS)


### Breakpoints


## VS Code Debugging

### launch.json


## Terminal Debugging

### Alternate Screen

TUI uses alternate screen. For debugging:


### Raw Mode Issues


## Panic Debugging

### Backtrace


### Panic Hook


## Memory Debugging

### Valgrind


### AddressSanitizer


## Performance Debugging

### CPU Profiling


### Flamegraph


## Test Debugging

### Single Test


### With Logging


## Tips

1. Use `eprintln!` for quick debugging
2. Enable backtraces always during dev
3. Keep debug builds for iteration
4. Use release builds for profiling
