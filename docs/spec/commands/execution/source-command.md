# Source Command

Execute commands from a file.

## Overview

The `:source` command reads
and executes Ex commands
from a file.

## Basic Syntax


## Simple Examples

### Source Config


### Current File


## File Resolution

### Absolute Path


### Relative Path


### Home Expansion


## Runtime Command

### Search Runtime


### Difference

| Command     | Behavior              |
|-------------|----------------------|
| `:source`   | Exact file path      |
| `:runtime`  | Search runtimepath   |

## Script Types

### Vim Script


### Commands File


## Configuration Loading

### Init File

On startup, sources:

### Project Local


## Source Scope

### Variables

Variables defined in sourced
file are global by default.

### Local Variables


## Error Handling

### On Error

Sourcing stops on first error.

### Continue on Error


### Check Exists


## Dynamic Sourcing

### With Execute


### Variable Path


## Common Use Cases

### Reload Config


### Apply Settings


### Run Script


## Modular Configuration

### Main Config


### Conditional


## Arguments

### Passing Data

Cannot pass arguments directly.
Use global variables:
