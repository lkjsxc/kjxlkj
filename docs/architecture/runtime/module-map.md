# Module Map Contract

## Target Module Groups

- `src/main.rs`: binary entrypoint and startup wiring.
- `src/lib.rs`: module exports.
- `src/config.rs`: environment configuration parsing.
- `src/error.rs`: application error taxonomy.
- `src/core/`: domain models and validation.
- `src/storage/`: filesystem-backed persistence adapter.
- `src/web/`: route definitions, handlers, and PostgreSQL auth/session adapter.
- `src/cli/`: docs and quality gate commands.

## File Size Constraint

Every source file stays within 200 lines.
