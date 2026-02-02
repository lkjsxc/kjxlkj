# Substitute Flags

Modifiers for the :s command.

## Overview

Flags modify how substitution
behaves and what gets replaced.

## Flag Summary

| Flag | Name        | Effect                    |
|------|------------|---------------------------|
| `g`  | global     | All occurrences on line   |
| `c`  | confirm    | Prompt for each           |
| `i`  | ignorecase | Case insensitive match    |
| `I`  | noignorecase | Case sensitive match    |
| `n`  | count      | Count matches, no replace |
| `e`  | error      | Suppress "not found"      |
| `&`  | flags      | Reuse previous flags      |
| `p`  | print      | Print changed lines       |
| `#`  | number     | Print with line numbers   |
| `l`  | list       | Print like :list          |

## Global Flag (g)

### Without g


Line: `aaa` → `baa`

### With g


Line: `aaa` → `bbb`

### Why Needed

Historical vim behavior.
Must opt-in to all matches.

## Confirm Flag (c)

### Interactive Mode


### Prompt Display


### Response Keys

| Key  | Action                |
|------|----------------------|
| `y`  | Yes, replace this    |
| `n`  | No, skip this        |
| `a`  | All remaining        |
| `q`  | Quit now             |
| `l`  | Last (replace, quit) |
| `^E` | Scroll up            |
| `^Y` | Scroll down          |
| `^L` | Redraw screen        |

### Practical Use


## Case Flags (i/I)

### Case Insensitive


Matches: word, WORD, Word

### Case Sensitive


Only matches: word

### Override Settings

Ignores `ignorecase` option.

### Smart Case

With both pattern cases:

## Count Flag (n)

### Preview Changes


Output:

### No Modification

File not changed.
Use before actual replace.

### With Range


## Error Suppression (e)

### Default Behavior


### With e Flag


### Macro/Script Use

Prevents script abortion:

## Flags Reuse (&)

### Repeat with Flags


### Combine with New


## Print Flags (p/#/l)

### Print Changed Lines


### With Line Numbers


### List Format


## Flag Combinations

### Common Combos

