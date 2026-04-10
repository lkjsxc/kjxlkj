# License Inventory Contract

## Scope

- This inventory covers direct third-party dependencies that this repo names, vendors, or executes directly.
- Transitive dependencies are governed by their upstream manifests and lockfiles and are not duplicated here.
- Container images are multi-license aggregates; this file lists the primary upstream projects and any notice handling this repo must keep.

## Repo License

- The repo itself is licensed under `Apache-2.0`.
- The root `LICENSE` file is the only project license file shipped in the repo tree.
- The repo does not vendor third-party browser editor code.

## Direct Runtime Crates

| Crate | Version | License |
|---|---:|---|
| `actix-multipart` | 0.7.2 | `MIT OR Apache-2.0` |
| `actix-rt` | 2.11.0 | `MIT OR Apache-2.0` |
| `actix-web` | 4.13.0 | `MIT OR Apache-2.0` |
| `ammonia` | 4.1.2 | `MIT OR Apache-2.0` |
| `async-trait` | 0.1.89 | `MIT OR Apache-2.0` |
| `aws-config` | 1.8.15 | `Apache-2.0` |
| `aws-credential-types` | 1.2.14 | `Apache-2.0` |
| `aws-sdk-s3` | 1.129.0 | `Apache-2.0` |
| `base64` | 0.22.1 | `MIT OR Apache-2.0` |
| `bcrypt` | 0.16.0 | `MIT` |
| `chrono` | 0.4.44 | `MIT OR Apache-2.0` |
| `clap` | 4.6.0 | `MIT OR Apache-2.0` |
| `deadpool-postgres` | 0.14.1 | `MIT OR Apache-2.0` |
| `futures-util` | 0.3.32 | `MIT OR Apache-2.0` |
| `image` | 0.25.10 | `MIT OR Apache-2.0` |
| `once_cell` | 1.21.4 | `MIT OR Apache-2.0` |
| `pulldown-cmark` | 0.13.3 | `MIT` |
| `regex` | 1.12.3 | `MIT OR Apache-2.0` |
| `serde` | 1.0.228 | `MIT OR Apache-2.0` |
| `serde_json` | 1.0.149 | `MIT OR Apache-2.0` |
| `sha2` | 0.10.9 | `MIT OR Apache-2.0` |
| `thiserror` | 1.0.69 | `MIT OR Apache-2.0` |
| `tokio` | 1.50.0 | `MIT` |
| `tokio-postgres` | 0.7.16 | `MIT OR Apache-2.0` |
| `tracing` | 0.1.44 | `MIT` |
| `tracing-subscriber` | 0.3.23 | `MIT` |
| `url` | 2.5.8 | `MIT OR Apache-2.0` |
| `uuid` | 1.22.0 | `Apache-2.0 OR MIT` |
| `webp` | 0.3.1 | `MIT OR Apache-2.0` |

## Direct Test Crates

| Crate | Version | License |
|---|---:|---|
| `actix-http` | 3.12.0 | `MIT OR Apache-2.0` |
| `tempfile` | 3.27.0 | `MIT OR Apache-2.0` |

## Verification Tooling

| Component | Version | Where Used | License | Distribution | Notice Rule |
|---|---:|---|---|---|---|
| Playwright | 1.54.2 | `Dockerfile.visual` / `src/verify/browser/` checks | `Apache-2.0` | Installed into the verification image at build time | No repo vendoring; keep version pin explicit |
| Chromium and other browser binaries in the Playwright image | image-provided | `visual-verify` runtime | vendor terms from browser providers | Not vendored in repo | Follow upstream browser terms through the Playwright image |

## Container Image Inputs

| Image | Primary Upstream Project | Primary License Signal | Repo Handling |
|---|---|---|---|
| `postgres:16-alpine` | PostgreSQL server + Alpine base | PostgreSQL server uses the PostgreSQL License; Alpine packages retain their own licenses | Operational dependency only; not vendored in repo |
| `minio/minio:RELEASE.2025-02-18T16-25-55Z` | MinIO object storage server | MinIO server uses `GNU AGPLv3`; bundled packages retain their own licenses | Operational dependency only; not vendored in repo |
| `rust:1.91-alpine` | Rust toolchain + Alpine base | Rust components are generally `MIT OR Apache-2.0`; Alpine packages retain their own licenses | Build-time and verification dependency only; not vendored in repo |
| `alpine:3.19` | Alpine Linux base image | package-by-package upstream licenses | Runtime base image only; not vendored in repo |
| `mcr.microsoft.com/playwright:v1.54.2-noble` | Playwright tooling + browser bundles + Ubuntu base | Playwright is `Apache-2.0`; bundled browsers keep their vendor licenses | Verification image only; not vendored in repo |

## Operational Rules

- New third-party code may not be vendored without adding it to this inventory first.
- Direct dependency version bumps must update this file when the locked version or license expression changes.
- If third-party assets are ever vendored again, they must be pinned and their notice handling must be documented here first.
