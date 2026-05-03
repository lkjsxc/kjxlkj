# Create, Update, and Delete Behavior

## Create Note (`POST /{user}/resources/notes`)

- Requires valid session.
- Auto-generates a 26-character opaque `id`.
- Request body must include Markdown `body`.
- Request body may include `alias`, `is_favorite`, and `visibility`.
- Missing `visibility` defaults to the personal-space default.
- Browser note creation seeds `body` with a local-time heading in `YYYY-MM-DD HH:mm`.
- Successful create returns `201` with created resource JSON.
- Creating a note also creates saved snapshot `1`.

## Create Media (`POST /{user}/resources/media`)

- Requires valid session.
- Requires multipart part `file`.
- Optional parts: `alias`, `is_favorite`, and `visibility`.
- The server stores the uploaded binary in SeaweedFS-backed storage and derives media metadata.
- The server prepares derivative WebP metadata only for image and video families.
- The initial Markdown `body` is seeded from the uploaded filename stem as a `# Heading`.
- Successful create returns `201` with created resource JSON.
- Creating media also creates saved snapshot `1`.
- This route is the direct API path for agents and automation rather than the canonical human-facing create screen.

## Attach Media to a Live Note (`POST /{user}/resources/{id}/media-attachments`)

- Requires valid session.
- Applies only to a live `note`.
- Requires one or more multipart `file` parts.
- The request also supplies the current draft `body`, `alias`, `is_favorite`, `visibility`, and the textarea `insert_start` and `insert_end`.
- The browser captures `insert_start` and `insert_end` before opening the file picker.
- The draft `body` is raw Markdown and is not trimmed before insertion or persistence.
- The server validates and processes the entire batch atomically.
- Each selected file creates one new `media` resource.
- Created media store the triggering note as immutable `owner_note_id`.
- Created media inherit the triggering note visibility.
- Created media receive the same derivative preparation as direct media uploads.
- The upload flow does not create generated notes that only link to or embed the media.
- The current note updates by inserting kind-aware Markdown at the supplied selection range in picker order.
- Image attachments insert `![](/<ref>/file)`.
- Video attachments insert `<video controls src="/<ref>/file"></video>`.
- File-family attachments insert `[filename](/<ref>)`.
- If the supplied selection range is stale or invalid for the submitted draft, the current note appends the embeds instead of failing the batch.
- A successful batch creates one new saved snapshot for the current note plus saved snapshot `1` for each newly created media.
- Any file failure aborts the whole batch and leaves the current note unchanged.

## Shared Update (`PUT /{user}/resources/{id}`)

- Requires valid session.
- Applies to both notes and media.
- JSON body contains `body`, `alias`, `is_favorite`, and `visibility`.
- Updates `updated_at`.
- Recomputes derived title, summary, and search fields.
- Creates one new immutable saved snapshot from the post-update live state.

## Public Visibility Control

- The canonical UI control is a checkbox labeled `Public`.
- The visibility control stores `public`, `space`, or `private`.
- Toggling visibility triggers immediate save and immediate chrome refresh.

## Alias and Favorite Rules

- Clearing the alias returns the resource to an ID-only canonical route.
- Alias validity and uniqueness are checked across both notes and media.
- Direct typing must preserve internal `-`, `_`, and `.` separators in the editor field.
- Favorite ordering is shared across both resource kinds.
- `owner_note_id` is immutable once set and direct media uploads leave it empty.

## Delete (`DELETE /{user}/resources/{id}`)

- Requires valid session.
- Performs soft delete on the live resource only.
- Returns `204` with no body.
- Saved snapshots remain immutable and available when their stored visibility allows access.
- The HTML admin rail uses a two-step armed delete control rather than a dialog.
- The first press arms delete for `4` seconds and changes the button copy to require a second press.
- The second press within the armed window issues `DELETE /{user}/resources/{id}`.
- Letting the armed window expire resets the button without network traffic.
- After a successful HTML delete, the admin UI redirects to `/`.
