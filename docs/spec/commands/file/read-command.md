# Read Command

Inserting external content.

## Overview

The `:read` command inserts
file contents or command output.

## Basic Syntax


## Read File

### After Current Line


### At Position


## Read Command

### Shell Output


### With Arguments


## Positioning

### Insert After


### Insert Before

Use line 0:

## Options

### Encoding


### File Format


### Binary


## Range Read

### File Sections

Not directly supported.
Use shell:

## Common Uses

### Insert Template


### Date/Time


### Include Output


### Code Generation


## Read vs Edit

### Difference

| Command  | Effect              |
|----------|---------------------|
| `:e`     | Open new buffer     |
| `:r`     | Insert into current |

## Read Stdin

### From Pipe


## Current File

### Re-read Part


## Error Handling

### Missing File


### Command Error

Shows command's error output.

## Performance

### Large Files

`:r` loads entire file.
Consider:

## In Insert Mode

### Using Shortcuts


## Undo Integration

### Single Undo

Read creates one undo point.
`u` removes all inserted.

## Filtering

### Process While Reading

