# Src Layout Contract

## Top-Level Source Groups

- `src/cli/`: docs and quality gate commands.
- `src/core/`: Markdown rendering and derived-field helpers.
- `src/tests/`: Rust test entrypoints kept inside the authored source tree.
- `src/verify/`: browser verification code and support assets.
- `src/web/`: HTTP, HTML, client JS, CSS, and static assets.

## Web Groups

- `src/web/assets/`: site-owned static assets including icon sources and generated favicon output.
- `src/web/db/auth/`: admin, session, password, and reset persistence.
- `src/web/db/listing/`: list, search, favorites, popularity, and navigation queries.
- `src/web/db/resources/`: shared resource, media, and write-support persistence.
- `src/web/db/history/`: saved-snapshot queries and cursor helpers.
- `src/web/db/settings/`: persisted global settings model and persistence.
- `src/web/db/support/`: migrations and database support files.
- `src/web/handlers/auth/`: setup, login, logout, sessions, and password reset handlers.
- `src/web/handlers/media/`: upload, derivative, and note-attachment handlers.
- `src/web/handlers/resources/`: resource pages, APIs, files, history, previews, and favorites.
- `src/web/handlers/settings/`: settings form and site-icon handlers.
- `src/web/handlers/system/`: home, admin, search, health, assets, discoverability, and HTML response helpers.
- `src/web/templates/shared/`: shell, layout, shared model, sections, and style bundle code.
- `src/web/templates/lists/`: home, admin, search, list, and popularity templates.
- `src/web/templates/resources/`: resource, editor, media, rail, and resource test templates.
- `src/web/templates/settings/`: settings page and setting-row templates.
- `src/web/templates/styles/`: authored CSS included by the style bundle.
- `src/web/templates/scripts/`: authored browser JavaScript included by templates.

## Verification Rules

- Authored Rust, JS, CSS, and Markdown verification code stays under the 200-line authored-file limit.
- Vendored upstream payload inside `src/web/assets/vendor/` is excluded from authored-code refactoring pressure.
- Include paths, Dockerfiles, and compose config must read from `src/` rather than root compatibility folders.
- Bundled third-party license notice text is consolidated at the repo root rather than duplicated under vendored subtrees.
