# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the scalable admin browse page.
- The dashboard is for scan, pagination, and entry into note editing.

## Layout

- The persistent shell rail remains visible.
- The rail contains navigation and restrained actions.
- For admins, `New note` sits near the top of the rail rather than below logout or delete actions.
- Main content is a dense paginated list.
- The main list remains the authoritative browse surface for thousands of notes.
- The page header does not show `Admin browse`.
- The page does not expose a top-right search button.

## Row Content

- Title.
- Summary preview.
- Created and updated time.
- Visibility state for admins.
- No visible raw IDs in normal rows.

## Visual Rules

- Actions are text-first.
- Search entry belongs on `/search`, not in the rail.
- Empty states remain compact and factual.
- Explanatory helper blocks such as `Admin index` are omitted.
