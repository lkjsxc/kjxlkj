# Settings API Schema

## Settings Shape

```json
{
  "site_name": "kjxlkj",
  "site_description": "Markdown-first resource system for LLM-operated workflows.",
  "public_base_url": "https://notes.example.com",
  "nostr_names": { "_": "7e7e9c42a91bfef19fa734ae08b1a69f3c4b5f0a74e7a9573c5d4be1f8f7f001" },
  "nostr_relays": ["wss://relay.example.com"],
  "live_default_source": "camera",
  "live_default_camera_facing": "environment",
  "live_default_height": 1080,
  "live_default_fps": 60,
  "live_default_microphone_enabled": false,
  "google_maps_embed_api_key": "",
  "media_webp_quality": 82,
  "default_new_resource_is_private": false
}
```

## Live Defaults

- `live_default_source` is `screen` or `camera`.
- Fresh installs default `live_default_source` to `camera`.
- `live_default_camera_facing` is `environment` for rear camera or `user` for front camera.
- Fresh installs default `live_default_camera_facing` to `environment`.
- `live_default_height` is one of `360`, `480`, `720`, `1080`, `1440`, or `2160`.
- `live_default_fps` is one of `15`, `30`, `45`, `60`, or `120`.
- `live_default_microphone_enabled` controls whether new broadcasts request audio by default.
- Existing `/live` pages may override live defaults for the current page session without persistence.

## Other Settings

- `media_webp_quality` is an integer from `1` through `100`.
- `nostr_names` accepts 64-character hex public keys or `npub...` input and stores lowercase hex.
- `nostr_relays` accepts `wss://` relay URLs.
- `google_maps_embed_api_key` is optional and enables generated Google Maps iframe embeds.
- Blank `google_maps_embed_api_key` disables generated Google Maps embeds.
- Site icon upload requests use `multipart/form-data` rather than JSON.

## Upload Limits

- Media upload limit defaults to `536870912` bytes.
- Site icon upload limit defaults to `2097152` bytes.
- Application-detected oversized multipart payloads return `413` with `payload_too_large` JSON.
- Browser upload clients must tolerate non-JSON limit errors from HTTP middleware or gateways.
- Media upload file parts spill to temporary files while the multipart stream is read.
- SeaweedFS uploads read original media bodies from temporary files rather than cloned memory buffers.
- Image derivative generation may read the source image into memory.
- Video poster generation may invoke server-side FFmpeg against the temporary upload file.
