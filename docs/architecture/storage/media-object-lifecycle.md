# Media Object Lifecycle Contract

## Create

- Uploading new media stores one current object and one saved-snapshot object reference.
- The live media row and snapshot row must agree on the initial file metadata.

## Replace

- Replacing a live media file writes a new current object reference.
- The new saved snapshot stores the post-replacement object reference.
- Earlier snapshots keep their older object references.

## Delete

- Soft-deleting live media does not delete historical snapshot objects immediately.
- Object cleanup may only remove binaries that are no longer referenced by any live resource or snapshot.

## Integrity

- Metadata such as content type, byte size, and checksum must be derived from the stored binary, not from client claims alone.
