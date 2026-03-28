# GitHub Actions Contract

## Workflow Files

- `.github/workflows/ci.yml` is the canonical GitHub automation entrypoint.

## Verify Jobs

- Pull requests run the compose-backed verification bundle.
- Default-branch pushes run the same verification bundle.
- Workflow logic must not replace compose verification with a weaker direct-host shortcut.

## Publish Job

- Runtime image publishing targets `ghcr.io/<owner>/<repo>`.
- Publish runs only after verification passes.
- Default-branch publishes include `latest` and commit-SHA tags.
- Version-tag pushes also publish the git tag as an image tag.
- Verification-only images do not need GHCR publishing.

## Artifacts

- Browser screenshots are uploaded as workflow artifacts.
- Failure logs stay downloadable from the workflow run.
