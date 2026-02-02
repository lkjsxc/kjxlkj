# Lookahead and Lookbehind

Zero-width assertions for matching based on surrounding context.

## Overview

Lookaround assertions match a position, not text. They check what's ahead or behind without consuming characters.

| Type | Syntax | Meaning |
|------|--------|---------|
| Positive lookahead | `\(pattern\)\@=` | Followed by pattern |
| Negative lookahead | `\(pattern\)\@!` | Not followed by pattern |
| Positive lookbehind | `\(pattern\)\@<=` | Preceded by pattern |
| Negative lookbehind | `\(pattern\)\@<!` | Not preceded by pattern |

## Positive Lookahead

Match only if followed by pattern:


Matches: "foo" in "foobar"
Does not match: "foo" in "foobaz"

### Use Cases


## Negative Lookahead

Match only if NOT followed by pattern:


Matches: "foo" in "foobaz"
Does not match: "foo" in "foobar"

### Use Cases


## Positive Lookbehind

Match only if preceded by pattern:


Matches: "bar" in "foobar"
Does not match: "bar" in "bazbar"

### Use Cases


## Negative Lookbehind

Match only if NOT preceded by pattern:


Matches: "bar" in "bazbar"
Does not match: "bar" in "foobar"

### Use Cases


## Very Magic Syntax

| Normal | Very Magic (\v) |
|--------|-----------------|
| `\(pat\)\@=` | `(pat)@=` |
| `\(pat\)\@!` | `(pat)@!` |
| `\(pat\)\@<=` | `(pat)@<=` |
| `\(pat\)\@<!` | `(pat)@<!` |


## Using \zs and \ze Instead

Often simpler alternative:


### Comparison

| Task | Lookaround | \zs/\ze |
|------|------------|---------|
| Word before ( | `/\w\+\((\)\@=` | `/\w\+\ze(` |
| After $ | `/\(\$\)\@<=\w\+` | `/\$\zs\w\+` |
| Between " " | Complex | `/".\{-}"` |

## Combining Lookarounds

### Both Ahead and Behind


Matches: "bar" in "foobarbaz"

### Multiple Conditions


Matches: "include" not after #, before .h

## Practical Examples

### Match Function Name


### Match Variable Assignment


### Match Import Path


### Match in Comments


## Performance Considerations

| Type | Performance |
|------|-------------|
| Lookahead | Fast (linear) |
| Lookbehind | Slower (variable length) |
| Nested | Slow |
| With `*` | Very slow |

### Optimization Tips


## Fixed-Width Constraint

Lookbehind requires fixed-width pattern:

