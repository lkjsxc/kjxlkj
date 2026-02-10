# File Explorer (Built-in)

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

The explorer is a first-class editor window in the shared layout tree.

## Core Contract

| Requirement | Detail |
|---|---|
| Window identity | explorer is a normal `WindowId` leaf |
| Launch wiring | command and key triggers must reach visible runtime behavior |
| Mixed navigation | `Ctrl-w` works across explorer/buffer/terminal leaves |
| Deterministic refresh | tree updates are explicit and reproducible |

## Launch and Wiring

| Trigger | Required Path |
|---|---|
| `:Explorer` | command parser -> core action -> explorer leaf open/focus |
| `:ExplorerClose` | command parser -> core action -> close explorer leaf |
| `:ExplorerReveal` | command parser -> reveal active buffer path in tree |
| `<leader>e` | keymap -> explorer toggle action |
| `<leader>E` | keymap -> explorer reveal action |

An explorer feature MUST NOT be marked complete if these triggers do not produce visible behavior.

## Explorer State Model

| Field | Requirement |
|---|---|
| root path | normalized absolute project root |
| visible nodes | deterministic flattening of expanded tree nodes |
| selected node index | clamped to visible range |
| expansion set | stable expanded directory IDs |
| clipboard state | optional cut/copy pending node set |

## Navigation and Open Targets

| Key | Behavior |
|---|---|
| `j` / `k` | move selection down/up |
| `h` | collapse directory or move to parent node |
| `l` | expand directory or open file |
| `Enter` / `o` | open file in current window |
| `v` | open file in vertical split |
| `s` | open file in horizontal split |
| `t` | open file in new tab |
| `q` | close explorer window |

Open target behavior MUST keep explorer state valid and focus deterministic.

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

All mutating operations MUST be routed through filesystem service calls and reflected by refresh.

## Refresh and External Drift

| Scenario | Requirement |
|---|---|
| external file create/rename/delete | refresh updates visible tree deterministically |
| missing path during reveal | fallback to nearest existing ancestor node |
| permission denied | operation fails with explicit diagnostic and no state corruption |

## Rendering and Overflow

| Rule | Requirement |
|---|---|
| On-screen guarantee | rendered rows stay inside explorer text area |
| Long node labels | labels wrap to continuation rows, never overflow right boundary |
| Selection identity | wrapped continuation rows map to one logical node |
| Badge safety | git/diagnostic badges never corrupt node text alignment |

## Error Handling

- invalid path operations fail with actionable messages
- failed rename/delete does not mutate in-memory tree state
- refresh cancellation leaves prior visible state intact

## Mandatory Verification

| ID | Scenario |
|---|---|
| `EXP-01R` | `:Explorer` opens explorer window from real command path |
| `EXP-02R` | `<leader>e` toggle and `<leader>E` reveal are wired |
| `EXP-03R` | open selected file in current/horizontal/vertical targets |
| `EXP-04R` | `Ctrl-w` navigation across explorer/buffer/terminal |
| `EXP-05R` | long labels wrap safely with stable selection identity |
| `EXP-06R` | external FS change refresh reflects create/rename/delete |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
