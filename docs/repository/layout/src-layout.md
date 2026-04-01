# Src Layout Contract

## Top-Level Source Groups

- `src/cli/`: docs and quality gate commands.
- `src/core/`: Markdown rendering and derived-field helpers.
- `src/tests/`: Rust test entrypoints kept inside the authored source tree.
- `src/verify/`: browser verification code and support assets.
- `src/web/`: HTTP, HTML, client JS, CSS, and static assets.

## Web Groups

- `src/web/assets/vendor/toastui/3.2.2/`: pinned upstream editor distribution plus `LICENSE` and provenance.
- `src/web/db/`: PostgreSQL adapter, migrations, and query modules.
- `src/web/handlers/`: route handlers and static-asset delivery.
- `src/web/templates/`: authored HTML/CSS/JS/template support modules.

## Verification Rules

- Authored Rust, JS, CSS, and Markdown verification code stays under the 200-line authored-file limit.
- Vendored upstream payload inside `src/web/assets/vendor/` is excluded from authored-code refactoring pressure.
- Include paths, Dockerfiles, and compose config must read from `src/` rather than root compatibility folders.
