# Mutation Testing

Verify test quality with mutation testing.

## Overview

Mutation testing modifies code to check if tests
catch the changes. Uncaught mutations indicate
weak tests.

## Tools

### cargo-mutants


## Basic Usage

### Run All Mutations


### Specific Module


## Output

### Terminal


### Summary


## Configuration

### mutants.toml


## Mutation Types

| Mutation | Example |
|----------|---------|
| Replace operator | `+` → `-` |
| Negate condition | `>` → `<=` |
| Replace constant | `1` → `0` |
| Remove statement | Delete line |
| Replace return | `true` → `false` |

## Interpreting Results

### Killed Mutants

Tests detected the change. Good.

### Survived Mutants

Tests didn't catch change. Review needed.

### Timeout Mutants

Mutant caused infinite loop. Usually OK.

## Strategies

### When to Run

- Before releases
- After major refactors
- Weekly in CI

### Focus Areas

1. Business logic
2. Parsing code
3. State transitions
4. Edge cases

## CI Integration

### GitHub Actions


## Improving Score

### Survived Mutant


## Performance

### Parallel Execution


### Incremental


## Makefile Targets

- **mutants** - Runs full mutation testing with `cargo mutants`
- **mutants-fast** - Tests only mutations in recent changes with `--in-diff HEAD~1`
