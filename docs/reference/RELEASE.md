# Release Process

This document describes how to create a new kjxlkj release.

## Version Numbering

We follow Semantic Versioning (semver):
- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

## Pre-Release Checklist

1. **All tests pass (implementation repo)** - Run `make test`

2. **No clippy warnings (implementation repo)** - Run `make clippy`

3. **Documentation up to date**
   - README.md reflects current features
   - CHANGELOG.md updated
   - API docs generated

4. **Version bumped**
   - Update `Cargo.toml` version (implementation repo)
   - Update `CHANGELOG.md` with release date

## Release Steps

### 1. Create Release Branch


### 2. Update Version


### 3. Update CHANGELOG

Move "Unreleased" items under new version heading:


### 4. Commit and Tag


### 5. Push


### 6. Create GitHub Release

- Go to GitHub Releases
- Select the tag
- Add release notes from CHANGELOG
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
