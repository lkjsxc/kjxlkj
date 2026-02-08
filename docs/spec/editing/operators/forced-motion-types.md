# Forced Motion Types

Detailed reference for motion type forcing.

## Overview

This document supplements operator-modifiers.md with detailed rules for how forcing interacts with exclusive/inclusive semantics.

## Exclusive vs Inclusive (normative)

| Category | Motions | Meaning |
|---|---|---|
| Inclusive | `e`, `ge`, `E`, `gE`, `$`, `g_`, `f{c}`, `F{c}`, `` ` ``mark, `%` | Destination character IS part of the operation |
| Exclusive | `w`, `W`, `b`, `B`, `t{c}`, `T{c}`, `(`, `)`, `{`, `}`, `/`, `?`, `n`, `N` | Destination character is NOT part of the operation |

## Exclusive-to-Inclusive Adjustment

When an exclusive characterwise motion results in the cursor being at column 0 of a line that is beyond the starting line, the operation is adjusted: the end moves back one character (to column 0 minus 1, i.e., end of previous line), and the motion becomes inclusive. This prevents unexpected empty line inclusion.

## The `v` Toggle

When `v` is placed between an operator and an exclusive characterwise motion, the motion becomes inclusive (and vice versa). This toggles the exclusive/inclusive behavior only for characterwise motions.

## Inherently Linewise Motions

| Motion | Description |
|---|---|
| `j`, `k` | Line up/down |
| `G`, `gg` | Go to line |
| `H`, `M`, `L` | Screen line |
| `]]`, `[[`, `][`, `[]` | Section |
| `_` | Current line |

## Inherently Characterwise Motions

| Motion | Description |
|---|---|
| `h`, `l` | Character left/right |
| `w`, `e`, `b` | Word motions |
| `f`, `t`, `F`, `T` | Character find |
| `$`, `0`, `^` | Line position |
| `/`, `?`, `n`, `N` | Search |

## Related

- Operator modifiers: [/docs/spec/editing/operators/operator-modifiers.md](/docs/spec/editing/operators/operator-modifiers.md)
- Exclusive/inclusive: [/docs/spec/editing/operators/exclusive-inclusive.md](/docs/spec/editing/operators/exclusive-inclusive.md)
