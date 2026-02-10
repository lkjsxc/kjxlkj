# Cursor Customization

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Cursor appearance and visibility requirements.

## Scope

- focused cursor/caret MUST remain visible at all times
- cursor styling MUST be deterministic from snapshot data
- cursor rendering MUST stay correct with wide graphemes and wrapped rows

## Mode Defaults

| Mode | Default Shape | Requirement |
|---|---|---|
| Normal | `block` | distinct from selection/search highlights |
| Insert | `bar` | visible at end-inclusive insertion points |
| Visual | `hollow` | selection remains visible beneath cursor |
| Replace | `underline` | visible while overwriting |
| Command | `bar` | tracks command-line caret |

## Rendering Priority (normative)

Rendering order MUST be:

1. content cells
2. selection/search/diagnostic highlights
3. cursor overlay (highest priority)

Later draws MUST NOT erase cursor visibility.

## Wide Grapheme Cursor Rules

| Rule | Requirement |
|---|---|
| Atomic highlight | if cursor targets width-2 grapheme, both cells are highlighted |
| No continuation targeting | cursor MUST NOT target continuation cell directly |
| Wrap safety | cursor display never appears split across two rows |

## Terminal Cursor vs Drawn Cursor

| Technique | Requirement |
|---|---|
| Native terminal cursor | SHOULD be configured when terminal supports it |
| Drawn cursor fallback | MUST remain visible even when terminal cursor shape is unsupported |

If configured cursor style is low contrast, fallback MUST switch to high-contrast invert or explicit outline.

## Multi-Window Focus Rule

Only the focused window may render primary cursor style.

- unfocused windows MAY render subtle caret markers
- secondary cursors (multicursor) MUST be visually distinct and never drive viewport follow

## Mandatory Verification

| ID | Scenario |
|---|---|
| `CUR-07R` | cursor remains visible through mode churn and redraw cycles |
| `CUR-08R` | width-2 grapheme cursor highlights both cells |
| `CUR-09R` | no cursor appears on continuation cell |
| `CUR-10R` | wrap boundary preserves cursor visibility |
| `CUR-11R` | mixed-window focus changes keep one primary cursor |

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
