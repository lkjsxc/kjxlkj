# Runtime Stack

- Axum for HTTP routing.
- PostgreSQL for live resources, saved snapshots, settings, sessions, search, and analytics.
- SeaweedFS S3 gateway for current and historical media binaries.
- Server-rendered HTML templates with inline CSS and JS.
- Automatic text response compression through Tower HTTP middleware.
- In-process WebRTC relay for `/live` broadcast media.
- Authored CSS remains readable on disk; runtime HTML may inline minified page-specific CSS.
- First-party Markdown body editing for authorized members.
- One sanitized Markdown renderer shared across guest display and admin preview.
- Client-side local-time formatting, drawer behavior, and chrome sync.
