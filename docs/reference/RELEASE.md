# Release Process

This document describes how to create a new kjxlkj release.

## Version Numbering

We follow Semantic Versioning (semver):
- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

## Pre-Release Checklist

1. **All tests pass** - Run `cargo test`

2. **No clippy warnings** - Run `cargo clippy --all-targets --all-features -- -D warnings`

3. **Documentation up to date**
   - README.md reflects current features
   - docs/reference/CONFORMANCE.md updated (current surface)
   - docs/reference/LIMITATIONS.md updated (known gaps)

4. **Version bumped**
   - Update workspace `Cargo.toml` version
   - Record release notes in the release artifact (tag/release body)

## Release Steps

### 1. Create Release Branch


### 2. Update Version


### 3. Update Release Notes

Ensure the release notes match the actual shipped surface.


### 4. Commit and Tag


### 5. Push


### 6. Create GitHub Release

- Go to GitHub Releases
- Select the tag
- Add release notes derived from the doc set and the conformance matrix
- GitHub Actions will build and attach binaries

### 7. Merge to Main


## Artifacts

GitHub Actions produces:
- Linux x86_64 binary
- Linux aarch64 binary
- Linux x86_64 AppImage
- Linux aarch64 AppImage
- macOS x86_64 binary
- macOS aarch64 binary
- Windows x86_64 binary
- Windows aarch64 binary

## Post-Release

1. Update Homebrew formula (if applicable)
2. Update AUR package (if applicable)
3. Announce on social channels
4. Start next development cycle
