# Glossary

Terminology used in kjxlkj specifications.

## Architecture Terms

| Term | Meaning |
|------|---------|
| Core task | The single Tokio task that owns and mutates editor state |
| Service | A supervised Tokio task that performs IO or compute |
| Snapshot | Immutable read-only projection of editor state for rendering |
| Message bus | Typed, bounded channels connecting core and services |
| Stale result | Service result targeting older buffer version; dropped |
| Backpressure | Bounded queues with overflow/coalescing policies |

## Editor Terms

| Term | Meaning |
|------|---------|
| Buffer | In-memory text container backed by rope structure |
| Window | Viewport into a buffer |
| Mode | Input context (Normal, Insert, Visual, Command, Replace) |
| Motion | Cursor movement command |
| Operator | Action operating on text |
| Text object | Semantic text region |
| Register | Named text storage |
| Mark | Saved cursor position |

## Related

- Full glossary: [docs/overview/glossary.md](/docs/overview/glossary.md)
- Architecture: [docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
