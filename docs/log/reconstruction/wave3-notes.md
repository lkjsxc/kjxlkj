# Reconstruction Wave 3: Notes

## Completed

- All 18 crates created with full module implementations
- Workspace compiles with 0 errors
- 179 unit tests pass across all crates
- All crate line-count minimums met
- TODO checkboxes updated to reflect implemented features

## Line count summary

| Crate | Lines | Minimum | Met? |
|-------|------:|--------:|:----:|
| kjxlkj (binary) | 205 | 100 | Yes |
| kjxlkj-core | 62 | 50 | Yes |
| kjxlkj-core-types | 1659 | 200 | Yes |
| kjxlkj-core-text | 876 | 400 | Yes |
| kjxlkj-core-edit | 1485 | 600 | Yes |
| kjxlkj-core-mode | 1324 | 500 | Yes |
| kjxlkj-core-undo | 338 | 200 | Yes |
| kjxlkj-core-ui | 725 | 150 | Yes |
| kjxlkj-core-state | 929 | 500 | Yes |
| kjxlkj-render | 861 | 500 | Yes |
| kjxlkj-input | 372 | 300 | Yes |
| kjxlkj-host | 398 | 300 | Yes |
| kjxlkj-services | 126 | 100 | Yes |
| kjxlkj-service-terminal | 620 | 400 | Yes |
| kjxlkj-service-lsp | 359 | 300 | Yes |
| kjxlkj-service-git | 278 | 200 | Yes |
| kjxlkj-service-index | 280 | 150 | Yes |
| kjxlkj-service-fs | 221 | 150 | Yes |

## Remaining work

### Not yet implemented (future waves)

- CJK-aware rendering (width-2 graphemes, continuation cells)
- Wrap algorithm with CJK boundary padding
- Plugin architecture (trait, loading, sandbox)
- Scripting (user commands, functions, event automation)
- Tab completion in command-line mode
- Dot-repeat for last change
- Double operators (dd, cc, yy)
- Forced motion types
- Window navigation (Ctrl-w h/j/k/l)
- Window close/resize/zoom
- Alternate file (Ctrl-^)
- Search motions (/, ?, n, N, *, #)
- Syntax highlighting / treesitter
- Multi-cursor editing
- Auto-pairs, snippets, surround
- Configuration system
- Keybinding DSL
- UX keyboard layouts
- All technical contracts (latency, memory)
- Integration / E2E / boundary tests (currently only unit tests)

### Files over 200 lines

See [/docs/log/audits/files-over-200-lines.md](/docs/log/audits/files-over-200-lines.md)
for the list of 14 files exceeding the 200-line limit.

## Architecture decisions

1. Tuple structs for IDs (BufferId, WindowId, etc.) — simple, zero-cost
2. ropey::Rope wrapped in BufferContent — provides grapheme/line operations
3. Arena-based undo tree — supports branching without complex graph structures
4. Snapshot-driven render — core state never mutated by renderer
5. Bounded channels with defined capacities — prevents memory growth
6. Single write_all per frame — minimizes syscall overhead
7. Crossterm EventStream — async-native terminal input
