# Nostr Identifier Discovery

## Route

- `GET /.well-known/nostr.json` provides NIP-05-style identity discovery.
- The endpoint is public.
- The endpoint returns `application/json`.
- The endpoint sets `Access-Control-Allow-Origin: *`.
- The endpoint never redirects.

## Settings Ownership

- `Nostr/Names_JSON` owns local-name to public-key mappings.
- `Nostr/Relays_JSON` owns the global relay list.
- Settings are edited from `/admin/settings`.
- Nostr discovery is identity discovery only and does not change admin sign-in.

## Names

- `Nostr/Names_JSON` stores a JSON object.
- Keys are local names such as `_`, `alice`, or `team.ops`.
- Local names are case-insensitive and stored lowercase.
- Local names may contain ASCII letters, digits, `-`, `_`, and `.`.
- Values may be 64-character hex public keys or `npub...` keys.
- Stored and emitted public keys are lowercase hex.

## Relays

- `Nostr/Relays_JSON` stores a JSON array of relay URLs.
- Relay URLs must use `wss://`.
- The same relay list applies to every configured public key.
- Empty relay configuration omits the `relays` field.

## Response Rules

- Missing `name` returns all configured names.
- Known `name` returns only that name in `names`.
- Unknown or invalid `name` returns `200` with an empty `names` object.
- When relays are configured, each returned public key maps to the full relay list.

## Response Shape

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
