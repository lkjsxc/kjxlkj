# Tab Pages

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Tab pages are top-level workspace containers. A tab owns its own window layout and active window pointer.

## Core model

| Entity | Requirement |
|---|---|
| Tab page | Contains one or more windows and exactly one active window. |
| Tab list | Ordered and stable during a session unless explicitly reordered. |
| Active tab | Exactly one tab is active at any time. |

## Invariants

| Rule | Requirement |
|---|---|
| Minimum tab count | There MUST always be at least one tab page. |
| Focus safety | Switching tabs MUST preserve each tab's last active window. |
| Isolation | Window split operations affect only the current tab unless command explicitly targets others. |
| Determinism | Given the same command stream, tab order and active index MUST be identical. |

## Commands and keys

| Input | Required behavior |
|---|---|
| `:tabnew` | Create a new tab and focus it. |
| `:tabclose` | Close current tab; if closing last tab, refuse unless equivalent quit flow is requested. |
| `:tabonly` | Keep current tab, close others with unsaved-change safeguards. |
| `:tabnext` / `gt` | Move to next tab (count-aware and wrapping). |
| `:tabprev` / `gT` | Move to previous tab (count-aware and wrapping). |
| `:tabfirst` | Focus first tab. |
| `:tablast` | Focus last tab. |
| `{n}gt` | Focus tab number `n` (1-indexed); out-of-range MUST report error and keep state unchanged. |

## Tabline requirements

| Topic | Requirement |
|---|---|
| Visibility | Tabline MUST render whenever more than one tab exists. |
| Labels | Tab labels SHOULD include active buffer name and modified marker. |
| Active indicator | Active tab MUST be visually distinct. |
| Overflow | If tabs exceed width, deterministic truncation or scrolling MUST be applied. |

## Sessions and persistence

| Topic | Requirement |
|---|---|
| Session save | Tab order, active tab, and per-tab window layouts MUST be serializable. |
| Session restore | Restored tabs MUST preserve relative order and focused tab when possible. |
| Missing resources | If a buffer path cannot be restored, tab structure MUST still load with a visible diagnostic. |

## Test requirements

| Test category | Minimum checks |
|---|---|
| Unit/integration | tab create/close/next/prev, count prefixes, index bounds, last-tab safeguards |
| Headless E2E | cross-tab layout isolation and restore behavior |
| PTY E2E | interactive `:tabnew`, edit in two tabs, `:wq` persistence and expected content |

## Related

- Windows and splits: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Layout persistence: [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- Session commands: [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)
