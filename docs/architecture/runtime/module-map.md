# Module Map Contract

## Target Module Groups

- `src/main.rs`: binary entrypoint and startup wiring.
- `src/lib.rs`: module exports.
- `src/config.rs`: environment configuration parsing.
- `src/error.rs`: application error taxonomy.
- `src/core/`: Markdown rendering and record validation helpers.
- `src/web/db/`: PostgreSQL-backed persistence adapter and migrations.
- `src/web/handlers/`: HTTP route handlers, including editor asset delivery.
- `src/web/templates/`: HTML, CSS, and client-side note behavior.
- `src/web/view.rs`: presentation adapters from database records to templates.
- `src/cli/`: docs and quality gate commands.
- `visual/`: browser verification scripts and assertions.

## File Size Constraint

Every authored source file stays within 200 lines.
