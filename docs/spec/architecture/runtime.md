# Runtime Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Runtime Topology

```mermaid
graph TD
  RT[Tokio Runtime]
  RT --> HTTP[Actix HTTP Server]
  RT --> WS[Actix WebSocket Handlers]
  RT --> BG[Background Jobs]
  RT --> DBPOOL[SQLx PgPool]

  HTTP --> CORE[Domain Services]
  WS --> CORE
  CORE --> DBPOOL
  BG --> DBPOOL
```

## Startup Sequence (normative)

1. load and validate configuration
2. initialize tracing and error handling
3. initialize PostgreSQL pool
4. run pending SQL migrations
5. start Actix server with HTTP + WS routes
6. start background workers (export/backup/job polling)

## Shutdown Sequence

1. stop accepting new connections
2. drain active HTTP/WS tasks with bounded timeout
3. flush telemetry and close DB pool
4. terminate process cleanly

## Concurrency Rules

- Writes to one note stream MUST serialize by note ID lock or transaction strategy.
- Cross-note writes MAY run in parallel.
- WS broadcast ordering MUST follow committed event sequence.
- Slow clients MUST NOT block global broadcast loops.

## Related

- Domain events: [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
