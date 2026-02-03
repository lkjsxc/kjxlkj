# Tokio Runtime Model

## Why Tokio

Tokio provides:

- Cooperative task scheduling
- Structured cancellation primitives
- Async IO integration (FS, sockets)
- A common substrate for protocol clients (LSP)

## Runtime topology

```mermaid
graph TD
  RT[Tokio Runtime]
  RT --> C[Core Task]
  RT --> R[Render Task]
  RT --> S1[Service: Index]
  RT --> S2[Service: LSP]
  RT --> S3[Service: Git]
  RT --> S4[Service: FS Watch]
  RT --> S5[Service: Terminal]

  C <--> MB[Message Bus (bounded)]
  MB <--> S1
  MB <--> S2
  MB <--> S3
  MB <--> S4
  MB <--> S5

  C -->|snapshots| R

```

## Related

- Architecture index: [README.md](README.md)
- Latency and ordering: [docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- LSP service: [docs/spec/features/lsp/lsp.md](/docs/spec/features/lsp/lsp.md)
- Git service: [docs/spec/features/git/git.md](/docs/spec/features/git/git.md)
- Indexer service: [docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- Terminal service: [docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- Session service: [docs/spec/features/session/README.md](/docs/spec/features/session/README.md)
