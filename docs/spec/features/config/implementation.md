# Configuration Implementation

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

How the editor loads, merges, and applies configuration.

## File format

All configuration files use TOML format. The main config file is `~/.config/kjxlkj/config.toml`.

## Load order

1. Compiled-in defaults (hardcoded in source)
2. System config (`/etc/kjxlkj/config.toml`, if present)
3. User config (`~/.config/kjxlkj/config.toml`)
4. Project config (`.kjxlkj.toml` in project root)
5. Command-line arguments
6. Runtime `:set` commands

Later sources override earlier ones for the same key.

## Merging

Table values are deep-merged: keys not present in the higher-priority source retain their lower-priority values. Array values are replaced entirely (not appended).

## Config schema

The configuration schema is a Rust struct hierarchy. Each field has a type, default value, and optional constraints.

| Field type | TOML representation |
|---|---|
| boolean | `true` / `false` |
| integer | `42` |
| string | `"value"` |
| array | `["a", "b"]` |
| table | `[section]` with nested keys |

## Reload

| Command | Description |
|---|---|
| `:ConfigReload` | Re-read config files and apply changes |

Changed settings take effect immediately for new operations. Buffer-local settings that were already applied to open buffers are not retroactively changed unless the buffer is re-opened.

## Validation

Invalid configuration values are rejected with a warning notification. The editor continues with the previous valid value for that setting.

| Error | Behavior |
|---|---|
| Unknown key | Warning notification, key ignored |
| Wrong type | Warning notification, default used |
| Out of range | Warning notification, clamped to nearest valid value |
| Parse error | Error notification, entire file skipped |

## Option types

| Type | `:set` syntax | Example |
|---|---|---|
| Boolean | `:set wrap` / `:set nowrap` | Enable/disable wrapping |
| Integer | `:set tabstop=4` | Set tab width |
| String | `:set shell=/bin/zsh` | Set shell path |

## Option scopes

| Scope | Description |
|---|---|
| Global | Applies to entire editor |
| Buffer-local | Per-buffer override (`:setlocal`) |
| Window-local | Per-window override (`:setlocal` for window options) |

## Environment variables

| Variable | Purpose |
|---|---|
| `KJXLKJ_CONFIG` | Override config file path |
| `KJXLKJ_LOG` | Set log level |
| `XDG_CONFIG_HOME` | Override config directory base |
| `XDG_DATA_HOME` | Override data directory base |

## Related

- Project config: [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md)
- After directory: [/docs/spec/features/config/after-dir.md](/docs/spec/features/config/after-dir.md)
- Hooks: [/docs/spec/features/config/hooks-events.md](/docs/spec/features/config/hooks-events.md)
