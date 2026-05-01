# Runtime Images and Packages

## Direct Container Inputs

| Input | Where Used | Primary License Signal | Repo Handling |
|---|---|---|---|
| `postgres:16-alpine` | compose runtime | PostgreSQL License for PostgreSQL; Alpine packages keep their own licenses | Operational dependency only |
| `chrislusf/seaweedfs:3.97` | compose runtime | `Apache-2.0` for SeaweedFS; bundled packages keep their own licenses | Operational dependency only |
| `rust:1.91-alpine` | build and verify images | Rust components are generally `MIT OR Apache-2.0`; Alpine packages keep their own licenses | Build-time dependency only |
| `alpine:3.19` | app runtime image | package-by-package Alpine licensing | Runtime base image only |
| `mcr.microsoft.com/playwright:v1.54.2-noble` | `visual-verify` image | Playwright image plus bundled browser/vendor licenses | Verification-only dependency |

## Direct System Packages Installed by Repo Dockerfiles

| Package | Dockerfile | License Signal | Notes |
|---|---|---|---|
| `ca-certificates` | `Dockerfile` | distro package licensing | Runtime support package only |
| `curl` | `Dockerfile` | distro package licensing | Runtime support package only |
| `ffmpeg` | `Dockerfile` | see [ffmpeg.md](ffmpeg.md) | Runtime multimedia helper only |
| `postgresql-client` | `Dockerfile.visual` | PostgreSQL License plus distro packaging terms | Verification helper only |

## Rules

- Container images remain multi-license aggregates and must not be described as project-license-only.
- Direct image tag changes must update this inventory when the image family or bundled tooling changes.
- System packages added through repo-owned Dockerfiles must be documented here in the same batch.
