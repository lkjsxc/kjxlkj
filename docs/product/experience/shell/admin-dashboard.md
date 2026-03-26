# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the scalable admin index.
- The dashboard is for search, scan, and entry into note editing, not for large decorative cards.

## Layout

- No all-notes side rail.
- Header contains search, result context, and restrained text actions.
- Main content is a dense paginated list.

## Row Content

- Title.
- Summary preview.
- Created and updated time.
- Visibility state for admins.
- No visible raw IDs in normal rows.

## Visual Rules

- Actions are text-only.
- Search and pagination controls are quieter than note content.
- Empty and no-result states remain compact and factual.
