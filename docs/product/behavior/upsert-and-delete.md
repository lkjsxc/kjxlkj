# Create, Update, and Delete Behavior

## Create Note (`POST /resources/notes`)

- Requires valid session.
- Auto-generates a 26-character opaque `id`.
- Request body must include Markdown `body`.
- Request body may include `alias`, `is_favorite`, and `is_private`.
- Browser note creation seeds `body` with a local-time heading in `YYYY-MM-DD HH:mm`.
- Successful create returns `201` with created resource JSON.
- Creating a note also creates saved snapshot `1`.

## Create Media (`POST /resources/media`)

- Requires valid session.
- Requires multipart part `file`.
- Optional parts: `alias`, `is_favorite`, and `is_private`.
- The server stores the uploaded binary in S3-compatible storage and derives media metadata.
- The initial Markdown `body` is seeded from the uploaded filename stem as a `# Heading`.
- Successful create returns `201` with created resource JSON.
- Creating media also creates saved snapshot `1`.
- This route is the direct API path for agents and automation rather than the canonical human-facing create screen.

## Attach Media to a Live Note (`POST /resources/{id}/media-attachments`)

- Requires valid session.
- Applies only to a live `note`.
- Requires one or more multipart `file` parts.
- The request also supplies the current draft `body`, `alias`, `is_favorite`, `is_private`, and the textarea `insert_start` and `insert_end`.
- The draft `body` is raw Markdown and is not trimmed before insertion or persistence.
- The server validates and processes the entire batch atomically.
- Each selected file creates one new `media` resource and one new background `note`.
- The created media and generated background notes inherit the triggering note visibility.
- The current note updates by inserting direct media embeds at the supplied selection range in picker order.
- If the supplied selection range is stale or invalid for the submitted draft, the current note appends the embeds instead of failing the batch.
- A successful batch creates one new saved snapshot for the current note plus saved snapshot `1` for each newly created media and background note.
- Any file failure aborts the whole batch and leaves the current note unchanged.

## Shared Update (`PUT /resources/{id}`)

- Requires valid session.
- Applies to both notes and media.
- JSON body contains `body`, `alias`, `is_favorite`, and `is_private`.
- Updates `updated_at`.
- Recomputes derived title, summary, and search fields.
- Creates one new immutable saved snapshot from the post-update live state.

## Public Visibility Control

- The canonical UI control is a checkbox labeled `Public`.
- Checked maps to `is_private = false`.
- Unchecked maps to `is_private = true`.
- Toggling visibility triggers immediate save and immediate chrome refresh.

## Alias and Favorite Rules

- Clearing the alias returns the resource to an ID-only canonical route.
- Alias validity and uniqueness are checked across both notes and media.
- Direct typing must preserve internal `-`, `_`, and `.` separators in the editor field.
- Favorite ordering is shared across both resource kinds.

## Delete (`DELETE /resources/{id}`)

- Requires valid session.
- Performs soft delete on the live resource only.
- Returns `204` with no body.
- Saved snapshots remain immutable and available when their stored visibility allows access.
