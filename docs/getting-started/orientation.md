# Orientation

## What `kjxlkj` Is

- `kjxlkj` is a contract-first resource system for LLM-operated workflows.
- The documentation under `docs/` is the only active canon for intended behavior.
- Operators use Docker Compose to run and verify the stack.
- Contributors update docs first, then realign code to match.

## Runtime Shape

- `postgres` stores live resources, snapshots, settings, analytics, and sessions.
- `minio` stores current and historical media binaries through an S3-compatible API.
- `app` is the Rust web service that runs migrations, ensures the bucket exists, and serves HTML plus JSON or multipart endpoints.
- `verify` runs Rust and docs quality gates inside Compose.
- `visual-verify` runs browser-rendered screenshot checks against the live stack.

## Fastest Reading Path

1. Read [compose-quickstart.md](compose-quickstart.md) to boot the stack.
2. Read [first-session.md](first-session.md) to create the first admin, note, and media resource.
3. Read [verification.md](verification.md) to run the full acceptance bundle.
4. Read [where-next.md](where-next.md) for deeper task-specific documentation.

## Key Defaults

- Host app port default: `8080`
- Untouched session timeout default: `1440` minutes
- Untouched new-resource visibility default: public
- Canonical deployment model: one host running Docker Compose
