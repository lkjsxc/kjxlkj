# Improvement Ideas

Back: [/docs/logs/README.md](README.md)

Ideas collected during the reconstruction session for future consideration.

## Architecture

- **SQLx compile-time checking**: Currently using runtime-checked queries
  (`query_as::<_, T>()`) to build without `DATABASE_URL`. Once the Docker
  container is operational, generate an offline `.sqlx` cache directory to
  enable `query_as!()` compile-time macro verification.

- **Cross-actor WebSocket broadcast**: Current WS implementation uses per-session
  actors. A future improvement would add a shared broadcast registry (likely via
  `actix::Addr` or `tokio::broadcast`) so that note edits propagate to all
  connected sessions in real time.

- **Connection pooling tuning**: `PgPoolOptions` currently uses sensible defaults.
  Once under load, tune `max_connections`, `idle_timeout`, and
  `acquire_timeout` based on actual usage patterns.

## Frontend

- **Extract Lit sub-components**: The `app-shell.ts` (422 lines) bundles all
  views (setup, login, notes list, note detail) in one component. Extracting
  each view into its own Lit element would improve readability and testability.

- **Markdown editor integration**: The current editor is a plain `<textarea>`.
  Integrating CodeMirror 6 or ProseMirror with markdown syntax highlighting
  and wiki-link autocompletion would match the Obsidian-like spec more fully.

- **Offline/PWA support**: The editor already tracks local draft state. Adding
  a service worker and IndexedDB cache would enable true offline editing with
  sync-on-reconnect.

## Testing

- **Property-based tests**: Use `proptest` crate for domain invariant testing
  (e.g., metadata key validation, note kind serialization round-trips).

- **Snapshot tests**: Use `insta` crate for JSON response snapshot testing to
  catch unintended API contract changes.

- **Integration test harness**: Create a `tests/` directory with a test helper
  that spins up a PostgreSQL container (via `testcontainers` crate) for
  full-stack integration testing.

## Security

- **CSP nonce injection**: Add Content-Security-Policy headers with nonce-based
  script allowlisting in the static file serving middleware.

- **Rate limiting**: Add per-IP rate limiting on `/api/auth/login` to prevent
  brute-force attacks. Consider `actix-governor` crate.

- **Session revocation broadcast**: When a user changes their password, broadcast
  a session invalidation event to all connected WebSocket sessions.

## Operations

- **Structured logging**: Replace `println!` calls with `tracing` crate for
  structured, leveled logging with span context.

- **Metrics endpoint**: Add a `/api/metrics` endpoint exposing Prometheus-format
  counters (request count, latency histograms, DB pool stats).

- **Graceful shutdown**: The entrypoint.sh handles SIGTERM, but the Rust server
  should also implement `actix_web::dev::ServerHandle` for graceful in-flight
  request draining.
