# Advanced Buffer Management

Deep buffer control and strategies.

## Overview

Advanced techniques for managing
multiple buffers efficiently.

## Buffer States

### State Types

| State | Description |
|-------|-------------|
| Active | Displayed in window |
| Hidden | Loaded, not displayed |
| Inactive | Not loaded |
| Modified | Has unsaved changes |
| Readonly | Cannot modify |

### Check State


## Buffer Types

### Special Buffers

| Type | Purpose |
|------|---------|
| Normal | Regular file editing |
| Help | Documentation |
| Quickfix | Error/search lists |
| Terminal | Terminal emulator |
| Prompt | Input prompts |
| Scratch | Temporary content |
| Nofile | No associated file |

### Set Type


## Buffer Lifecycle

### Create


### Delete


### Unload


## Buffer Navigation

### Direct Jump


### Relative


### Alternate


## Buffer Lists

### Standard List


### Flags

| Flag | Meaning |
|------|---------|
| `%` | Current buffer |
| `#` | Alternate buffer |
| `a` | Active (loaded, displayed) |
| `h` | Hidden (loaded, not displayed) |
| `+` | Modified |
| `-` | Readonly |
| `=` | Readonly |
| `u` | Unlisted |

## Buffer Filtering

### By Modified


### By Type


## Buffer Commands

### Apply to All


### Apply with Pattern


## Buffer Variables

### Local Variables


### Check Variable


## Buffer Options

### Local Options


### Buffer-Local Only

| Option | Description |
|--------|-------------|
| `buftype` | Buffer type |
| `bufhidden` | Hidden behavior |
| `buflisted` | In buffer list |
| `swapfile` | Use swap file |
| `modified` | Has changes |

## Buffer Hidden Behavior

### bufhidden Options

| Value | Behavior |
|-------|----------|
| `hide` | Hide when abandoned |
| `unload` | Unload when hidden |
| `delete` | Delete when hidden |
| `wipe` | Wipe when hidden |

### Configuration

