# Test Coverage Summary

Back: [/docs/reference/README.md](/docs/reference/README.md)

## Overview

This document provides a summary of the test coverage for the kjxlkj editor implementation.

**Last updated:** Iteration 34

## Test Statistics

| Metric | Value |
|--------|-------|
| Total unit tests | 537 |
| Test files | 40+ |
| Crates with tests | 18 |

## Coverage by Component

### Core Editing

| Component | Test Count | Coverage |
|-----------|------------|----------|
| Motions | 35+ | Comprehensive |
| Operators | 30 | Full |
| Text Objects | 10 | Full |
| Cursor Movement | 23 | Comprehensive |
| Mode Transitions | 30 | Full |
| Search | 17 | Comprehensive |
| Undo/Redo | 15 | Full |

### Editor State

| Component | Test Count | Coverage |
|-----------|------------|----------|
| Advanced Editing | 21 | Marks, Macros, Registers |
| Ex Commands | 20 | Essential commands |
| Command Mode | 11 | Mode handling |
| Operators Integration | 30 | With motions |

### UI and Rendering

| Component | Test Count | Coverage |
|-----------|------------|----------|
| Viewport | 21 | Scroll, bounds |
| Snapshots | 10 | Immutability, correctness |
| Render Diff | 15 | Change detection |

### Services

| Component | Test Count | Coverage |
|-----------|------------|----------|
| Supervision | 5 | Lifecycle, failure |
| Terminal | 3 | Basic IO |
| Git | 2 | Status |
| LSP | 4 | Protocol |
| Index | 4 | Search |

### Integration

| Component | Test Count | Coverage |
|-----------|------------|----------|
| E2E Event Loop | 5 | Full cycle |
| Architecture | 15 | Invariants |

## Test Categories

### Unit Tests

Located in `src/crates/*/src/` as inline `#[cfg(test)]` modules:

- Type construction and validation
- Pure function behavior
- Module-internal logic

### Integration Tests

Located in `src/crates/*/tests/`:

- Cross-module interactions
- State machine behavior
- Command execution
- Mode transitions

### E2E Tests

Located in `src/crates/kjxlkj-host/tests/`:

- Full event loop cycles
- Snapshot → render pipelines
- Determinism verification

## Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test --package kjxlkj-core-state

# Run specific test file
cargo test --package kjxlkj-core-state --test operators

# Run with output
cargo test --workspace -- --nocapture
```

## Test Requirements

All new features must include:

1. Unit tests for core logic
2. Integration tests for interactions
3. Boundary condition tests
4. No-crash tests for edge cases

## Related Documents

- [CONFORMANCE.md](CONFORMANCE.md) - Implementation status
- [LIMITATIONS.md](LIMITATIONS.md) - Known gaps
- [/docs/technical/testing/README.md](/docs/technical/testing/README.md) - Testing guidelines
