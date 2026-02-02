# After-Directory Configuration

Configuration loaded after defaults.

## Overview

The "after" directory contains configuration that
loads after built-in defaults, allowing overrides.

## Directory Structure


## Loading Order

1. Built-in defaults
2. User config (~/.config/kjxlkj/)
3. After config (~/.config/kjxlkj/after/)

## Use Cases

### Override Syntax

Modify built-in highlighting:


### Extend Filetype

Add to existing filetype config:


### Theme Tweaks

Override theme colors:


## Syntax Overrides

### Example

Override Rust function highlighting:


### Behavior

After queries merged with defaults.

## Filetype Overrides

### Example


### Merging

After settings merged, overriding conflicts.

## Theme Overrides

### Example


## Project After Directory

### Location


### Loading Order

1. Built-in
2. User config
3. User after
4. Project config
5. Project after

## Benefits

### Non-Destructive

Original config unchanged.

### Targeted Changes

Only override specific items.

### Easy Maintenance

Keep overrides separate.

## Configuration


## Tips

1. Use for small tweaks
2. Keep main config clean
3. Document your overrides
4. Check loading with `:verbose set option?`

## Debugging

### Show Load Order


Shows where setting came from.

### List Loaded Files

