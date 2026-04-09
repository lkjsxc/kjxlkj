# Note Media Attachment Contract

## Surface Rules

- Live admin note pages expose `Upload media` beside `Show preview`.
- The control is absent on guest pages and on media edit pages.
- The picker accepts one or more image or video files.
- Upload state and failure messaging stay inside the note editor surface.

## Batch Behavior

- Upload is all-or-nothing per picker submission.
- Selected file order is preserved.
- The server uses the current unsaved note draft supplied by the editor, not a stale last-saved body.
- The submitted draft `body` is preserved byte-for-byte after UTF-8 decoding; trailing spaces and newlines are meaningful note content.
- Each selected file creates one new `media` resource.
- Created media inherit the visibility of the note that triggered the upload.
- Uploads do not create generated notes that only link to or embed the media.
- The currently open note inserts direct embeds for the new media at the submitted textarea caret or selection when that range is valid.
- If the submitted selection range is stale or invalid, the server appends the embeds to the end of the submitted draft instead of failing the batch.
- Inserted embed blocks are separated by blank lines in the same order as the selected files.
- When append fallback is used, the editor shows brief non-error status copy: `Selection changed; inserted at end.`

## Embed Format

- Images insert `![](/<media-ref>/file)`.
- Videos insert `<video controls src="/<media-ref>/file"></video>`.
- `<media-ref>` prefers alias when one exists and otherwise uses the live media `id`.
- Generated notes are not part of the note-editor upload flow.

## Result Shape

- A successful upload updates the current live note and therefore creates one new saved snapshot for that note.
- Each new media resource writes saved snapshot `1`.
- The response reports whether selection fallback was used.
- If any file fails validation or storage, the server creates nothing and updates nothing.
