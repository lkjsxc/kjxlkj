# Hooks and Events

Event system for extensibility.

## Overview

Hooks allow custom behavior at defined points
in the editor lifecycle.

## Hook Types

### Pre-Hooks

Execute before an action:


### Post-Hooks

Execute after an action:


## Built-in Events

### Lifecycle Events

| Event | Description |
|-------|-------------|
| `startup` | Editor starting |
| `shutdown` | Editor closing |
| `idle` | No activity |
| `focus_gained` | Window focused |
| `focus_lost` | Window unfocused |

### Buffer Events

| Event | Description |
|-------|-------------|
| `buffer_open` | Buffer opened |
| `buffer_close` | Buffer closed |
| `buffer_change` | Content changed |
| `buffer_save` | Buffer saved |
| `buffer_reload` | Buffer reloaded |

### Mode Events

| Event | Description |
|-------|-------------|
| `mode_change` | Mode switched |
| `insert_enter` | Enter insert |
| `insert_leave` | Leave insert |
| `visual_enter` | Enter visual |
| `visual_leave` | Leave visual |

### Cursor Events

| Event | Description |
|-------|-------------|
| `cursor_move` | Cursor moved |
| `cursor_hold` | Cursor idle |
| `selection_change` | Selection changed |

## Event Handlers

### Configuration


### Multiple Handlers


### Conditional Handlers


## Event Data

Events pass context to handlers:

### Buffer Events


### Mode Events


### Cursor Events


## Custom Events

### Emitting


### Handling


## Debouncing

Prevent excessive triggers:


## Throttling

Rate limit handlers:


## Priority

Control execution order:


## Error Handling

### Continue on Error


