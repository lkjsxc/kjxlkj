# Frame Provider Contract

## Rules

- Frame providers render only from allowlisted URL shapes.
- Frame `src` values are constructed by the renderer, never copied blindly from user HTML.
- Frames use `loading="lazy"`.
- Frames stay inside the prose column.
- Frames must not autoplay.
- Provider-specific parse failures fall back to static external cards.

## Providers

- YouTube videos, shorts, embeds, and playlists render through `youtube-nocookie.com`.
- Vimeo numeric video URLs render through `player.vimeo.com`.
- TikTok public video URLs render through TikTok embed/player URLs.
- Dailymotion public video URLs render through Dailymotion embed URLs.
- Twitch videos and clips render only when the configured public origin can provide the required `parent`.
- Spotify tracks, albums, playlists, episodes, shows, and artists render through Spotify embed URLs.
- SoundCloud public URLs render through the SoundCloud widget URL with the authored URL encoded.
- CodePen pens render through CodePen embed URLs.
- Google Maps place/search URLs render only when `google_maps_embed_api_key` is non-blank.
- Direct PDF-like URLs may render in a browser frame.

## Google Maps

- `google_maps_embed_api_key` is stored in app settings.
- A blank Maps key disables generated Google Maps iframes.
- Blank-key Google Maps URLs fall back to static cards.
- The key is not treated as a secret because Maps embeds require the key in browser-visible URLs.
