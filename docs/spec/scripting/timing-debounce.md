# Timing and Debounce

Key timing, debounce, and throttle.

## Overview

Control timing behavior for
keybindings and event handlers.

## Timeout Configuration

### Key Sequence Timeout


### Behavior

- Wait `timeout` ms for next key in sequence
- Fall through to shorter match if timeout
- Terminal codes use faster `ttimeout`

## Key Sequence Timing

### Multi-Key Mappings


Typing `g` waits for next key.

### Timeout Fallback

If no second key within timeout:
- Execute single key mapping if exists
- Otherwise no action

## Debounce

### Definition

Delay execution until input stops.
Useful for search-as-you-type.

### Configuration


### Use Cases

| Feature | Debounce | Purpose |
|---------|----------|---------|
| Search | 150ms | Wait for typing |
| Completion | 50ms | Fast response |
| Diagnostics | 100ms | Reduce flicker |
| Save | 1000ms | Batch saves |

## Throttle

### Definition

Limit execution rate. Execute at most
once per interval.

### Configuration


### Use Cases

| Feature | Throttle | Purpose |
|---------|----------|---------|
| Rendering | 16ms | 60fps cap |
| Statusline | 100ms | Reduce updates |
| Scroll | 16ms | Smooth scroll |
| Resize | 50ms | Reduce redraws |

## Cursor Hold

### Configuration


### Event Trigger


## Auto-Save Timing

### Debounced Save


### Idle Save


## Completion Timing

### Trigger Delay


### Preview Delay


## Search Timing

### Live Search


### Finder


## LSP Timing

### Request Debounce


### Response Timeout


## Animation Timing

### Smooth Scroll


