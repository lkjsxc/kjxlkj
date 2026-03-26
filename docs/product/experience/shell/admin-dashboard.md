# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the scalable admin index.
- The dashboard is for search, scan, and entry into note editing, not for large decorative cards.

## Layout

- The persistent shell rail remains visible.
- The rail contains admin search, scope context, recent accessible notes, and restrained text actions.
- Main content is a dense paginated list.
- The main list remains the authoritative browse surface for thousands of notes.

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
