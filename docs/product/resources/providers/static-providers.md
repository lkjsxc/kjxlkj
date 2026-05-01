# Static Provider Contract

## Rules

- Static provider cards never require network fetches during public rendering.
- Cached metadata may upgrade static provider cards into bookmark cards.
- Static fallback cards show provider, concise URL-derived title text, and canonical host/path.
- Static cards open in a new browsing context with `rel="noopener noreferrer"`.
- Static cards are the fallback for malformed rich-provider URLs.

## Providers

- GitHub repository, issue, pull request, commit, tree, blob, release, discussion, and gist URLs.
- npm package pages.
- crates.io crate pages.
- docs.rs crate documentation pages.
- PyPI project pages.
- Docker Hub repository pages.
- MDN documentation pages.
- Pixiv artwork and user pages.
- Mastodon-like profile and post URLs.
- Unknown absolute HTTP or HTTPS URLs.
