# Project-Local Configuration

Override global settings for specific projects.

## Overview

kjxlkj supports project-local configuration that
overrides user settings when working in a project.

## Configuration Hierarchy

Priority (highest to lowest):

1. Command line arguments
2. Buffer-local settings
3. Project config (`.kjxlkj.toml`)
4. Workspace config
5. User config (`~/.config/kjxlkj/config.toml`)
6. System defaults

## Project Config File

### Location

Create `.kjxlkj.toml` in project root:


### Example


## Workspace Config

For multi-root workspaces:


## Security

### Trusted Workspaces


### Restricted Settings

Some settings cannot be overridden:

- `shell` - Security risk
- `external_commands` - Security risk

## File Type Overrides

### Per Project


## LSP Configuration

### Project-Specific Servers


## Formatter Settings


## Git Integration

### Auto-Detect

Projects with `.git` auto-enable git features.

### Override


## Ignore Files

### Custom Ignores


## Environment Variables


## Session Settings


## Example: Rust Project


## Example: Web Project

