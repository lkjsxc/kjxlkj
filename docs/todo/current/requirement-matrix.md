# Requirement Matrix

Back: [/docs/todo/current/phases/phase-0-foundation.md](/docs/todo/current/phases/phase-0-foundation.md)

Generated from normative spec files in `/docs/spec/`.

## Editing Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-EDIT-01 | editing/cursor/README.md | Grapheme-based cursor movement | implemented |
| R-EDIT-02 | editing/cursor/README.md | End-inclusive Normal cursor (clamp before \n) | implemented |
| R-EDIT-03 | editing/cursor/README.md | End-exclusive Insert cursor | implemented |
| R-EDIT-04 | editing/operators/ | Delete, change, yank operators with motions | implemented |
| R-EDIT-05 | editing/text-objects/ | Word, sentence, paragraph text objects | implemented |
| R-EDIT-06 | editing/undo-redo.md | Undo/redo tree with branching | implemented |
| R-EDIT-07 | editing/registers.md | Named registers for yank/delete/paste | implemented |

## Mode Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-MODE-01 | modes/normal/ | Normal mode key dispatch | implemented |
| R-MODE-02 | modes/insert/ | Insert mode character input | implemented |
| R-MODE-03 | modes/visual/ | Visual char/line/block selection | implemented |
| R-MODE-04 | modes/command/ | Command-line (ex) mode | implemented |
| R-MODE-05 | modes/replace/ | Replace mode single-char override | implemented |
| R-MODE-06 | modes/insert/input/insert-japanese-ime.md | IME composition model | implemented |

## Command Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-CMD-01 | commands/essential.md | :w, :q, :wq, :q!, :e | implemented |
| R-CMD-02 | commands/buffer/ | :bn, :bp, :bd | implemented |
| R-CMD-03 | commands/file/ | :w <path>, file-write path | implemented |
| R-CMD-04 | commands/syntax.md | Ex command parser | implemented |
| R-CMD-05 | commands/quit-commands.md | Quit variants (force, all) | implemented |

## Window Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-WIN-01 | editor/windows.md | Window tree / split layout | implemented |
| R-WIN-02 | features/window/splits-windows.md | Horizontal & vertical splits | implemented |
| R-WIN-03 | features/window/splits-windows.md | Ctrl-w navigation (h/j/k/l/w/W) | implemented |
| R-WIN-04 | editor/windows.md | Window close with rebalance | implemented |
| R-WIN-05 | editor/windows.md | Stable WindowId across layout changes | implemented |

## Terminal Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-TERM-01 | features/terminal/terminal.md | PTY-backed terminal window | implemented |
| R-TERM-02 | features/terminal/escape-parser.md | ECMA-48 VT state machine | implemented |
| R-TERM-03 | features/terminal/terminal.md | Terminal resize (SIGWINCH) | implemented |
| R-TERM-04 | features/terminal/terminal.md | TerminalInsert mode | implemented |

## Explorer Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-EXP-01 | features/navigation/file_explorer.md | Explorer launch wiring | implemented |
| R-EXP-02 | features/navigation/file_explorer.md | Explorer navigation (j/k/h/l) | implemented |
| R-EXP-03 | features/navigation/file_explorer.md | Open file from explorer | implemented |
| R-EXP-04 | features/navigation/file_explorer.md | File operations (create/rename/delete) | implemented |

## UI / Viewport Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-UI-01 | features/ui/viewport.md | Vertical scrolloff enforcement | implemented |
| R-UI-02 | features/ui/viewport.md | Horizontal follow (no-wrap mode) | implemented |
| R-UI-03 | features/ui/viewport.md | Soft-wrap with width-2 boundary padding | implemented |
| R-UI-04 | features/ui/viewport.md | zz/zt/zb viewport commands | implemented |
| R-UI-05 | ui/views.md | Status line, tab line, command line | implemented |

## Session Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-SESS-01 | features/session/sessions.md | Session save/load (JSON) | implemented |
| R-SESS-02 | features/session/sessions.md | Terminal/explorer persisted as window nodes | implemented |
| R-SESS-03 | features/session/sessions.md | Auto-session on exit/startup | implemented |

## I18N Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-I18N-01 | modes/insert/input/insert-japanese-ime.md | IME compose/commit/cancel | implemented |
| R-I18N-02 | modes/insert/input/insert-japanese-ime.md | IME leader isolation | implemented |
| R-I18N-03 | editing/cursor/README.md | CJK display-width cursor | implemented |

## Service Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-SVC-01 | features/lsp/README.md | LSP completion and diagnostics | implemented |
| R-SVC-02 | features/git/README.md | Git status and diff | implemented |
| R-SVC-03 | features/navigation/finder.md | Indexing / finder service | implemented |
| R-SVC-04 | features/syntax/README.md | Syntax highlighting | implemented |

## Architecture Domain

| ID | Spec Source | Requirement | Status |
|---|---|---|---|
| R-ARCH-01 | architecture/source-layout.md | 18-crate workspace layout | implemented |
| R-ARCH-02 | architecture/runtime.md | Tokio multi-task runtime | implemented |
| R-ARCH-03 | architecture/input-decoding.md | Input decoding with shifted normalization | implemented |
| R-ARCH-04 | architecture/render-pipeline.md | Snapshot-based render pipeline | implemented |
| R-ARCH-05 | architecture/crates.md | Crate dependency DAG | implemented |
