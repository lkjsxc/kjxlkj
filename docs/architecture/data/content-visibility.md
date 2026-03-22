# Content Visibility Contract

## Visibility Rule

- `private: true` marks content as private.
- Missing `private` is treated as private (`true`).
- Privacy enforcement requirements are canonical in [../../product/policies/privacy.md](../../product/policies/privacy.md).

## Public Surface Behavior

- Public listings exclude private content for logged-out users.
- Direct requests for private slugs return not found when logged out.

## Admin Surface Behavior

- Authenticated admin can list, open, and mutate both public and private content.
- Public article views do not show author attribution/byline metadata.
