# Jumplist

Global navigation history for position-based movement.

## Overview

The jumplist tracks cursor positions across all buffers, enabling quick navigation back to previously visited locations.

## Navigation

| Key | Action |
|-----|--------|
| `Ctrl-O` | Jump to older position |
| `Ctrl-I` | Jump to newer position |
| `Tab` | Same as `Ctrl-I` (in some configs) |

## How It Works


## What Creates Jump Entries

Entries added for:
- Search commands (`/`, `?`, `n`, `N`, `*`, `#`)
- Line jumps (`G`, `gg`, `:123`)
- Mark jumps (`'a`, `` `a ``)
- Matching bracket (`%`)
- Paragraph/sentence jumps (`{`, `}`, `(`, `)`)
- Tag jumps (`:tag`, `Ctrl-]`)
- Buffer switches

NOT added for:
- Character movement (`h`, `j`, `k`, `l`)
- Word movement (`w`, `b`, `e`)
- Find character (`f`, `t`, `F`, `T`)
- Scrolling (`Ctrl-D`, `Ctrl-U`)

## Viewing the Jumplist


Output:

## Count with Jumps


## Jumplist vs Other Navigation

| Feature | Jumplist | Changelist | `''` |
|---------|----------|------------|------|
| Scope | Global | Per buffer | Buffer |
| Keys | `Ctrl-O/I` | `g;/g,` | `''` |
| Tracks | Navigation | Changes | Last position |
| Cross-buffer | Yes | No | No |

## Jumplist Limits


## Cross-Buffer Jumping

The jumplist seamlessly switches buffers:


## File Position Memory

Each file remembers its position in the jumplist:


## Clearing Jumplist


## Jumplist Branches

When you jump back and then make a new jump:


## Special Jump Commands

| Command | Action |
|---------|--------|
| `Ctrl-O` | Older position |
| `Ctrl-I` | Newer position |
| `:jumps` | Show jumplist |
| `:clearjumps` | Clear list |

## No-Jumplist Variants

Some commands don't add to jumplist:

| Normal | No-Jumplist |
|--------|-------------|
| `'a` | `g'a` |
| `` `a `` | `` g`a `` |
| `''` | `g''` |

## Configuration


## Persistence


## Picker Integration


Shows visual list with:
- File paths
- Line numbers
- Line previews

## API Reference

