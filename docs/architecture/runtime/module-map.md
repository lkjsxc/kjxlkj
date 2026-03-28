# Module Map Contract

## Target Module Groups

- `src/main.rs`: binary entrypoint and startup wiring.
- `src/lib.rs`: module exports.
- `src/config.rs`: environment configuration parsing.
- `src/error.rs`: application error taxonomy.
- `src/core/`: Markdown rendering and record validation helpers.
- `src/tests/`: Rust test entrypoints defined inside the source tree.
- `src/verify/browser/`: Playwright verification scripts, assertions, and support.
- `src/web/db/`: PostgreSQL-backed persistence adapter and migrations.
- `src/web/handlers/`: HTTP route handlers, including locally served editor asset delivery.
- `src/web/templates/`: authored HTML, CSS, note-shell rendering, history pages, and client-side note behavior.
- `src/web/assets/vendor/`: pinned upstream browser assets and notices.
- `src/web/view.rs`: presentation adapters from database records to templates.
- `src/cli/`: docs and quality gate commands.

## File Size Constraint

Every authored source file stays within 200 lines.
