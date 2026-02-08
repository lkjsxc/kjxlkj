# Floating Windows

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Floating windows are overlay panes rendered above the tiled split layout. They are used for transient UI such as dialogs, tooltips, previews, and completion menus.

## Core model

| Property | Requirement |
|---|---|
| Layer | Floating windows MUST render above all tiled windows and below only other higher-z floats. |
| Independence | A floating window MUST NOT participate in the split tree. It has its own position and size. |
| Buffer binding | A floating window MUST be bound to exactly one buffer, which MAY be a scratch buffer. |

## Creating floating windows

| Method | Required behavior |
|---|---|
| API call | The `open_float` API MUST accept a buffer handle and a configuration table describing size, position, border, and focus. |
| `:FloatNew` | MUST create a floating window with a new empty buffer centered on the screen. |
| `:FloatCurrent` | MUST move the current buffer into a new centered floating window, closing its tiled window. |

## Positioning

Positions are specified as `anchor`, `row`, and `col` relative to an anchor point.

| Anchor | Meaning |
|---|---|
| `editor` | Position is relative to the top-left corner of the entire editor area. |
| `cursor` | Position is relative to the current cursor position. MUST recompute on cursor move if the float is tied to cursor. |
| `window` | Position is relative to the top-left corner of the parent tiled window. |

### Centering

When `center` is `true`, the float MUST be placed so its midpoint coincides with the midpoint of the editor area. Explicit `row` and `col` values MUST be ignored when `center` is active.

### Near-cursor placement

Tooltip and completion floats SHOULD appear directly below the cursor. If insufficient space exists below, the float MUST flip above the cursor. If insufficient space exists in either direction, horizontal clamping to the editor boundary MUST apply.

### Corner anchoring

Setting `anchor` to one of `NW`, `NE`, `SW`, `SE` MUST position the float so the named corner of the float aligns with the given `row`/`col` coordinate.

## Sizing

| Parameter | Requirement |
|---|---|
| `width`, `height` (integer) | MUST set absolute size in columns and rows. |
| `width`, `height` (float 0.0--1.0) | MUST be interpreted as a fraction of the editor area dimensions. |
| `min_width`, `min_height` | The float MUST NOT shrink below these values during terminal resize. |
| `max_width`, `max_height` | The float MUST NOT grow beyond these values. |

On terminal resize, floats using relative sizing MUST recompute dimensions. Floats that would exceed the terminal boundary MUST be clamped to remain fully visible.

## Border styles

| Style keyword | Appearance |
|---|---|
| `none` | No border; the float occupies only the content area. |
| `single` | Single-line box-drawing characters. |
| `double` | Double-line box-drawing characters. |
| `rounded` | Rounded corners with single-line sides. |
| `solid` | Solid block characters filling the border cells. |
| `shadow` | No drawn border on top/left; shadow characters on bottom/right edges. |

A custom border MAY be specified as an array of eight strings representing the eight border positions clockwise from top-left corner: top-left, top, top-right, right, bottom-right, bottom, bottom-left, left. Each string MUST be exactly one grapheme cluster or empty. The border MUST respect the `FloatBorder` highlight group.

## Title and footer

| Element | Requirement |
|---|---|
| Title text | A float MAY have a title string rendered inside the top border. |
| Title position | Title alignment MUST be one of `left`, `center`, `right`. Default is `left`. |
| Footer text | A float MAY have a footer string rendered inside the bottom border with the same alignment options. |
| Highlight | Title and footer MUST use the `FloatTitle` and `FloatFooter` highlight groups respectively. |

## Z-order

| Rule | Requirement |
|---|---|
| Default stacking | Floats MUST stack in creation order: newer floats render above older ones. |
| Explicit z-index | When a `zindex` integer is provided, floats MUST render in ascending z-index order. Ties MUST fall back to creation order. |
| Focus raise | Focusing a float SHOULD raise it to the top of same-z-index peers. This behavior MUST be disableable via `float.raise_on_focus`. |

## Focus behavior

| Rule | Requirement |
|---|---|
| Focusable flag | A float with `focusable = false` MUST be skipped by all focus-cycling commands (`Ctrl-W w`, `Ctrl-W W`). |
| Initial focus | When `enter = true` (default), creating a float MUST move focus into it. When `false`, focus MUST remain in the previously active window. |
| Return focus | Closing a focused float MUST return focus to the window that was active before the float was focused. |
| Mouse focus | Clicking inside a focusable float MUST focus it. Clicking outside MUST NOT close the float unless a close trigger is configured. |

## Close triggers

| Trigger | Requirement |
|---|---|
| `<Esc>` | Dialog and preview floats SHOULD close on `<Esc>` unless the float's buffer is in insert mode. |
| `q` | Non-editable floats SHOULD close on `q` in normal mode. |
| Focus loss | When `close_on_focus_loss` is `true`, the float MUST close when focus moves to any other window. |
| Cursor move | Tooltip floats SHOULD close when the cursor moves away from the anchor position. |
| Explicit close | `:close`, `:quit`, `Ctrl-W c`, and `Ctrl-W q` MUST close the focused float. |

## Float types

| Type | Characteristics |
|---|---|
| Dialog | Centered, bordered, focusable, closes on `<Esc>` or explicit action. Used for confirmations and input prompts. |
| Tooltip | Small, near-cursor, non-focusable, closes on cursor move. Used for hover info and signature help. |
| Preview | Medium-large, bordered, focusable, read-only buffer. Used for definition peek and documentation preview. |
| Completion menu | Near-cursor, non-focusable, navigated by `Ctrl-N`/`Ctrl-P` from the parent window. See completion spec for details. |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `float.border` | `"rounded"` | Default border style for new floats. |
| `float.winblend` | `0` | Transparency level 0--100 where 0 is opaque. Terminal MUST support this or ignore silently. |
| `float.raise_on_focus` | `true` | Whether focusing a float raises it above same-z peers. |
| `float.close_on_focus_loss` | `false` | Whether floats close when they lose focus. |

## Test requirements

| Test category | Minimum checks |
|---|---|
| Unit | sizing arithmetic (absolute, relative, clamp), anchor positioning, z-order sort |
| Integration | focus cycling skips non-focusable, close triggers, return-focus correctness |
| PTY E2E | create float, interact, close, verify underlying tiled layout is unaffected |

## Related

- Split windows: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Tab pages: [/docs/spec/features/window/tabs.md](/docs/spec/features/window/tabs.md)
- UI components: [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
