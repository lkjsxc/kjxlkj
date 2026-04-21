# Module Map Contract

## Target Module Groups

- `src/main.rs`: binary entrypoint and startup wiring.
- `src/config.rs`: environment and object-storage configuration parsing.
- `src/core/`: Markdown rendering and shared validation helpers.
- `src/storage/`: SeaweedFS S3 media object client and metadata helpers.
- `src/media/`: media type detection, derivative generation, and derivative metadata helpers.
- `src/web/db/`: PostgreSQL-backed persistence adapter and migrations.
- `src/web/handlers/`: HTTP route handlers, including file delivery and media upload paths.
- `src/web/templates/`: authored HTML, CSS, note/media shell rendering, and client-side behavior.
- `src/web/live.rs`: in-memory live broadcast signaling hub.
- `src/verify/browser/`: browser verification scripts and support.
- `src/cli/`: docs and quality gate commands.

## File Size Constraint

- Every authored source file stays within 200 lines.
