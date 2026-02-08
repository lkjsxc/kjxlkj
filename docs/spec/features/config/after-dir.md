# After Directory

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Post-processing configuration loaded after all other configuration sources.

## Overview

The `after/` directory is a configuration directory that is loaded after all other configuration. It allows the user to override or augment settings established by filetype detection, syntax files, and other configuration sources.

## Location

| Path | Purpose |
|---|---|
| `~/.config/kjxlkj/after/` | User after-directory |
| `.kjxlkj/after/` | Project after-directory |

## Load order

1. System defaults
2. User config (`~/.config/kjxlkj/config.toml`)
3. Project config (`.kjxlkj.toml`)
4. Filetype detection and filetype-specific settings
5. After-directory scripts (loaded last, can override everything above)

## Directory structure

| Path | Purpose |
|---|---|
| `after/ftconfig/` | Per-filetype setting overrides |
| `after/syntax/` | Syntax highlighting overrides |
| `after/indent/` | Indentation rule overrides |

## Ftconfig overrides

Files in `after/ftconfig/` are named by filetype (e.g., `after/ftconfig/rust.toml`). They are loaded after the default filetype configuration and override matching settings.

## Syntax overrides

Files in `after/syntax/` can extend or replace highlight groups for a filetype. They are loaded after the default syntax definitions.

## Indent overrides

Files in `after/indent/` can override indentation rules for a filetype. They are loaded after the default indentation configuration.

## Use cases

| Use case | Example |
|---|---|
| Override tab width for Rust | `after/ftconfig/rust.toml` with `tab_width = 2` |
| Add custom highlight groups | `after/syntax/python.toml` with additional groups |
| Fix indentation for a language | `after/indent/html.toml` with custom rules |

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `config.after_dir` | string | `~/.config/kjxlkj/after/` | Path to after-directory |
| `config.load_after` | boolean | `true` | Enable after-directory loading |

## Related

- Configuration: [/docs/spec/features/config/implementation.md](/docs/spec/features/config/implementation.md)
- Filetype config: [/docs/spec/features/config/ftconfig.md](/docs/spec/features/config/ftconfig.md)
- Filetype detection: [/docs/spec/features/config/filetype.md](/docs/spec/features/config/filetype.md)
