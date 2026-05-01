# Embed Unfurling

## Goal

- Standalone URL blocks should render like proper article-site embeds.
- Known media providers become safe players.
- Article, documentation, repository, package, and generic pages become bookmark cards.
- Rendering must never depend on live third-party metadata fetches for public visitors.

## Fetch Timing

- Admin preview may fetch missing or stale metadata.
- Resource create and update may fetch missing or stale metadata after the resource write succeeds.
- Public resource rendering must not make outbound network requests.
- Public resource rendering uses cached metadata or deterministic fallbacks.
- Failed fetches are cached briefly to avoid repeated slow retries.
- External embeds never mutate stored Markdown.

## Metadata Sources

- Provider-specific deterministic parsing wins over fetched metadata for player embeds.
- oEmbed metadata may fill bookmark title, author, site, description, and thumbnail fields.
- Open Graph metadata may fill title, description, site, canonical URL, and image fields.
- Twitter Card metadata may fill missing Open Graph-equivalent fields.
- Plain HTML `<title>` may fill the title only when richer metadata is unavailable.
- The renderer must not trust arbitrary upstream embed HTML.

## Rendering Choice

- Direct image URLs render as contained lazy images.
- Direct video URLs render as contained native `<video controls>`.
- Direct audio URLs render as contained native `<audio controls>`.
- Allowlisted video, audio, map, and code providers render as generated iframes.
- Non-player URLs with metadata render as bookmark cards.
- Non-player URLs without metadata render as deterministic static cards.
- Generic cards open in a new browsing context with `rel="noopener noreferrer"`.

## Privacy And Safety

- Provider players and social scripts load automatically when present on a rendered page.
- Bookmark-card thumbnails use direct remote image URLs.
- Server-side metadata fetching allows only `http` and `https`.
- Server-side metadata fetching blocks localhost, private, link-local, multicast, and unspecified IP targets.
- Server-side metadata fetching uses short timeouts, byte caps, redirect limits, and stable user-agent text.
- Server-side metadata fetching stores only normalized metadata, not raw upstream HTML.
