# Substitute Special Characters

Special sequences in patterns and replacements.

## Overview

Special characters provide
advanced replacement features.

## Pattern Metacharacters

### Basic

| Char   | Meaning              |
|--------|---------------------|
| `.`    | Any character       |
| `*`    | Zero or more        |
| `\+`   | One or more         |
| `\?`   | Zero or one         |
| `^`    | Start of line       |
| `$`    | End of line         |
| `\|`   | Alternation         |

### Character Classes

| Pattern   | Matches             |
|-----------|---------------------|
| `[abc]`   | a, b, or c          |
| `[^abc]`  | Not a, b, or c      |
| `[a-z]`   | Lowercase letters   |
| `\w`      | Word character      |
| `\W`      | Non-word character  |
| `\s`      | Whitespace          |
| `\S`      | Non-whitespace      |
| `\d`      | Digit               |
| `\D`      | Non-digit           |

## Replacement Specials

### Matched Text

| Sequence | Inserts              |
|----------|---------------------|
| `&`      | Entire match        |
| `\0`     | Entire match        |
| `~`      | Previous replacement|

### Captured Groups

| Sequence | Inserts              |
|----------|---------------------|
| `\1`     | First group         |
| `\2`     | Second group        |
| `\3`-`\9`| Groups 3-9          |

### Example

"John Smith" → "Smith, John"

## Case Modifiers

### Sequences

| Sequence | Effect              |
|----------|---------------------|
| `\u`     | Next char uppercase |
| `\U`     | Following uppercase |
| `\l`     | Next char lowercase |
| `\L`     | Following lowercase |
| `\e`     | End case change     |
| `\E`     | End case change     |

### Examples


## Newline Characters

### In Pattern

| Sequence | Matches             |
|----------|---------------------|
| `\n`     | Newline             |
| `\r`     | Carriage return     |
| `\_s`    | Whitespace + newline|
| `\_^`    | Start of line       |
| `\_.`    | Any including newline|

### In Replacement

| Sequence | Inserts             |
|----------|---------------------|
| `\r`     | Newline (split line)|
| `\n`     | NUL character       |

### Example


## Special Atoms

### Boundaries

| Atom    | Matches             |
|---------|---------------------|
| `\<`    | Start of word       |
| `\>`    | End of word         |
| `\zs`   | Start match here    |
| `\ze`   | End match here      |

### Example


## Literal Characters

### Escaping


### Very Nomagic


## Backreferences

### Capture Groups


### Named Groups (if supported)


## Submatch Modifiers

### Apply to Groups


## Tab and Space

### Tab Character


### Whitespace


## Ampersand (&)

### Whole Match


### Literal Ampersand

