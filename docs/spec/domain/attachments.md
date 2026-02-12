# Attachments Contract

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Storage Model

Attachments are stored in PostgreSQL as chunk rows.

| Parameter | Value |
|---|---|
| Chunk size | 4 MiB |
| Per-file max | 500 MiB |
| Integrity | SHA-256 checksum at file and chunk levels |

## Upload Rules

- Upload MUST stream without loading entire file in memory.
- Server MUST reject files larger than max with `413`.
- Completed upload MUST persist metadata and chunk rows transactionally.

## Download Rules

- Download MUST stream chunks in index order.
- Stream MUST verify chunk continuity before response completion.

## Deletion Rules

- Deleting attachment MUST remove chunk rows and metadata row atomically.

## Related

- API endpoints: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- Performance constraints: [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md)
