# Object Storage Contract

## Runtime Model

- Media binaries live in one SeaweedFS bucket exposed through its S3 gateway.
- The canonical local and single-host deployment path uses one SeaweedFS container.
- Runtime code targets the S3 protocol and does not call SeaweedFS native filer APIs.

## Required Configuration

- `SEAWEEDFS_S3_ENDPOINT`
- `SEAWEEDFS_S3_REGION`
- `SEAWEEDFS_S3_BUCKET`
- `SEAWEEDFS_S3_ACCESS_KEY`
- `SEAWEEDFS_S3_SECRET_KEY`
- `SEAWEEDFS_S3_PATH_STYLE`

## Client Rules

- The app must ensure the target bucket exists before serving traffic.
- Bucket initialization must tolerate SeaweedFS startup lag and retry until the S3 gateway is actually ready, without requiring a manual app restart.
- Uploads must set canonical content type metadata.
- Reads must support normal browser image delivery and video seeking.
- Keys should be opaque and stable enough that snapshots can keep immutable references.
- Derivative reads must use the same visibility rules as the original media object.
