# Path Expansion

Filename modifiers and wildcards.

## Overview

Expand and modify file paths
using special tokens.

## Current File (%)

### Basic


### In Commands


## Alternate File (#)

### Previous Buffer


### Toggle


## Modifiers

### Overview

| Modifier | Result              |
|----------|---------------------|
| `:p`     | Full path           |
| `:h`     | Head (directory)    |
| `:t`     | Tail (filename)     |
| `:r`     | Root (no extension) |
| `:e`     | Extension only      |
| `:~`     | Home relative       |
| `:.`     | CWD relative        |

### Examples

For file `/home/user/src/main.rs`:

| Expression | Result                  |
|------------|-------------------------|
| `%`        | `main.rs`               |
| `%:p`      | `/home/user/src/main.rs`|
| `%:h`      | `src` (or full dir)     |
| `%:p:h`    | `/home/user/src`        |
| `%:t`      | `main.rs`               |
| `%:r`      | `main`                  |
| `%:e`      | `rs`                    |
| `%:~`      | `~/src/main.rs`         |

## Chaining Modifiers

### Multiple


### Order Matters

Apply left to right.

## Substitute

### Syntax


### Examples


## Shell Escape

### For Shell


### Use Case


## Expand Function

### Usage


### Special Tokens

| Token       | Meaning              |
|-------------|---------------------|
| `<cfile>`   | File under cursor   |
| `<cword>`   | Word under cursor   |
| `<cWORD>`   | WORD under cursor   |
| `<sfile>`   | Sourced file        |
| `<afile>`   | Autocmd file        |

## Wildcards

### Glob Patterns

| Pattern | Matches             |
|---------|---------------------|
| `*`     | Any chars (no /)    |
| `**`    | Any chars (with /)  |
| `?`     | Single character    |
| `[abc]` | Character class     |

### Examples


## Glob Function

### Get Matches


### As List


## Globpath

### Search in Path


## Environment Variables

### Syntax


### Expand


## Expression Register

### In Commands

