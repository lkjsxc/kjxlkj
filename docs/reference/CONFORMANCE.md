# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | confirmed by deterministic evidence in current repo state |
| `partial` | partly available with user-visible gaps |
| `scaffold-only` | structural artifacts exist but runtime path is incomplete |
| `unverified` | no current evidence |

## Current Snapshot (2026-02-10, reconstructed)

Repository is in active reconstruction state. Phase 5 (hardening) active.

## Build Verification

| Check | Status | Evidence |
|---|---|---|
| Workspace builds | `verified` | `cargo build --workspace` passes, 18 crates |
| Formatting clean | `verified` | `cargo fmt --all -- --check` passes |
| Clippy clean | `verified` | `cargo clippy --workspace --all-targets` zero warnings |
| All tests pass | `verified` | `cargo test --workspace` 224 tests pass |
| CI workflow present | `verified` | `.github/workflows/ci.yml` exists |
| All files under 200 lines | `verified` | max file is 200 lines (116 source files) |
| Directory children ≤ 12 | `verified` | all src/ directories ≤ 12 direct children |

## Domain Summary

| Domain | Status | Note |
|---|---|---|
| Input decoding | `partial` | Shift normalization verified (WR-01), KI tests pass |
| Mode transitions | `verified` | Normal/Insert/Visual/Command/Replace/OpPending verified; visual dispatch with operators; replace overwrite with backspace restore |
| Cursor semantics | `partial` | a/A/I wired with CUR-01 through CUR-05 tests |
| Editing primitives | `verified` | Insert/delete/motion/operator verified; named registers (RegisterSet) with numbered, special; CE tests + register_tests + gap_tests |
| Text/rope model | `verified` | CT-01 through CT-11 all pass |
| Undo tree | `partial` | Basic undo/redo verified |
| Rendering | `partial` | Grid/cell model verified (RR tests), diff rendering exists |
| Ex commands | `verified` | :w, :w path, :q, :e, :set, :split, :vsplit parsed, routed, and executed |
| Window tree | `partial` | Splits, tab pages, window navigation present |
| Terminal service | `verified` | VT parser, screen model, alternate screen, CSI/SGR dispatch; ST-01 to ST-12, PE-01 to PE-06 tests pass |
| Explorer | `verified` | Toggle/reveal, j/k/h/l nav (expand/collapse), file ops (create/rename/delete), dispatch_explorer_key; gap_tests |
| LSP service | `verified` | LspService with lifecycle, crash recovery, request dispatch; 8 tests pass |
| Git service | `verified` | GitService with status cache, hunk navigation, signs; 8 tests pass |
| Index/Finder service | `verified` | IndexService with fuzzy matching, finder queries; 12 tests pass |
| Syntax highlighting | `verified` | Language detection, keyword/string/comment highlighting; 18 tests pass |
| I18N/IME | `verified` | IME composition model with leader isolation; JP-01 to JP-05 tests pass |
| Long-line wrap safety | `verified` | Width-2 boundary padding; BD-01 through BD-10 tests pass |
| Session persistence | `verified` | SessionData serde with layout tree; auto-session save/load on exit/startup; gap_tests |
| Boundary/stress | `verified` | BD-03 to BD-10 boundary tests pass (7 new + 3 existing) |
| Source topology | `verified` | 18 crates, all dirs ≤ 12 children, all files ≤ 200 lines |
| Spec authority | `verified` | `/docs/spec/` is canonical target |
| Reconstruction controls | `verified` | `/docs/todo/` governs rebuild sequencing and gates |

## Claim Rules

Any runtime conformance claim MUST remain absent until reimplementation produces:

1. reachable behavior from real input path
2. deterministic verification evidence
3. synchronized updates to `LIMITATIONS` and `DRIFT_MATRIX`

## Related

- Open gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Mismatch matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
