# Join and Split Operations

Line joining and splitting.

## Overview

Join multiple lines together or split
lines at specific positions.

## Join Operations

### Basic Join

| Key | Action |
|-----|--------|
| `J` | Join with space |
| `gJ` | Join without space |

### Join with Space

Before:

After `J`:

### Join Without Space

Before:

After `gJ`:

## Join with Count

### Multiple Lines


### Example

Before:

After `3J`:

## Join Command

### Range Join


### Examples


## Join Options

### Preserve Indent


### Remove Leading


## Split Operations

### Manual Split


### At Cursor

Before:

Cursor on space, `r<CR>`:

## Split Command

### Split on Pattern


### Examples


## Split at Width

### Hard Wrap


### Set Width


### Example

Before (textwidth=20):

After `gqq`:

## Split Sentence

### Each Sentence on Line


## Split Arguments

### Function Arguments

Before:

After splitting:

### Command

