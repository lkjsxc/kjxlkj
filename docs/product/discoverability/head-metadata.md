# Head Metadata Contract

## Shared Head Fields

- Every HTML page emits a document `<title>`.
- Every HTML page emits a viewport meta tag.
- Every HTML page emits the canonical favicon link.
- Every HTML page emits a meta description.
- HTML pages may emit a canonical link only when `public_base_url` is non-blank and valid.
- HTML pages emit robots meta based on page type and SEO-mode state.

## Description Rules

- Homepage uses `site_description`.
- Search uses `site_description`.
- Setup, login, admin, settings, history, snapshot, and not-found pages use concise route-appropriate descriptions.
- Live note pages use the note summary derived from canonical Markdown content.
- Missing or empty live-note summary falls back to `site_description`.

## Canonical Rules

- Canonical links are absolute URLs built from `public_base_url`.
- Homepage canonical is the public root URL.
- Live note canonical uses the public current-note URL and prefers alias routes.
- Search, setup, login, admin, settings, history, snapshot, and not-found pages do not emit canonical links unless the route is meant to be indexed.
- Snapshot URLs never advertise themselves as canonical note URLs.

## Robots Meta Rules

- Homepage is `index,follow` only when `public_base_url` is non-blank and valid.
- Public live note pages are `index,follow` only when `public_base_url` is non-blank and valid.
- Search, setup, login, logout-result pages, admin, settings, history, snapshots, and not-found pages are `noindex,nofollow`.
- When `public_base_url` is blank or invalid, every HTML page becomes `noindex,nofollow`.

## Accessibility Rule

- Hidden shell drawers and overlays must not leave reachable focus targets behind an off-screen or hidden container.
- Hiding navigation with accessibility attributes alone is insufficient when descendants stay focusable.
