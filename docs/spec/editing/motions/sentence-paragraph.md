# Sentence and Paragraph Motions

Navigate by text structure.

## Overview

Move by sentences and paragraphs
for prose and code editing.

## Sentence Motions

### Commands

| Motion | Description |
|--------|-------------|
| `)` | Next sentence start |
| `(` | Previous sentence start |

### Definition

A sentence ends at `.`, `!`, `?` 
followed by end of line or spaces.

### Example


## Sentence Rules

### End Characters


### Followed By

- End of line
- Space, tab
- `)`, `]`, `"`, `'`

### Example Ends


## Paragraph Motions

### Commands

| Motion | Description |
|--------|-------------|
| `}` | Next paragraph start |
| `{` | Previous paragraph start |

### Definition

Paragraphs are separated by
blank lines.

### Example


## Paragraph Boundaries

### Separators

- Blank lines
- Form feed characters
- Certain section macros

### Code Context

In code, blank lines separate
"paragraphs" of code blocks.


## Text Objects

### Sentence Objects

| Object | Description |
|--------|-------------|
| `is` | Inner sentence |
| `as` | A sentence (with space) |

### Paragraph Objects

| Object | Description |
|--------|-------------|
| `ip` | Inner paragraph |
| `ap` | A paragraph (with blanks) |

## Operator Examples

### Delete Sentence


### Delete Paragraph


## Code Applications

### Function Navigation

`{` and `}` jump between functions
separated by blank lines.

### Block Navigation


`}` jumps to the blank line.

## Format Paragraph

### gq Operator


### Text Width


## Section Motions

### Commands

| Motion | Description |
|--------|-------------|
| `]]` | Next section start |
| `[[` | Previous section start |
| `][` | Next section end |
| `[]` | Previous section end |

### Definition

Sections start with `{` at
beginning of line (C functions).

## Prose Editing

### Writing Workflow

