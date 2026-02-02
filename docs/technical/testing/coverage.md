# Code Coverage Configuration

Setting up code coverage for kjxlkj development.

## Tools

### cargo-llvm-cov

Recommended tool for Rust coverage.


### tarpaulin

Alternative coverage tool.


## Basic Usage

### Generate Report


### Coverage Formats


## Configuration

### .cargo/config.toml


### Exclude Patterns


## CI Integration

### GitHub Actions


## Coverage Targets

| Component | Target |
|-----------|--------|
| Core | 80% |
| Editor | 70% |
| Input | 75% |
| Render | 60% |
| Features | 70% |

## Viewing Results

### HTML Report


### VS Code

Install Coverage Gutters extension for inline display.

## Exclusions

### Ignore Lines


### Ignore Functions


## Best Practices

1. Run coverage before PRs
2. Don't chase 100% coverage
3. Focus on critical paths
4. Test edge cases

## Makefile Targets

- **coverage** - Runs `cargo llvm-cov --html --open` to generate and view HTML report
- **coverage-ci** - Runs `cargo llvm-cov --lcov` to generate lcov.info for CI integration
