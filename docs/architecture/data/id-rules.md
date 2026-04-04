# ID Rules

## Canonical Term

- The canonical note identifier is `id`, not `slug`.

## Format

- `id` is a 128-bit random value.
- Encoding is lowercase Base32 without separators.
- Length is exactly `26` characters.
- Hyphens are never inserted.

## Generation

- IDs are generated from cryptographically secure random bytes.
- IDs are opaque and carry no timestamp or title meaning.
- The system retries on collision before failing creation.
- Current-note IDs and saved-snapshot IDs share one global opaque-ID namespace.

## Validation

- Any malformed `id` is rejected before storage access.
- Validation checks exact length plus lowercase Base32 character set.
- Aliases remain current-note-only and never identify saved snapshots.
