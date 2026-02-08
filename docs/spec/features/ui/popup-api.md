# Popup Window API

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Programmatic popup creation for completion menus, hover info, and custom UI elements.

## Popup model (normative)

A popup window is a floating overlay rendered on top of the main window layout.

| Field | Type | Description |
|---|---|---|
| `popup_id` | PopupId | Stable unique identifier |
| `content` | array of lines | Text content to display |
| `row` | integer | Top-left row position (editor-relative or cursor-relative) |
| `col` | integer | Top-left column position |
| `width` | integer | Content area width in cells |
| `height` | integer | Content area height in cells |
| `border` | BorderStyle | None, Single, Double, Rounded, or custom characters |
| `focusable` | boolean | Whether the popup can receive focus and key input |
| `zindex` | integer | Stacking order (higher = on top) |
| `highlight` | HighlightGroup | Background and foreground highlight for the popup |

## Positioning modes (normative)

| Mode | Description |
|---|---|
| Cursor-relative | Position is offset from the current cursor position. Used for completion and hover. |
| Window-relative | Position is offset from the focused window's top-left corner. |
| Editor-relative | Position is offset from the editor's top-left (0,0). Used for centered dialogs. |

When a popup would extend beyond the terminal bounds, it MUST be clamped or flipped:

| Overflow direction | Behavior |
|---|---|
| Right edge | Shift popup left to fit within terminal width |
| Bottom edge | Flip popup above the anchor point if space allows; otherwise clamp height |
| Left edge | Clamp to column 0 |
| Top edge | Clamp to row 0 |

## Border styles (normative)

| Style | Characters (top-left, top, top-right, right, bottom-right, bottom, bottom-left, left) |
|---|---|
| None | No border drawn |
| Single | `┌─┐│┘─└│` |
| Double | `╔═╗║╝═╚║` |
| Rounded | `╭─╮│╯─╰│` |

## Lifecycle (normative)

| Event | Behavior |
|---|---|
| Open | Popup is added to the render overlay list. If focusable, it receives focus. |
| Close | Popup is removed from the overlay list. Focus returns to the previous window. |
| Auto-close | Non-focusable popups close on any key press, cursor movement, or mode change. |
| Timeout | Optional: close after a configurable duration (e.g., notifications). |

## Rendering

Popups are rendered in the decoration overlay stage of the render pipeline, after the main window cell grids are computed. Popups overwrite cells in the final frame buffer at their positioned area.

| Rule | Requirement |
|---|---|
| Z-order | Multiple popups stack by `zindex`; highest is rendered last (on top). |
| CJK awareness | Wide characters at popup edges MUST be handled (padding cell if clipped). |
| Scrolling | If content exceeds popup height, a scroll indicator MUST be shown. Scrollable via `Ctrl-d`/`Ctrl-u` when focused. |

## Related

- Completion popup: [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md)
- Hover: [/docs/spec/features/lsp/hover.md](/docs/spec/features/lsp/hover.md)
- Notifications: [/docs/spec/features/ui/notifications.md](/docs/spec/features/ui/notifications.md)
- Render pipeline: [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
