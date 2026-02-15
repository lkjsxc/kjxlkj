# Runtime Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Runtime Topology

```mermaid
graph TD
 RT[Tokio Runtime]
 RT --> HTTP[Actix HTTP Server]
 RT --> WS[Actix WebSocket Handlers]
 RT --> BG[Background Jobs and Automation]
 RT --> LLM[LLM Provider Adapters]
 RT --> DBPOOL[SQLx PgPool]

 HTTP --> CORE[Domain Services]
 WS --> CORE
 CORE --> DBPOOL
 BG --> DBPOOL
 BG --> LLM
```

## Startup Sequence (normative)

1. load `.env` (secrets) and validate required secret keys
2. load and validate non-secret runtime config from `data/config.json`
3. initialize tracing and error handling
4. initialize PostgreSQL pool
5. run pending SQL migrations
6. start Actix server with HTTP + WS routes
7. initialize LLM provider adapters (OpenRouter/LM Studio)
8. start background workers (automation/export/backup/job polling)

## Shutdown Sequence

1. stop accepting new connections
2. drain active HTTP/WS tasks with bounded timeout
3. flush telemetry and close DB pool
4. terminate process cleanly

## Concurrency Rules

- Writes to one note stream MUST serialize by note ID lock or transaction strategy.
- Automation writes MUST serialize by target stream identity.
- Librarian operation application MUST serialize by workspace + target note stream.
- Cross-stream writes MAY run in parallel.
- WS broadcast ordering MUST follow committed event sequence.
- Slow clients MUST NOT block global broadcast loops.

## Related

- Domain events: [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- Automation: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- Librarian protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- Configuration: [configuration.md](configuration.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
