# Flash Navigation

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

Quick jump to any visible location using label characters.

## Overview

Flash labels visible words/positions with 1-2 character labels. Typing the label jumps the cursor there instantly.

## Activation

| Key | Mode | Description |
|---|---|---|
| `s{char}{char}` | Normal | Flash search for 2-char sequence |
| `S{char}{char}` | Normal | Flash search backward |

## How It Works

1. User presses `s` and types two characters.
2. All visible matches are highlighted with unique labels (e.g., `a`, `b`, `c`, ...).
3. User types the label character to jump to that match.

## Label Characters

Labels are drawn from `asdfghjklqwertyuiopzxcvbnm`. Single-character labels are used first; if more matches exist, two-character labels are used.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `flash.labels` | `"asdfghjklqwertyuiopzxcvbnm"` | Characters used for labels |
| `flash.search_mode` | `"fuzzy"` | Matching mode |

## Related

- Navigation: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)
- Search: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
