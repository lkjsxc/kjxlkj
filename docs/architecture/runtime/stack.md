# Runtime Stack

- Actix Web for HTTP routing.
- PostgreSQL for live resources, saved snapshots, settings, sessions, search, and analytics.
- S3-compatible object storage for current and historical media binaries.
- Server-rendered HTML templates with inline CSS and JS.
- Automatic text response compression through Actix middleware.
- Authored CSS remains readable on disk; runtime HTML may inline minified page-specific CSS.
- First-party Markdown body editing for admins.
- One sanitized Markdown renderer shared across guest display and admin preview.
- Client-side local-time formatting, drawer behavior, and chrome sync.
