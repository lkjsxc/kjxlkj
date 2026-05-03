# External URL Embed Contract

## Scope

- External URL embeds apply to absolute `http` and `https` URLs that appear as their own Markdown paragraph.
- A URL paragraph contains one trimmed URL and no other visible text.
- Blank lines or document boundaries separate URL paragraphs from surrounding text.
- URLs inside fenced code, indented code, inline Markdown links, images, or ordinary prose are not external embeds.
- Public rendering must not perform server-side network requests to discover external metadata.
- Admin preview and write paths may fetch and cache metadata under [embed-unfurling.md](embed-unfurling.md).
- The renderer may emit browser-loaded images, bookmark cards, or provider iframes when the provider and URL shape are allowlisted.

## Provider Allowlist

- `kjxlkj`: local or configured public-origin resource pages and file URLs.
- `github.com`: repositories, issues, pull requests, commits, trees, blobs, releases, and discussions.
- Video players: YouTube, Vimeo, TikTok, Dailymotion, and Twitch videos or clips.
- Audio players: Spotify and SoundCloud.
- Map viewers: Google Maps when the persisted Maps Embed API key is non-blank.
- Code and document frames: CodePen and direct PDF-like URLs.
- `x.com` and `twitter.com`: profiles and status URLs.
- `instagram.com`: public posts, reels, and profiles.
- `pixiv.net`: artworks and user pages.
- `bsky.app`: profile and post URLs.
- Mastodon-style ActivityPub web URLs: `@user` profiles and post-like numeric paths on non-denied hosts.
- `npmjs.com`: package pages.
- `crates.io`: crate pages.
- `docs.rs`: crate documentation pages.
- `pypi.org`, `hub.docker.com`, `developer.mozilla.org`, and `gist.github.com`: static rich cards.
- Direct image URLs ending in a browser image extension render as contained images.
- Direct video URLs ending in a browser video extension render as contained native video.
- Direct audio URLs ending in a browser audio extension render as contained native audio.
- Other absolute URLs render as generic external cards.

## Rendering Rules

- Generated external embed HTML is trusted renderer output, not trusted user HTML.
- User-authored `iframe`, `script`, event handlers, and unsafe HTML remain sanitized.
- Metadata bookmark cards show provider/site, title, optional description, optional author, and optional thumbnail.
- Static fallback cards show provider, concise title text derived from the URL, and the canonical URL host/path.
- Static cards open in a new browsing context with `rel="noopener noreferrer"`.
- Provider iframes are lazy-loaded and visually contained by the prose column.
- YouTube embeds use `https://www.youtube-nocookie.com/embed/{video_id}`.
- Vimeo embeds use `https://player.vimeo.com/video/{video_id}`.
- SoundCloud embeds use `https://w.soundcloud.com/player/?url={encoded_url}`.
- Google Maps embeds use the persisted Maps Embed API key and never guess a key.
- Script-backed social embeds render inert blockquote/card markup first, then hydrate through a page-level lazy loader.
- oEmbed, Open Graph, and Twitter Card metadata are cache inputs, not trusted HTML output.
- Direct image embeds use the original image URL as `src` and keep `loading="lazy"`.
- Direct video and audio embeds use native controls and never autoplay.
- Unrecognized or malformed provider-specific URLs fall back to a generic external card.

## Privacy and Failure Rules

- Local private resources may render as rich local cards only for authorized members.
- Local private resources render as ordinary links or generic cards for guests.
- External embeds never mutate stored Markdown.
- If an iframe or image fails to load, the surrounding Markdown output remains stable.
- The admin preview and guest renderer must produce the same external embed shape when the same cache state exists.
