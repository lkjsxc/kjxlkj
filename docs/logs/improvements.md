# Improvement Ideas

Back: [/docs/logs/README.md](/docs/logs/README.md)

## Architecture

- Consider connection pooling tuning for PostgreSQL in single-container model
- Evaluate SQLx compile-time query checking for migration safety
- Cross-actor WS broadcast (currently in-process only, see LIM-WS-BROADCAST-01)
- Add backup restore drill automation (see LIM-OPS-RESTORE-01)

## Documentation

- Add mermaid diagrams for request flow sequences
- Consider adding JSON schema files alongside openapi.yaml
- Document split strategy for files approaching 400+ lines

## Testing

- Explore property-based testing for patch operations
- Consider snapshot testing for API response shapes
- Build integration test harness with real DB (see LIM-TEST-01)
- Add performance benchmarks for hot paths (see LIM-PERF-01)

## Frontend

- Consider splitting index.css into CSS modules per component
- Add E2E tests (Playwright or Cypress) for critical flows
- Evaluate React.lazy() code splitting for LibrarianReview component

## Security

- Evaluate Content-Security-Policy nonce-based script src
- Add rate limiting middleware for auth endpoints
- Consider session revocation broadcast on password change
