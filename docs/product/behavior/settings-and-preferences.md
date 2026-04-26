# Settings and Preferences Behavior

## Global Settings

- `site_name`, `site_description`, and `public_base_url` remain global.
- `nostr_names` and `nostr_relays` control Nostr identifier discovery.
- `live_ice_servers` controls browser WebRTC ICE server configuration for `/live`.
  - Supports `stun:`, `turn:`, and `turns:` URL schemes.
  - TURN entries should include `username` and `credential`.
  - Fresh installs default to an empty array.
  - External STUN or TURN providers are configured only through `/admin/settings`.
- `live_default_source` controls whether new live pages default to screen or camera capture.
- `live_default_height` and `live_default_fps` control the default live capture quality.
- `live_default_microphone_enabled` controls whether new broadcasts request microphone audio by default.
- `google_maps_embed_api_key` controls generated Google Maps iframe embeds.
- `default_new_resource_is_private` controls both new note and new media defaults.
- `search_results_per_page` still controls the default `/search` page size.
- `media_webp_quality` controls future image WebP and video poster generation quality.
- Uploaded site icon metadata controls favicon and shell icon delivery.
- Home section visibility, ordering, and limits apply to mixed-resource sections.
- Fresh-install home section order is `Recently updated`, `Favorites`, then `Popular`.

## Workspace State

- `/admin/settings` is the canonical admin workspace for global settings plus favorite ordering.
- The page exposes one browser-local settings search that filters visible settings rows without persisting anything.
- Settings render as one flat list of parallel rows instead of visually grouped sections.
- Ordinary scalar settings each own one row.
- Row labels use slash-path names such as `Site_identity/Site_name`.
- Category headings and boxed setting groups are not part of the page language.
- Complex rows are allowed only for list-like settings, ordering controls, action rows, and password fields.
- Reordering home sections changes only the in-form pending state until `Save settings`.
- Home-section ordering uses drag-and-drop only; explicit `Up` and `Down` controls are not part of the contract.
- Favorite ordering and site icon upload/reset persist immediately through their own admin-only requests.
- Dirty settings block same-origin shell navigation, browser back/forward, and full-page unload until the user explicitly discards the pending edits.
- Choosing to stay keeps both the visible page and the browser URL on `/admin/settings`.
- Async favorite reordering and async site icon upload/reset must not discard unsaved main-form edits.

## Immediate Effects

- Saving settings immediately affects `/`, `/search`, `/admin`, new note pages, new media pages, and discovery surfaces.
- Changing `default_new_resource_is_private` affects future creations only.
- Changing `media_webp_quality` affects future uploads only.
- Uploading or resetting the site icon affects subsequent HTML head and shell icon rendering immediately.
- Reordering favorites affects `/`, `/admin`, and `/search?scope=favorites` immediately.
- Changing `public_base_url` affects later canonical URLs, `robots.txt`, and `sitemap.xml` without restart.
- Changing Nostr settings affects later `/.well-known/nostr.json` responses without restart.
- Changing `Live/ICE_servers_JSON`, `Live/Default_source`, `Live/Default_quality`, `Live/Default_fps`, or `Live/Microphone_default` affects newly opened live pages without restart.
- Changing `Embeds/Google_Maps_API_key` affects later note rendering and admin previews without restart.
- Existing `/live` pages may override live defaults for the current page session without persisting those overrides.
