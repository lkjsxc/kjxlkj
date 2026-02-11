# File Explorer (Built-in)

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

The explorer is a first-class window in the shared layout tree.

## Core Contract

| Requirement | Detail |
|---|---|
| Window identity | explorer is a normal `WindowId` leaf in the split tree |
| Launch reachability | command and key routes must produce visible pane changes |
| Mixed navigation | `Ctrl-w` navigation works across buffer, explorer, and terminal panes |
| Deterministic refresh | file-tree updates are explicit and reproducible |
| Screen-observable closure | feature is closed only with frame-level assertions |

## Launch and Routing

| Trigger | Required Path | Visible Result |
|---|---|---|
| `:Explorer` | command parser -> action -> explorer open/focus | explorer pane appears and is focused |
| `:ExplorerClose` | command parser -> close explorer leaf | explorer pane disappears |
| `:ExplorerReveal` | command parser -> reveal active file | target node is visible and selected |
| `<leader>e` | mapping resolver -> explorer toggle | pane toggles open/closed |
| `<leader>E` | mapping resolver -> reveal action | pane opens and highlights active file |

## State Model

| Field | Requirement |
|---|---|
| root path | normalized absolute workspace root |
| node IDs | deterministic parent/child relationship |
| expansion set | stable expanded directory IDs |
| visible rows | flattened from tree + expansion set only |
| selected row | clamped after every mutation |
| pending operation | explicit create/rename/delete state |

## Navigation and Open Targets

| Key | Behavior |
|---|---|
| `j` / `k` | move selection down/up |
| `h` | collapse directory or move to parent |
| `l` | expand directory or open file |
| `Enter` / `o` | open in current target pane |
| `v` | open file in vertical split |
| `s` | open file in horizontal split |
| `t` | open file in new tab |
| `q` | close explorer pane |

Open-target actions must preserve deterministic focus and selection identity.

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

File mutations route through filesystem service, then refresh explorer state.

## Error and Drift Handling

| Scenario | Required Behavior |
|---|---|
| external create/rename/delete | refresh updates visible rows deterministically |
| reveal target missing | fallback to nearest existing ancestor |
| permission denied | explicit error with no in-memory tree corruption |
| partial refresh failure | previous valid tree state remains usable |

## Rendering Rules

| Rule | Requirement |
|---|---|
| on-screen guarantee | rows stay inside pane bounds |
| long labels | wrap without overflow and keep node identity |
| selection marker | exactly one selected node row is visible |
| badge safety | git and diagnostics badges preserve alignment |

## Mandatory E2E Verification

| ID | Scenario | Required Assertions |
|---|---|---|
| `EXP-01R` | run `:Explorer` | pane appears and focus marker moves to explorer |
| `EXP-02R` | run `<leader>e` and `<leader>E` | toggle/reveal routes are visible and deterministic |
| `EXP-03R` | open by `Enter`, `v`, `s` | target pane type/path and focus are correct per step |
| `EXP-04R` | mixed `Ctrl-w` navigation | focus path across pane types is deterministic |
| `EXP-05R` | long labels + badges | wrapped rows remain bounded and selectable |
| `EXP-06R` | external FS drift | refreshed rows match filesystem state without corruption |
| `EXP-SCREEN-01` | pane visibility oracle | rendered pane map includes explorer region when expected |
| `EXP-SCREEN-02` | open-target screen oracle | active pane and opened file path match expected frame |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- E2E contract: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
