# Module Map Contract

## Target Module Groups

- `src/main.rs`: binary entrypoint and startup wiring.
- `src/config.rs`: environment and object-storage configuration parsing.
- `src/core/`: Markdown rendering, embed provider parsing, and shared validation helpers.
- `src/storage/`: SeaweedFS S3 media object client and metadata helpers.
- `src/media/`: media type detection, derivative generation, and derivative metadata helpers.
- `src/web/db/`: grouped PostgreSQL persistence adapter, migrations, and query modules.
- `src/web/handlers/`: grouped HTTP route handlers, including file delivery and media upload paths.
- `src/web/templates/`: grouped HTML templates plus CSS and browser JavaScript assets.
- `src/web/live.rs`: in-memory live broadcast signaling hub.
- `src/verify/browser/`: browser verification scripts and support.
- `src/cli/`: docs and quality gate commands.

## File Size Constraint

- Every authored source file stays within 200 lines.
