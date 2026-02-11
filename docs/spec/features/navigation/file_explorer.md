# File Explorer (Built-in)

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

The explorer is a first-class window in the shared layout tree.

## Core Contract

| Requirement | Detail |
|---|---|
| Window identity | explorer is a normal `WindowId` leaf |
| Launch reachability | command and key triggers must produce visible behavior |
| Mixed navigation | `Ctrl-w` works across explorer, buffer, and terminal leaves |
| Deterministic refresh | file tree updates are explicit, reproducible, and bounded |

## Launch and Wiring

| Trigger | Required Path |
|---|---|
| `:Explorer` | command parser -> core action -> explorer leaf open/focus |
| `:ExplorerClose` | command parser -> close explorer leaf |
| `:ExplorerReveal` | command parser -> reveal active buffer path |
| `<leader>e` | mapping resolver -> explorer toggle |
| `<leader>E` | mapping resolver -> explorer reveal |

No trigger above may be marked complete unless reachable via live runtime path.

## Explorer State Model

| Field | Requirement |
|---|---|
| root path | normalized absolute project root |
| node index | deterministic node IDs and parent/child linkage |
| expansion set | stable expanded directory IDs |
| visible list | flattened view derived only from expansion set |
| selected index | clamped to visible range after every mutation |
| clipboard state | optional cut/copy payload with explicit source IDs |

## Navigation and Open Targets

| Key | Behavior |
|---|---|
| `j` / `k` | move selection down/up |
| `h` | collapse dir or move to parent |
| `l` | expand dir or open file |
| `Enter` / `o` | open in current window |
| `v` | open file in vertical split |
| `s` | open file in horizontal split |
| `t` | open file in new tab |
| `q` | close explorer window |

Open-target actions must preserve deterministic focus and explorer selection state.

## File Operations

| Key | Behavior |
|---|---|
| `a` | create file |
| `A` | create directory |
| `r` | rename |
| `d` | safe delete |
| `D` | force delete |
| `x` / `c` / `p` | cut / copy / paste |
| `y` / `Y` / `gy` | copy name / relative path / absolute path |

Mutations must route through filesystem service and then refresh explorer state.

## External Drift and Errors

| Scenario | Required Behavior |
|---|---|
| external create/rename/delete | refresh updates visible tree deterministically |
| reveal target missing | fallback to nearest existing ancestor |
| permission denied | operation fails with explicit diagnostics and no in-memory corruption |
| partial refresh failure | previous visible state remains valid |

## Rendering and Wrap Safety

| Rule | Requirement |
|---|---|
| on-screen guarantee | explorer rows stay within text area bounds |
| long labels | wrap to continuation rows without overflow |
| selection identity | continuation rows map back to one logical node |
| badge safety | git/diagnostic badges do not corrupt alignment |

## Mandatory Verification

| ID | Scenario |
|---|---|
| `EXP-01R` | `:Explorer` opens explorer via real command path |
| `EXP-02R` | `<leader>e` and `<leader>E` toggle/reveal wiring |
| `EXP-03R` | open selected file via current/horizontal/vertical targets |
| `EXP-04R` | `Ctrl-w` navigation across mixed window types |
| `EXP-05R` | long label wrapping with stable selection identity |
| `EXP-06R` | external FS drift refresh without focus corruption |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
