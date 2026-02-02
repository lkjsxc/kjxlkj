# Snippet Support

Code snippets for rapid text insertion.

## Overview

Snippets are templates that expand into larger
code blocks with placeholder values.

## Defining Snippets

### Configuration


## Triggering Snippets

### Tab Expansion


### Explicit Command


## Placeholders

### Syntax

| Syntax | Description |
|--------|-------------|
| `$1` | First placeholder |
| `${1:default}` | With default |
| `$0` | Final cursor |

### Navigation

| Key | Action |
|-----|--------|
| `<Tab>` | Next placeholder |
| `<S-Tab>` | Previous placeholder |
| `<Esc>` | Exit snippet |

## Linked Placeholders

### Same Value


Editing `$1` updates all instances.

## Transformations

### Case Conversion


### Regex Replace


## Built-in Snippets

### Rust

| Prefix | Description |
|--------|-------------|
| `fn` | Function |
| `impl` | Implementation |
| `struct` | Struct |
| `enum` | Enum |
| `test` | Test function |

### JavaScript

| Prefix | Description |
|--------|-------------|
| `func` | Function |
| `arrow` | Arrow function |
| `class` | Class |
| `import` | Import statement |

## Custom Snippet Files

### Directory Structure


### Global Snippets

Apply to all file types.

## LSP Snippets

### From Language Server

LSP servers can provide snippets in completions.


## Variables

### Built-in

| Variable | Value |
|----------|-------|
| `$TM_FILENAME` | Current filename |
| `$TM_FILEPATH` | Full path |
| `$TM_LINE_NUMBER` | Line number |
| `$CURRENT_DATE` | Today's date |

### Example


## Configuration


## Best Practices

1. Use descriptive prefixes
2. Add descriptions
3. Keep snippets focused
4. Use placeholders effectively
