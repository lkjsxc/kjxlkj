# Social Provider Contract

## Rules

- Social providers that require JavaScript render deterministic placeholder markup first.
- The page-level embed hydrator loads provider scripts lazily.
- Hydration runs after initial page load, partial shell navigation, and admin preview replacement.
- Hydration failure leaves the placeholder card usable as a normal external link.
- Server-side rendering never fetches oEmbed or social metadata.

## Providers

- X and Twitter status URLs render as hydrated post placeholders.
- X and Twitter profile URLs render as hydrated timeline/profile placeholders when supported by the provider script.
- Instagram posts, reels, and profiles render as Instagram embed placeholders.
- Bluesky post URLs render as Bluesky embed placeholders.
- Mastodon-like URLs render static cards unless a future provider has deterministic client hydration.

## Privacy

- Social scripts are loaded only when the rendered page contains a matching placeholder.
- Social placeholders use the original URL as the visible fallback link.
- Social embeds must not load hidden tracking if no matching authored URL exists.
