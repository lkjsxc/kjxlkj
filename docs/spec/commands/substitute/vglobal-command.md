# VGlobal Command

Executing commands on non-matching lines.

## Overview

The `:vglobal` (`:v`) command
executes Ex commands on lines
NOT matching a pattern.

## Basic Syntax

```
:[range]v/pattern/command
```

Without a range, operates on the entire file (`%` is the default range). Executes the command on every line where the pattern does NOT match.

## Equivalents

| Form | Meaning |
|------|---------|
| `:v/pattern/command` | Execute on lines NOT matching `pattern` |
| `:g!/pattern/command` | Identical to `:v/pattern/command` |

Both do the same thing.

## Common Uses

### Keep Matching Lines

`:v/pattern/d` deletes every line that does NOT match, keeping only lines containing the pattern.

| Command | Effect |
|---------|--------|
| `:v/TODO/d` | Keep only lines containing `TODO` |
| `:v/error\|warn/d` | Keep only lines with `error` or `warn` |
| `:v/pattern/d` | Keep only lines matching `pattern` |

### Filter Results

| Command | Effect |
|---------|--------|
| `:v/^#/d` | Keep only comment lines |
| `:v/pattern/s/old/new/g` | Substitute only on non-matching lines |
| `:v/^$/normal >>` | Indent all non-empty lines |

## Inverse Logic

### Delete What Doesn't Match

`:v/keep_this/d` is the standard idiom. All lines not containing `keep_this` are deleted, leaving only relevant lines.

### Clean Non-Code

| Command | Effect |
|---------|--------|
| `:v/\S/d` | Delete all whitespace-only and blank lines |
| `:v/[a-zA-Z]/d` | Delete lines containing no letters |
| `:v/^\s*[^#]/d` | Keep only non-comment lines |

## Practical Examples

### Log Analysis

| Command | Effect |
|---------|--------|
| `:v/ERROR/d` | Keep only error lines from a log |
| `:v/2024-01-15/d` | Keep only entries from a specific date |
| `:v/\[CRITICAL\]/d` | Keep only critical-level entries |

### Code Filtering

| Command | Effect |
|---------|--------|
| `:v/^import/d` | Keep only import statements |
| `:v/def \|class /d` | Keep only function and class definitions |
| `:v/^\s*return/d` | Keep only return statements |

### Cleanup

| Command | Effect |
|---------|--------|
| `:v/\S/d` | Remove blank lines |
| `:v/[^[:space:]]/d` | Remove whitespace-only lines |
| `:v/./d` | Remove empty lines (same as `:g/^$/d`) |

## With Range

### Limit Scope

| Command | Effect |
|---------|--------|
| `:1,50v/pattern/d` | Filter within first 50 lines only |
| `:'<,'>v/keep/d` | Filter within visual selection |
| `:.,.+20v/data/d` | Filter in next 20 lines |

## Combining Patterns

### OR Logic

Use regex alternation to keep lines matching any term: `:v/error\|warning\|fatal/d` keeps lines containing any of those words.

### Complex Patterns

Keep function and class definitions.

`:v/^\(function\|class\|def\)/d` retains only lines starting with definition keywords.

## With Other Commands

### Print

`:v/pattern/p` prints all lines NOT matching the pattern. Useful for previewing what remains without modifying the buffer.

### Normal Mode

`:v/pattern/normal >>` indents all non-matching lines. `:v/^#/normal I// ` adds `// ` before all non-comment lines.

### Substitute

`:v/^#/s/foo/bar/g` replaces `foo` with `bar` only on non-comment lines. The substitute runs on every line where the `:v` pattern does not match.

## Practical Workflows

### Extract Code

`:v/^\(import\|from\)/d` extracts only import statements from a Python file for review.

### Configuration

`:v/^[^;#]/d` keeps only active (non-commented) lines from an INI or config file.

### Data Processing

`:v/^[0-9]/d` keeps only lines starting with a digit, useful for extracting numeric data rows from mixed-format files.

## Comparison: g vs v

### Global

`:g/pattern/d` acts on lines that MATCH the pattern. It deletes matching lines, leaving non-matches.

### VGlobal

`:v/pattern/d` acts on lines that do NOT match the pattern. It deletes non-matching lines, leaving matches.

### Choose Based on Goal

- More to delete → use `:g`
- More to keep → use `:v`

## Double Negative

### Keep Everything

`:v` cannot be chained. To apply multiple filters, run them sequentially: first `:v/error/d`, then `:v/2024/d`. Each pass further narrows the remaining lines.

## Complex Filtering

### Multi-Step

Run multiple passes to narrow results. `:v/error/d` then `:v/2024/d` keeps only lines containing both `error` and `2024`.

### Combined

`:v/error.*2024\|2024.*error/d` achieves the same result with a single regex alternation, matching lines with both terms in either order.

## With Marks

### Mark Matching

`:v/pattern/mark a` sets mark `a` on non-matching lines. Only the last non-matching line retains the mark since each execution overwrites it.
