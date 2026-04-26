# Settings API Schema

## Settings

```json
{
  "site_name": "kjxlkj",
  "site_description": "Markdown-first resource system for LLM-operated workflows.",
  "public_base_url": "https://notes.example.com",
  "nostr_names": { "_": "7e7e9c42a91bfef19fa734ae08b1a69f3c4b5f0a74e7a9573c5d4be1f8f7f001" },
  "nostr_relays": ["wss://relay.example.com"],
  "live_ice_servers": [],
  "live_default_source": "screen",
  "live_default_height": 1080,
  "live_default_fps": 60,
  "live_default_microphone_enabled": false,
  "google_maps_embed_api_key": "",
  "media_webp_quality": 82,
  "default_new_resource_is_private": false
}
```

## Rules

- `media_webp_quality` is an integer from `1` through `100`.
- `nostr_names` accepts 64-character hex public keys or `npub...` input and stores lowercase hex.
- `nostr_relays` accepts `wss://` relay URLs.
- `live_ice_servers` is a JSON array compatible with browser `RTCIceServer[]`.
- Empty `live_ice_servers` disables configured ICE servers.
- External STUN or TURN services are configured only through `Live/ICE_servers_JSON`.
- `live_default_source` is `screen` or `camera`.
- `live_default_height` is one of `360`, `480`, `720`, `1080`, `1440`, or `2160`.
- `live_default_fps` is one of `15`, `30`, `45`, `60`, or `120`.
- `google_maps_embed_api_key` is optional.
- Blank `google_maps_embed_api_key` disables generated Google Maps embeds.

## Nostr Discovery Response

```json
{
  "names": {
    "_": "7e7e9c42a91bfef19fa734ae08b1a69f3c4b5f0a74e7a9573c5d4be1f8f7f001"
  },
  "relays": {
    "7e7e9c42a91bfef19fa734ae08b1a69f3c4b5f0a74e7a9573c5d4be1f8f7f001": [
      "wss://relay.example.com"
    ]
  }
}
```

## Site Icon Response

```json
{
  "configured": true,
  "href": "/assets/site-icon",
  "content_type": "image/png"
}
```

- `configured=false` means the bundled fallback icon is active.
- `POST /admin/site-icon` is admin-only `multipart/form-data` with required part `icon`.
- `POST /admin/site-icon/reset` is admin-only and clears uploaded icon state.
