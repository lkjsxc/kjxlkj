# Object Storage Contract

## Runtime Model

- Media binaries live in one S3-compatible bucket.
- The canonical local and single-host deployment path uses MinIO.
- Runtime code targets an S3-compatible abstraction rather than MinIO-specific APIs.

## Required Configuration

- `S3_ENDPOINT`
- `S3_REGION`
- `S3_BUCKET`
- `S3_ACCESS_KEY`
- `S3_SECRET_KEY`
- `S3_PATH_STYLE`

## Client Rules

- The app must ensure the target bucket exists before serving traffic.
- Uploads must set canonical content type metadata.
- Reads must support normal browser image delivery and video seeking.
- Keys should be opaque and stable enough that snapshots can keep immutable references.
- Derivative reads must use the same visibility rules as the original media object.
