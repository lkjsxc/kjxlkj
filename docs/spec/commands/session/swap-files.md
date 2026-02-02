# Swap File Management

Crash recovery and swap files.

## Overview

Swap files protect against
data loss from crashes.

## Purpose

### Recovery

- Save editing state
- Recover after crash
- Prevent dual editing

## Swap File Location

### Default


### Configuration


### Same Directory


## Enable/Disable

### Global


### Per Buffer


## Swap File Names

### Format


### In Swap Directory


## Swap Detection

### On Open


### Options

| Choice | Action                |
|--------|-----------------------|
| O      | Open Read-Only        |
| E      | Edit anyway           |
| R      | Recover               |
| D      | Delete and edit       |
| Q      | Quit                  |
| A      | Abort                 |

## Recovery

### Recover File


### After Recovery


### Delete Swap

After successful recovery:

## Recovery Process

### Steps

1. Open shows swap warning
2. Choose `R` to recover
3. Review recovered content
4. `:w` to save
5. Delete swap file

### Diff Original


## Automatic Cleanup

### On Normal Exit

Swap files deleted on `:wq`.

### Stale Swaps

From crashed sessions remain.

## List Swap Files

### Find All


### Check Current


## Swap Write Timing

### Updatetime


### Updatecount


## Disk Space

### Monitor

Large files = large swap files.

### Cleanup Stale


Caution: Check for recovery first.

## Multiple Editors

### Prevention

Swap files indicate editing.

### Detection

Second editor sees swap warning.

### Same Machine

Works via swap presence.

### Remote
