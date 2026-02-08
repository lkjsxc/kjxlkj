# Tab Pages

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Tab pages are top-level workspace containers. Each tab owns its own window split tree and maintains a pointer to its active window.

## Core model

| Entity | Requirement |
|---|---|
| Tab page | Contains one or more windows and exactly one active window. |
| Tab list | Ordered and stable during a session unless explicitly reordered. |
| Active tab | Exactly one tab MUST be active at any time. |
| Buffer sharing | Buffers are global; the same buffer MAY appear in windows across different tabs. |

## Invariants

| Rule | Requirement |
|---|---|
| Minimum tab count | There MUST always be at least one tab page. |
| Focus safety | Switching tabs MUST preserve each tab's last active window. |
| Isolation | Window split operations affect only the current tab unless a command explicitly targets others. |
| Determinism | Given the same command stream, tab order and active index MUST be identical. |

## Commands and keys

| Input | Required behavior |
|---|---|
| `:tabnew [path]` | Create a new tab after the current tab and focus it. If `path` is given, open that file in the new tab's window. |
| `:tabclose` | Close current tab. If it is the last tab, MUST refuse unless `!` is appended or an equivalent quit flow is requested. Unsaved-change safeguards apply. |
| `:tabclose!` | Force-close current tab, discarding unsaved changes in windows unique to this tab. |
| `:tabonly` | Keep current tab, close all others. Unsaved-change safeguards apply to each closed tab. |
| `:tabnext` / `gt` | Move to next tab. Count-aware: `3gt` focuses tab 3. Wrapping MUST apply when at the last tab. |
| `:tabprev` / `gT` | Move to previous tab. Count-aware and wrapping. |
| `:tabfirst` | Focus the first tab. |
| `:tablast` | Focus the last tab. |
| `{n}gt` | Focus tab number `n` (1-indexed). Out-of-range MUST report an error and keep state unchanged. |
| `:tabmove {n}` | Move current tab to position `n` (0-indexed). `:tabmove 0` MUST make it first; `:tabmove $` MUST make it last. |
| `:tabmove +{n}` / `:tabmove -{n}` | Move current tab right or left by `n` positions. Clamping at boundaries MUST apply. |

## Tabline requirements

| Topic | Requirement |
|---|---|
| Visibility | The tabline MUST render whenever more than one tab exists. When only one tab exists, the tabline SHOULD be hidden unless `tabline.always_show` is `true`. |
| Labels | Tab labels SHOULD include the active buffer name and a modified marker (`[+]`). |
| Active indicator | The active tab MUST be visually distinct via the `TabLineSel` highlight group. |
| Inactive style | Inactive tabs MUST use the `TabLine` highlight group. |
| Fill | Empty space in the tabline MUST use the `TabLineFill` highlight group. |
| Overflow | If tabs exceed terminal width, deterministic truncation or horizontal scrolling MUST be applied so the active tab is always visible. |
| Close button | Each tab label SHOULD include a close affordance (e.g. `x`) that closes the tab on click. |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `tabline.always_show` | `false` | Show the tabline even when only one tab exists. |
| `tabline.max_label_width` | `30` | Maximum characters for a tab label before truncation. |
| `tabline.show_index` | `false` | Prefix each tab label with its 1-based index number. |

## Sessions and persistence

| Topic | Requirement |
|---|---|
| Session save | Tab order, active tab, and per-tab window layouts MUST be serializable. |
| Session restore | Restored tabs MUST preserve relative order and focused tab when possible. |
| Missing resources | If a buffer path cannot be restored, the tab structure MUST still load with a visible diagnostic. |

## Test requirements

| Test category | Minimum checks |
|---|---|
| Unit | tab create/close/next/prev, count prefixes, index bounds, last-tab safeguards, tab reorder |
| Integration | cross-tab layout isolation, buffer sharing across tabs, restore behavior |
| PTY E2E | interactive `:tabnew`, edit in two tabs, `:wq` persistence and expected content |

## Related

- Windows and splits: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Floating windows: [/docs/spec/features/window/floating-windows.md](/docs/spec/features/window/floating-windows.md)
- Layout persistence: [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- Session commands: [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)
