# Architecture

Back: [/docs/spec/README.md](/docs/spec/README.md)
## Documents

| Document | Purpose |
|----------|---------|
| [crates.md](crates.md) | Crate structure |
| [workspace-manifest.md](workspace-manifest.md) | Root workspace manifest and dependency policy |
| [plugins.md](plugins.md) | Plugin architecture |
| [runtime.md](runtime.md) | Runtime design |

## System shape

```mermaid
graph TD
  UI[Terminal UI] -->|events| HOST[Host]
  HOST -->|actions| CORE[Core Task: Editor State]

  CORE -->|immutable snapshot| RENDER[Render Task]
  RENDER -->|frame| UI

  CORE <--> BUS[Message Bus]

  BUS <--> IDX[Indexer Service]
  BUS <--> LSP[LSP Service]
  BUS <--> GIT[Git Service]
  BUS <--> FS[FS Watch Service]
  BUS <--> TERM[Terminal Service]

  IDX -->|results| BUS
  LSP -->|diagnostics, edits| BUS
  GIT -->|hunks, blame, status| BUS
  FS -->|file events| BUS
  TERM -->|pty events| BUS

```

## Related

- Runtime model: [runtime.md](runtime.md)
