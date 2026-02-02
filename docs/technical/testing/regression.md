# Performance Regression Tests

Prevent performance regressions with automated benchmarks.

## Overview

Track performance across commits to catch regressions
before they reach users.

## Criterion Benchmarks

### Location


### Running Benchmarks


## Baseline Comparison

### Save Baseline


### Compare to Baseline


## Benchmark Examples

### Buffer Operations


### Rendering


## CI Integration

### GitHub Actions


### Benchmark Bot

Use GitHub Actions to comment benchmark results on PRs.

## Thresholds

### Acceptable Variance

| Operation | Threshold |
|-----------|-----------|
| Buffer ops | ±5% |
| Rendering | ±10% |
| Input | ±5% |
| LSP | ±20% |

## Tracking Over Time

### Criterion Reports


### Export Data


## Custom Harness

### Simple Timing


## Benchmark Categories

### Micro-benchmarks

- Character insertion
- Single line render
- Key parsing

### Macro-benchmarks

- File loading
- Full render cycle
- Search across file

## Preventing Regressions

1. Run benchmarks in CI
2. Block PRs on significant regressions
3. Track trends over time
4. Review benchmark results in code review

## Makefile Targets

- **bench** - Runs benchmarks with `cargo bench`
- **bench-save** - Saves benchmark baseline using current commit hash
- **bench-compare** - Compares against the main branch baseline
