# Inner Text Objects

Selecting text without delimiters.

## Overview

Inner text objects select content
INSIDE delimiters, excluding the
delimiters themselves.

## Word Objects

### Inner Word


### Behavior

Selects word characters only.
No surrounding whitespace.

### Example


## WORD Objects

### Inner WORD


### Behavior

Selects non-whitespace sequence.
No surrounding whitespace.

### Example


## Sentence Objects

### Inner Sentence


### Behavior

Sentence without trailing space.

### Sentence Definition

Ends with `.`, `!`, `?` followed
by whitespace or EOL.

### Example


## Paragraph Objects

### Inner Paragraph


### Behavior

Lines between blank lines.
Excludes the blank lines.

### Example


`dip` on Line 1 removes Line 1-2,
leaves blank line.

## Quote Objects

### Inner Double Quote


### Inner Single Quote


### Inner Backtick


### Example


## Bracket Objects

### Inner Parentheses


### Inner Brackets


### Inner Braces


### Inner Angle Brackets


### Example


## Nested Brackets

### Behavior

Finds matching pair containing cursor.

### Example


### Outer Level

Position cursor before inner:

## Tag Objects

### Inner Tag

