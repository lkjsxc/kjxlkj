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
- Each selected file creates one new `media` resource and one new background `note`.
- Both created resources inherit the visibility of the note that triggered the upload.
- Background notes start with a filename-derived `# Heading` followed by the canonical embed for their media.
- The currently open note inserts direct embeds for the new media at the current textarea caret or selection.
- Inserted embed blocks are separated by blank lines in the same order as the selected files.

## Embed Format

- Images insert `![](/<media-ref>/file)`.
- Videos insert `<video controls src="/<media-ref>/file"></video>`.
- `<media-ref>` prefers alias when one exists and otherwise uses the live media `id`.
- Generated background notes use the same embed body rules as the current-note insertion path.

## Result Shape

- A successful upload updates the current live note and therefore creates one new saved snapshot for that note.
- Each new media resource writes saved snapshot `1`.
- Each generated background note writes saved snapshot `1`.
- If any file fails validation or storage, the server creates nothing and updates nothing.
