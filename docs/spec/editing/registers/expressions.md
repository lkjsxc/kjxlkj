# Vim-Style Expressions

Expression evaluation and usage.

## Overview

Expressions enable dynamic computation
in commands, mappings, and registers.

## Expression Register

### Insert Expression Result


### Examples


## In Substitution

### Use Expression


### Examples


## In Mappings

### Expression Mapping


### Conditional Mapping


## Operators

### Arithmetic

| Operator | Description |
|----------|-------------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |

### String

| Operator | Description |
|----------|-------------|
| `.` | Concatenation |
| `=~` | Regex match |
| `!~` | Regex not match |
| `==` | Equal |
| `!=` | Not equal |

### Comparison

| Operator | Description |
|----------|-------------|
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |
| `==?` | Equal (case insensitive) |
| `==#` | Equal (case sensitive) |

### Logical

| Operator | Description |
|----------|-------------|
| `&&` | And |
| `\|\|` | Or |
| `!` | Not |

## Functions

### String Functions

| Function | Description |
|----------|-------------|
| `strlen(s)` | String length |
| `strpart(s,n,m)` | Substring |
| `substitute(s,p,r,f)` | Replace |
| `toupper(s)` | Uppercase |
| `tolower(s)` | Lowercase |
| `trim(s)` | Trim whitespace |

### List Functions

| Function | Description |
|----------|-------------|
| `len(list)` | List length |
| `get(list,idx)` | Get element |
| `add(list,item)` | Append |
| `remove(list,idx)` | Remove |
| `sort(list)` | Sort |
| `reverse(list)` | Reverse |

### Buffer Functions

| Function | Description |
|----------|-------------|
| `line(".")` | Current line |
| `col(".")` | Current column |
| `getline(n)` | Get line n |
| `bufname("%")` | Buffer name |
| `bufnr("%")` | Buffer number |

### File Functions

| Function | Description |
|----------|-------------|
| `expand("%")` | Current file |
| `expand("%:p")` | Full path |
| `expand("%:t")` | Tail (name) |
| `expand("%:r")` | Root (no ext) |
| `expand("%:e")` | Extension |

### Time Functions

| Function | Description |
|----------|-------------|
| `strftime(fmt)` | Format time |
| `localtime()` | Unix timestamp |

## Variables

### Special Variables

| Variable | Description |
|----------|-------------|
| `v:count` | Command count |
| `v:register` | Register name |
| `v:version` | Version number |
| `v:true` | Boolean true |
| `v:false` | Boolean false |
| `v:null` | Null value |

### Environment


### Options


## Conditionals

### Ternary


### If/Else


## Command Line

### Evaluate Expression

