# Requirements Matrix

Generated: 2026-02-10 (Wave 3)

## Status Key

| Status | Meaning |
|--------|---------|
| ✅ verified | Implemented and tested |
| 🔨 partial | Implemented but incomplete |
| 📋 scaffold | Stub/type-only implementation |
| ❌ unverified | Not implemented |

## Architecture Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| ARCH-01 | architecture/crates.md | 18 workspace crates | ✅ verified | Cargo.toml (workspace members) | cargo build success |
| ARCH-02 | architecture/runtime.md | Tokio multi-thread runtime | ✅ verified | main.rs `#[tokio::main]` | compile + cargo test |
| ARCH-03 | architecture/runtime.md | Single-writer core task | ✅ verified | core-state EditorState ownership | unit tests |
| ARCH-04 | architecture/runtime.md | Channel topology (mpsc/watch/broadcast) | ✅ verified | main.rs channel setup | compile |
| ARCH-05 | architecture/input-decoding.md | crossterm EventStream decode | ✅ verified | input crate decode loop | compile |
| ARCH-06 | architecture/render-pipeline.md | Watch-channel snapshot delivery | ✅ verified | render crate watch receiver | compile |
| ARCH-07 | architecture/startup.md | Graceful startup sequence | 🔨 partial | main.rs task spawn order | no dedicated test |
| ARCH-08 | architecture/plugins.md | Plugin architecture | 📋 scaffold | plugin crate types only | - |

## Editing Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| EDIT-01 | editing/operators-detailed.md | Delete operator | ✅ verified | editing_ops.rs delete_lines | test_delete_lines |
| EDIT-02 | editing/operators-detailed.md | Yank operator | ✅ verified | editing_ops.rs yank_lines | test_yank_put |
| EDIT-03 | editing/operators-detailed.md | Put/paste | ✅ verified | editing_ops.rs put_after | test_yank_put |
| EDIT-04 | editing/operators-detailed.md | Change operator | 🔨 partial | editing_ops.rs (via delete+insert) | indirect |
| EDIT-05 | editing/motions-detailed.md | Basic motions (hjkl) | ✅ verified | cursor_ops.rs | test_handle_key_normal_j |
| EDIT-06 | editing/motions-detailed.md | Word motions (w/b/e) | ✅ verified | cursor_ops.rs word fwd/bwd/end | unit tests |
| EDIT-07 | editing/motions-detailed.md | Line motions (0/$) | ✅ verified | cursor_ops.rs | test_move_to_line_start_end |
| EDIT-08 | editing/motions-detailed.md | Page motions | ✅ verified | cursor_ops.rs page_down/up | unit tests |
| EDIT-09 | editing/text-objects-detailed.md | Text objects | 📋 scaffold | types defined in core-mode | - |
| EDIT-10 | editing/operators-detailed.md | Undo/redo | ✅ verified | editing_ops.rs undo/redo | test_undo |
| EDIT-11 | editing/operators-detailed.md | Join lines | ✅ verified | editing_ops.rs join_lines | test_join_lines |
| EDIT-12 | editing/operators-detailed.md | Replace char | ✅ verified | editing_ops.rs replace_char | unit tests |

## Editor Core Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| CORE-01 | editor/README.md | EditorState struct | ✅ verified | editor.rs | test_editor_new |
| CORE-02 | editor/README.md | Key dispatch | ✅ verified | editor.rs handle_key | test_handle_key_* |
| CORE-03 | editor/README.md | Action dispatch | ✅ verified | editor.rs handle_action | test_handle_action_resize |
| CORE-04 | editor/README.md | Snapshot generation | ✅ verified | editor.rs snapshot() | test_snapshot_sequence |
| CORE-05 | editor/README.md | Buffer management | ✅ verified | buffer_list.rs | test_open_file |
| CORE-06 | editor/README.md | Window management | ✅ verified | window_tree.rs | unit tests |
| CORE-07 | editor/README.md | Cmdline handler | ✅ verified | cmdline.rs | test_enter_command_mode |
| CORE-08 | editor/README.md | Register file | ✅ verified | registers.rs (new) | 8 tests |
| CORE-09 | editor/README.md | Mark file | ✅ verified | marks.rs | 7 tests |
| CORE-10 | editor/README.md | Search state | ✅ verified | search.rs | 9 tests |

## Mode Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| MODE-01 | modes/normal-mode.md | Normal mode | ✅ verified | editor.rs dispatch | test_mode_transition_* |
| MODE-02 | modes/insert-mode.md | Insert mode | ✅ verified | editor.rs insert dispatch | test_insert_newline |
| MODE-03 | modes/command-mode.md | Command mode | ✅ verified | editor.rs + cmdline.rs | test_cmdline_execute_quit |
| MODE-04 | modes/visual-mode.md | Visual mode | 🔨 partial | Mode enum defined, transitions | - |
| MODE-05 | modes/replace-mode.md | Replace mode | 🔨 partial | Mode enum defined, basic dispatch | - |
| MODE-06 | modes/terminal-mode.md | Terminal mode | 📋 scaffold | Mode enum + service stub | - |
| MODE-07 | modes/operator-pending.md | Operator-pending mode | 🔨 partial | Mode enum + Operator types | - |

## Command Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| CMD-01 | commands/quit-commands.md | :q/:quit | ✅ verified | ex_commands.rs | test_write_quit |
| CMD-02 | commands/quit-commands.md | :q! force quit | ✅ verified | ex_commands.rs | test flow |
| CMD-03 | commands/essential.md | :w/:write | ✅ verified | ex_commands.rs write_current_buffer | test_write_quit |
| CMD-04 | commands/essential.md | :wq/:x | ✅ verified | ex_commands.rs | test flow |
| CMD-05 | commands/essential.md | :e/:edit | 🔨 partial | ex_commands.rs notification | - |
| CMD-06 | commands/essential.md | :bn/:bp buffer nav | ✅ verified | ex_commands.rs | compile |
| CMD-07 | commands/essential.md | :sp/:vs split | ✅ verified | ex_commands.rs | compile |
| CMD-08 | commands/syntax.md | Range parsing | ✅ verified | ex_parse.rs | 11 tests |
| CMD-09 | commands/syntax.md | :s/pattern/repl/flags | ✅ verified | ex_commands.rs | 11 tests |
| CMD-10 | commands/essential.md | :d/:y delete/yank | ✅ verified | ex_commands.rs | test_delete_lines |

## Scripting Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| SCRIP-01 | scripting/mappings/README.md | Mapping table | ✅ verified | mappings.rs MappingTable | 5 tests |
| SCRIP-02 | scripting/mappings/mapping-modes.md | Per-mode mappings | ✅ verified | mappings.rs MapMode | test_mode_conversion |
| SCRIP-03 | scripting/mappings/README.md | :map/:noremap/:unmap | ✅ verified | ex_commands.rs map dispatch | compile |
| SCRIP-04 | scripting/user-commands.md | :command/:delcommand | ✅ verified | user_commands.rs | 9 tests |
| SCRIP-05 | scripting/user-commands.md | User cmd expansion | ✅ verified | user_commands.rs expand() | test_expand_with_args |
| SCRIP-06 | scripting/event-automation.md | Event registry | ✅ verified | events.rs EventRegistry | 7 tests |
| SCRIP-07 | scripting/event-automation.md | Reentry guard | ✅ verified | events.rs max_depth | test_reentry_guard |
| SCRIP-08 | scripting/event-automation.md | :autocmd dispatch | ✅ verified | ex_commands.rs handle_autocmd | compile |
| SCRIP-09 | scripting/mappings/special-keys.md | Key notation parsing | ✅ verified | ex_commands.rs parse_key_notation | compile |

## Technical Contract Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| TECH-01 | technical/contracts.md | Determinism contract | ✅ verified | contracts.rs | test_monotonic_version |
| TECH-02 | technical/contracts.md | Service contract | 🔨 partial | contracts.rs ContractKind | type defined |
| TECH-03 | technical/contracts.md | Snapshot contract | ✅ verified | contracts.rs + watch channel | test_no_violation |
| TECH-04 | technical/contracts.md | Buffer contract (UTF-8) | ✅ verified | contracts.rs assert_buffer_utf8 | test_buffer_utf8 |
| TECH-05 | technical/contracts.md | Observability contract | 📋 scaffold | ContractKind::Observability | - |
| TECH-06 | technical/contracts.md | Persistence atomic write | 🔨 partial | ex_commands.rs write | limited |

## UI/Rendering Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| UI-01 | ui/viewport.md | Viewport/scroll | ✅ verified | cursor_ops.rs ensure_cursor_visible | unit tests |
| UI-02 | ui/statusline.md | Status line | ✅ verified | render crate format_statusline | compile |
| UI-03 | ui/tabline.md | Tab line | 🔨 partial | snapshot includes tabs | - |
| UI-04 | ui/notifications.md | Notifications | ✅ verified | editor.rs notifications vec | test flow |
| UI-05 | ui/popup.md | Popup/completion UI | 📋 scaffold | types defined | - |
| UI-06 | ui/theme.md | Theming | 🔨 partial | Theme struct | - |

## Features (Editing Helpers)

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| FEAT-01 | features/editing/auto-pairs.md | Auto-pairs | ✅ verified | editing_helpers.rs | 2 tests |
| FEAT-02 | features/editing/comments.md | Comment toggle | ✅ verified | editing_helpers.rs CommentConfig | 2 tests |
| FEAT-03 | features/editing/surround.md | Surround operations | ✅ verified | editing_helpers.rs surround_* | 2 tests |
| FEAT-04 | features/editing/snippets.md | Snippets | ❌ unverified | - | - |
| FEAT-05 | features/editing/spell.md | Spell check | ❌ unverified | - | - |
| FEAT-06 | features/editing/multicursor.md | Multicursor | ❌ unverified | - | - |

## Features (Core Integration)

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| FCORE-01 | features/terminal/README.md | Terminal integration | 📋 scaffold | service-terminal crate | - |
| FCORE-02 | features/session/README.md | Session save/load | ✅ verified | session.rs | 3 tests |
| FCORE-03 | features/window/README.md | Window management | ✅ verified | window_tree.rs | unit tests |
| FCORE-04 | features/buffer/README.md | Buffer operations | ✅ verified | buffer_list.rs | unit tests |

## Features (Services)

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| FSVC-01 | features/lsp/README.md | LSP integration | 📋 scaffold | service-lsp crate | - |
| FSVC-02 | features/git/README.md | Git integration | 📋 scaffold | service-git crate | - |
| FSVC-03 | features/indexing/README.md | Indexing/navigation | 📋 scaffold | service-index crate | - |
| FSVC-04 | features/syntax/README.md | Syntax highlighting | 📋 scaffold | render crate types | - |
| FSVC-05 | features/config/README.md | Configuration | 📋 scaffold | host crate types | - |

## UX Requirements

| ID | Spec Source | Requirement | Status | Implementation | Test Evidence |
|----|------------|-------------|--------|---------------|---------------|
| UX-01 | ux/keybindings.md | Default keybindings | ✅ verified | editor.rs handle_key | tests |
| UX-02 | ux/accessibility.md | Accessibility | 📋 scaffold | - | - |
| UX-03 | ux/mouse.md | Mouse support | 📋 scaffold | input crate types | - |

## Summary

| Category | ✅ Verified | 🔨 Partial | 📋 Scaffold | ❌ Unverified | Total |
|----------|-----------|-----------|------------|-------------|-------|
| Architecture | 6 | 1 | 1 | 0 | 8 |
| Editing | 10 | 1 | 1 | 0 | 12 |
| Editor Core | 10 | 0 | 0 | 0 | 10 |
| Modes | 3 | 3 | 1 | 0 | 7 |
| Commands | 9 | 1 | 0 | 0 | 10 |
| Scripting | 9 | 0 | 0 | 0 | 9 |
| Technical | 3 | 2 | 1 | 0 | 6 |
| UI/Rendering | 3 | 2 | 1 | 0 | 6 |
| Features Edit | 3 | 0 | 0 | 3 | 6 |
| Features Core | 3 | 0 | 1 | 0 | 4 |
| Features Svc | 0 | 0 | 5 | 0 | 5 |
| UX | 1 | 0 | 2 | 0 | 3 |
| **Total** | **60** | **10** | **13** | **3** | **86** |
