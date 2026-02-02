# Register Commands

Ex commands for viewing and manipulating registers.

## Viewing Registers

### :registers / :reg

Display register contents:


### :display / :di

Alias for `:registers`:


## Output Format


## Pasting Registers

### :put

Paste register contents on a new line:


### Range with :put


## Setting Registers

### :let @{reg}

Set register contents directly:


### Appending


## Executing Registers

### :@{reg}

Execute register contents as ex commands:


### Example


## Register Operations

### Clear Register


### Clear All Named Registers


### Copy Between Registers


## Interactive Register Selection

In normal mode, type `"` then wait:


## Register in Substitution

Use register content in substitution:


## Register in Expressions


## Macros and Registers

Macros are stored in registers:


Edit macro:

## Configuration


## Keybindings


## API Reference

