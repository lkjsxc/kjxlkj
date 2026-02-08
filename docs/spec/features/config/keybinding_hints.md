# Keybinding Hints

which-key style popup for discovering key sequences.

## Overview

When a key prefix is pressed and no immediate command
matches, a floating panel appears showing all possible
continuations. This helps users discover and learn
keybindings without leaving their editing flow.

## Trigger Conditions

| Trigger | Description |
|---------|-------------|
| Leader key | Show all leader-prefixed commands |
| g prefix | Show all g-prefixed commands |
| z prefix | Show all fold/scroll commands |
| ] / [ prefix | Show all bracket motions |
| Window prefix | After `Ctrl-W`, show split commands |
| Operator pending | After d/c/y, show motion hints |
| Custom prefix | Any user-defined prefix key |

## Timing

### Display Delay

Hints appear after `whichkey_timeout` milliseconds
(default: 500). This avoids flashing for experienced
users who type sequences quickly.

### Timeout Behavior

If `timeoutlen` expires before a continuation key,
the prefix itself is processed (or discarded if no
solo meaning). The hint popup closes immediately.

## Data Model

### KeymapTrie

All keybindings are stored in a trie structure indexed
by mode. Each node contains:

| Field | Type | Description |
|-------|------|-------------|
| `key` | `Key` | The key at this node |
| `command` | `Option<CommandId>` | Command if this is a leaf |
| `children` | `BTreeMap<Key, Node>` | Sub-prefix continuations |
| `description` | `String` | Human-readable label |

### HintEntry

Each entry in the popup displays:

| Field | Description |
|-------|-------------|
| `key` | Next key to press |
| `label` | Short description of the command |
| `group` | Category for visual grouping |
| `is_prefix` | Whether this leads to more keys |

## Display Layout

### Panel Structure

The popup appears as a floating window anchored to
the bottom of the screen (above the status line).

### Organization

Entries are organized in columns, sorted by key.
Groups are visually separated with headers.
Prefix entries show `+` to indicate sub-menus.

### Example Display

After pressing leader key:
```text
f  +file     b  +buffer     w  +window
ff  find      bn  next       ws  split
fs  save      bp  prev       wv  vsplit
fq  quit      bd  delete     wq  close
```

## Filtering

### Type-to-Filter

While the hint popup is visible, typing narrows the
displayed entries. Only matching entries remain.

### Matching

Filter matches against both key sequences and
description text, case-insensitively.

## Context Awareness

### Mode-Dependent

Hints reflect the current mode. Normal mode shows
normal mappings; visual mode shows visual mappings.

### View-Dependent

If the focused view is the file explorer, hints
include file-explorer-specific bindings. Similarly
for the terminal window.

## Configuration

### Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `whichkey.enabled` | bool | true | Enable hint popup |
| `whichkey.timeout` | int | 500 | Delay in ms |
| `whichkey.max_columns` | int | 5 | Column count |
| `whichkey.show_icons` | bool | false | Show category icons |
| `whichkey.sort` | string | "key" | Sort order |

### Custom Labels

Users can override the description for any mapping
in the configuration file.

## Acceptance Criteria

- Hints MUST never block input processing
- Hints MUST derive data from in-memory keymaps only
- Hint popup MUST close immediately on any key press
- The UI MUST distinguish prefixes from leaf commands
- Display MUST not delay rendering of the editor
- Filtering MUST be responsive (no async delay)
