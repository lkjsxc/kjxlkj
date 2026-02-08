# Indent Guides

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Visual lines showing indentation levels.

## Overview

Indent guides display thin vertical lines at each indentation level to help visualize code structure.

## Display

A `│` character is rendered at each `shiftwidth`-multiple column on lines that have at least that much indentation.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `indent_guides.enabled` | `true` | Enable indent guides |
| `indent_guides.char` | `│` | Character used for guide |

## Colors

| Highlight Group | Usage |
|---|---|
| `IndentGuide` | Normal indent guide |
| `IndentGuideCurrent` | Guide for the current scope level |

## Exclude Filetypes

| Setting | Default | Description |
|---|---|---|
| `indent_guides.exclude` | `["help", "terminal"]` | File types where guides are not shown |

## Related

- UI: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)
- Indentation: [/docs/spec/modes/insert/insert-indentation.md](/docs/spec/modes/insert/insert-indentation.md)
