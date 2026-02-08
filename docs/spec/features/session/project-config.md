# Project Configuration (Session)

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

Per-project settings stored alongside the session.

## Overview

Project configuration allows settings to be scoped to a workspace directory. These override global settings when editing files within the project.

## Project Config File

`.kjxlkj/config.toml` in the project root is loaded on startup when the editor opens that directory.

## Security

Project config files are sandboxed. They cannot:

- Execute arbitrary commands
- Modify global settings
- Access files outside the project

First-time project configs prompt for trust confirmation.

## LSP Configuration

Project-local LSP settings (e.g., rust-analyzer features, Python virtual env path) are defined in the project config.

## Git Integration

Project config can specify Git-related settings like diff algorithm, merge tool, etc.

## Related

- Configuration: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
- Sessions: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Workspace manifest: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
