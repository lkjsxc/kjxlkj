# Proposal: Anti-MVP Measures for Full Implementation

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Previous reconstruction produced a minimal implementation (MVP) where most features existed only as type definitions or scaffold code without real behavior. The code was approximately one-tenth of the expected volume. Features were marked as "implemented" in conformance docs but were actually unreachable.

## Root causes

| Cause | Description |
|---|---|
| Broad TODO items | TODOs like "implement editing features" allow an agent to create stub types and check the item off. |
| No reachability checks | Conformance claims were not gated on user-reachable behavior. |
| Missing integration tests | Tests verified type existence rather than end-to-end behavior. |
| No feature-use audits | The app's main entry point did not exercise most features. |

## Required measures

### 1. Granular TODO items with acceptance criteria

Every TODO item MUST include:

- The exact user-facing command or keybinding that exercises the feature.
- An acceptance criterion in Given/When/Then form.
- A required test type (unit, integration, headless E2E, or PTY E2E).

### 2. Feature reachability audit

Before marking any feature complete, the implementation MUST demonstrate:

- The feature is reachable from the binary's `main` function through real user input.
- The feature produces observable output (screen change, file write, or state mutation).
- A test exercises this exact path.

### 3. Integration-first test design

Tests MUST validate behavior through real paths, not just type construction:

- Insert text, save file, verify file contents on disk.
- Open terminal, run a command, verify output appears in scrollback.
- Split window, navigate between splits, verify focus changes.
- Save session, load session, verify window layout matches.

### 4. No scaffold-only completions

The reconstruction prompt already prohibits evidence-free completion. Additionally:

- A TODO item MUST NOT be checked if the feature only exists as types/structs.
- A TODO item MUST NOT be checked if the feature is not wired into the main dispatch loop.
- The `CONFORMANCE.md` status vocabulary (`implemented`, `partial`, `scaffold-only`, `planned`) MUST be used accurately.

### 5. Minimum code volume targets per crate

| Crate | Minimum lines (excluding tests) | Rationale |
|---|---|---|
| `kjxlkj-core-text` | 400 | Rope wrapper, grapheme decomposition, display width, line operations |
| `kjxlkj-core-edit` | 600 | Operators, text objects, motions, register operations |
| `kjxlkj-core-mode` | 500 | Mode state machines, transition logic, cursor clamping |
| `kjxlkj-core-state` | 500 | Editor state, command dispatch, viewport follow |
| `kjxlkj-render` | 500 | Cell rendering, wrapping, gutter, statusline, diff display |
| `kjxlkj-input` | 300 | Key parsing, mapping expansion, leader handling |
| `kjxlkj-host` | 300 | Terminal raw mode, event loop, PTY harness |
| `kjxlkj-service-terminal` | 400 | Escape parsing state machine, PTY spawn, screen buffer |
| `kjxlkj-service-lsp` | 300 | JSON-RPC client, request/response lifecycle |
| `kjxlkj-service-git` | 200 | Git subprocess, diff parsing, status/blame |

These are minimums. Individual source files MUST still be under 200 lines each, which means splitting into well-structured modules.

### 6. Mandatory wiring checklist

Before the reconstruction is considered complete, the following wiring points MUST be verified:

| Wiring point | Requirement |
|---|---|
| Key dispatch | All keybindings in [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md) MUST route to real handlers. |
| Command dispatch | All commands in [/docs/spec/commands/README.md](/docs/spec/commands/README.md) MUST route to real handlers. |
| Render pipeline | Snapshots MUST flow from core state through the render pipeline to terminal output. |
| Service bus | Services MUST send and receive messages through the message bus. |
| Session I/O | `:SessionSave` MUST write JSON; `:SessionLoad` MUST read and restore. |
| Terminal PTY | `:terminal` MUST spawn a real PTY process. |
| File I/O | `:w` and `:e` MUST perform real filesystem reads and writes. |

## Acceptance criteria

The next reconstruction wave is acceptable only when:

- All TODO items have passing tests that exercise real user paths.
- No `scaffold-only` items remain in `CONFORMANCE.md` for core features.
- The binary can perform a basic editing workflow: open file, edit, save, split, terminal, session save/load.
- Code volume meets the minimum targets above.

## Related

- Reconstruction prompt: [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)
- Conformance ledger: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
