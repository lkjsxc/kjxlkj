# Media Object Lifecycle Contract

## Create

- Uploading new media stores one current object and one saved-snapshot object reference.
- Uploading new media may store WebP derivative objects beside the current object.
- The live media row and snapshot row must agree on the initial file metadata.
- The live media row and snapshot row must agree on initial derivative metadata.

## Replace

- Replacing a live media file writes a new current object reference.
- The new saved snapshot stores the post-replacement object reference.
- Earlier snapshots keep their older object references.
- Earlier snapshots keep their older derivative object references.

## Delete

- Soft-deleting live media does not delete historical snapshot objects immediately.
- Object cleanup may only remove binaries that are no longer referenced by any live resource or snapshot.
- Object cleanup treats original and derivative objects the same.

## Integrity

- Metadata such as content type, byte size, and checksum must be derived from the stored binary, not from client claims alone.
