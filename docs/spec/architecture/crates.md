# Crates

## Source layout requirement

When implementation exists, it MUST include at least 10 crates under `src/crates/`.

This repository includes an implementation. This document describes the intended (and current) crate topology.

## Workspace members

The Cargo workspace includes these crates under `src/crates/`:

| Crate | Role |
|---|---|
| `kjxlkj` | Binary entrypoint (wires host, core, render, services) |
| `kjxlkj-core` | Core facade crate (exports editor-facing core APIs) |
| `kjxlkj-core-types` | Shared types used across core, UI, render, and services |
| `kjxlkj-core-text` | Text model (rope/graphemes) and text-centric helpers |
| `kjxlkj-core-edit` | Editing primitives and operators |
| `kjxlkj-core-mode` | Modal state machines and input interpretation |
| `kjxlkj-core-undo` | Undo/redo model |
| `kjxlkj-core-ui` | UI model types and snapshot structures |
| `kjxlkj-core-state` | Editor state aggregation and snapshot production |
| `kjxlkj-host` | Terminal host integration (lifecycle + plumbing) |
| `kjxlkj-input` | Terminal input decoding → actions/events |
| `kjxlkj-render` | Rendering pipeline (snapshot → terminal frame) |
| `kjxlkj-services` | Service supervisor and service wiring |
| `kjxlkj-service-lsp` | Language Server Protocol client service |
| `kjxlkj-service-git` | Git integration service |
| `kjxlkj-service-index` | Index/navigation service |
| `kjxlkj-service-fs` | Filesystem IO/watch service |
| `kjxlkj-service-terminal` | Terminal/PTY service |

## Workspace overview

```mermaid
graph TD
  BIN[kjxlkj (binary)] --> CORE[kjxlkj-core (facade)]
  BIN --> HOST[kjxlkj-host]
  BIN --> INPUT[kjxlkj-input]
  BIN --> RENDER[kjxlkj-render]
  BIN --> SVC[kjxlkj-services]

  CORE --> CT[kjxlkj-core-types]
  CORE --> TEXT[kjxlkj-core-text]
  CORE --> EDIT[kjxlkj-core-edit]
  CORE --> MODE[kjxlkj-core-mode]
  CORE --> UNDO[kjxlkj-core-undo]
  CORE --> UI[kjxlkj-core-ui]
  CORE --> STATE[kjxlkj-core-state]

  SVC --> LSP[kjxlkj-service-lsp]
  SVC --> GIT[kjxlkj-service-git]
  SVC --> IDX[kjxlkj-service-index]
  SVC --> FS[kjxlkj-service-fs]
  SVC --> TERM[kjxlkj-service-terminal]

  INPUT --> CT
  RENDER --> UI
  UI --> CT
  TEXT --> CT
  EDIT --> CT
  MODE --> CT
  UNDO --> CT
  STATE --> CT
  STATE --> TEXT
  STATE --> EDIT
  STATE --> MODE
  STATE --> UNDO
  UI --> TEXT

```

## Related

- Runtime model: [runtime.md](runtime.md)
- Architecture index: [README.md](README.md)
