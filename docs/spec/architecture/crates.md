# Crates

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Canonical Rust workspace decomposition.

## Required Crates

| Group | Crate | Purpose |
|---|---|---|
| app | `kjxlkj-server` | startup and route wiring |
| http | `kjxlkj-http` | HTTP handlers |
| ws | `kjxlkj-ws` | realtime protocol |
| domain | `kjxlkj-domain` | entities and rules |
| db | `kjxlkj-db` | persistence and migrations |
| auth | `kjxlkj-auth` | auth/session/security |
| search | `kjxlkj-search` | hybrid lexical/semantic retrieval |
| automation | `kjxlkj-automation` | `kjxlkj-agent` loop and rule execution |
| rbac | `kjxlkj-rbac` | permission checks |
| workspace | `kjxlkj-workspace` | workspace/project services |

## Related

- Source layout: [source-layout.md](source-layout.md)
- Runtime model: [runtime.md](runtime.md)
