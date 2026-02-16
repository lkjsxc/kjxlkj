/// kjxlkj-http: HTTP handlers and route wiring.
///
/// Canonical spec: /docs/spec/api/http.md
/// Canonical spec: /docs/spec/api/errors.md
///
/// Route modules are split per resource to stay under 200 lines
/// per /docs/policy/STRUCTURE.md and IMP-STRUCT-01.
pub mod state;
pub mod routes;
pub mod routes_auth;
pub mod routes_automation;
pub mod routes_health;
pub mod routes_note;
pub mod routes_search;
pub mod routes_workspace;
pub mod routes_attachment;
pub mod note_events;
pub mod error_response;
pub mod middleware;
pub mod rate_limit;
pub mod tracing_mw;
pub mod csp;
