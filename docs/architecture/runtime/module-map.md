# Module Map Contract

## Target Module Groups

- `src/main.rs`: binary entrypoint and startup wiring.
- `src/lib.rs`: module exports.
- `src/config.rs`: environment configuration parsing.
- `src/error.rs`: application error taxonomy.
- `src/core/`: domain models and validation.
- `src/storage/`: PostgreSQL-backed persistence adapter.
- `src/web/`: route definitions, handlers, and templates.
- `src/cli/`: docs and quality gate commands.

## File Size Constraint

Every source file stays within 200 lines.
