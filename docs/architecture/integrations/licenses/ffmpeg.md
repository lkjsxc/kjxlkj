# FFmpeg License Notes

## Why This File Exists

- `Dockerfile` installs `ffmpeg` into the runtime image.
- That means the built app image carries extra license obligations beyond the repo's own `Apache-2.0` code.

## License Signals

- FFmpeg upstream documents the project as `LGPL v2.1 or later` by default, with `GPL v2 or later` applying when optional GPL parts are enabled.
- The Alpine `v3.19` `ffmpeg` package that this repo installs advertises the package license expression `GPL-2.0-or-later AND LGPL-2.1-or-later`.
- Treat the package metadata as the packaging-level signal and FFmpeg upstream as the feature-level compliance guide.

## Repo Rules

- Do not describe the shipped runtime image as Apache-only while it contains `ffmpeg`.
- If the runtime image is redistributed, review the exact FFmpeg package build and satisfy the applicable notice and source-offer obligations for that build.
- Keep FFmpeg as an operational dependency only. The repo does not vendor FFmpeg source into the tree.
- If FFmpeg is removed from the runtime image, update this file and the package inventory in the same batch.

## Upstream References

- FFmpeg legal and compliance guide: `https://ffmpeg.org/legal.html`
- Alpine package metadata for the installed package family: `https://pkgs.alpinelinux.org/package/v3.19/community/x86/ffmpeg`
