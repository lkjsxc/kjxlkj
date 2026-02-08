# Linewise vs Characterwise

Motion scope types and their effect on operators.

## Overview

Every motion has an inherent scope: characterwise, linewise, or blockwise. This scope determines how much text an operator affects.

## Characterwise Motions (normative)

Operate on individual characters within or across lines (partial lines affected).

| Motion | Description |
|---|---|
| `h`, `l` | Left/right by character |
| `w`, `e`, `b`, `W`, `E`, `B` | Word motions |
| `f{c}`, `t{c}`, `F{c}`, `T{c}` | Character find on current line |
| `0`, `^`, `$` | Line positions |
| `/pattern`, `?pattern` | Search |
| `%` | Matching bracket |
| `` `{mark} `` | Go to mark (exact position) |

## Linewise Motions (normative)

Operate on entire lines (full lines always affected, regardless of cursor column).

| Motion | Description |
|---|---|
| `j`, `k` | Up/down line |
| `{`, `}` | Paragraph boundary |
| `[[`, `]]`, `[]`, `][` | Section boundary |
| `'{mark}` | Go to mark (line) |
| `H`, `M`, `L` | Screen position |
| `gg`, `G` | File position |

## Forced Scope (normative)

The scope of any pending operator can be overridden by pressing `v`, `V`, or `Ctrl-v` between the operator and the motion:

| Override | Effect |
|---|---|
| `v` | Force characterwise (if the motion would be linewise) or toggle inclusiveness |
| `V` | Force linewise (if the motion would be characterwise) |
| `Ctrl-v` | Force blockwise |

Example: `dV/pattern` deletes linewise to the pattern match, even though `/pattern` is normally characterwise.

## Impact on Yank and Put (normative)

The register stores the scope (characterwise, linewise, or blockwise) alongside the text:

| Scope | Put with `p` | Put with `P` |
|---|---|---|
| Characterwise | After cursor on current line | Before cursor on current line |
| Linewise | Below current line | Above current line |
| Blockwise | At cursor column, each line on successive rows | Same, before cursor column |

## Text Objects and Scope

| Object | Typical scope |
|---|---|
| `aw`, `iw`, `aW`, `iW` | Characterwise |
| `a"`, `i"`, `a'`, `i'` | Characterwise |
| `ab`, `ib`, `a{`, `i{` | Characterwise (unless selecting whole lines) |
| `ap`, `ip` | Linewise |
| `at`, `it` | Characterwise |

When a text object spans from the start of a line to the end of another line, it may be promoted to linewise.

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Visual mode: [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)
