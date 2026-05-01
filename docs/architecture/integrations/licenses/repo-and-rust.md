# Repo and Rust Licenses

## Scope

- This inventory covers direct third-party dependencies that this repo declares in `Cargo.toml`.
- Transitive crates remain governed by the lockfile and upstream manifests and are not duplicated here.
- The root `LICENSE` file remains the only in-repo project license text.

## Repo License

- The repo itself is licensed under `PolyForm-Noncommercial-1.0.0`.
- Unauthorized commercial use is not permitted.
- Commercial use requires separate written permission from the licensor.
- Noncommercial use, modification, and distribution are governed by the root `LICENSE`.
- The repo does not vendor third-party editor bundles or browser automation libraries into the source tree.
- Direct and transitive third-party dependencies keep their own upstream licenses.

## Direct Runtime Crates

| Crate | Version | License |
|---|---:|---|
| `ammonia` | 4.1.2 | `MIT OR Apache-2.0` |
| `argon2` | 0.5.3 | `MIT OR Apache-2.0` |
| `async-trait` | 0.1.89 | `MIT OR Apache-2.0` |
| `aws-config` | 1.8.15 | `Apache-2.0` |
| `aws-credential-types` | 1.2.14 | `Apache-2.0` |
| `aws-sdk-s3` | 1.129.0 | `Apache-2.0` |
| `axum` | 0.8.8 | `MIT` |
| `base64` | 0.22.1 | `MIT OR Apache-2.0` |
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
| `tower-http` | 0.6.7 | `MIT` |
| `tracing` | 0.1.44 | `MIT` |
| `tracing-subscriber` | 0.3.23 | `MIT` |
| `url` | 2.5.8 | `MIT OR Apache-2.0` |
| `uuid` | 1.22.0 | `Apache-2.0 OR MIT` |
| `webrtc` | 0.17.1 | `MIT OR Apache-2.0` |
| `webp` | 0.3.1 | `MIT OR Apache-2.0` |

## Direct Test Crates

| Crate | Version | License |
|---|---:|---|
| `tempfile` | 3.27.0 | `MIT OR Apache-2.0` |

## Rules

- Direct dependency version bumps must update this inventory when the locked version or license expression changes.
- New vendored third-party source may not land without adding a dedicated license entry first.
