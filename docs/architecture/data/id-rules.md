# ID Rules

## Canonical Term

- The canonical note identifier is `id`, not `slug`.

## Format

- `id` is a 128-bit random value.
- Encoding is Base64URL without padding.
- Length is exactly `22` characters.
- Hyphens are never inserted.

## Generation

- IDs are generated from cryptographically secure random bytes.
- IDs are opaque and carry no timestamp or title meaning.
- The system retries on collision before failing creation.

## Validation

- Any malformed `id` is rejected before storage access.
- Validation checks exact length plus Base64URL character set.
