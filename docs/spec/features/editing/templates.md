# Template Files

Create new files from templates.

## Overview

Templates provide boilerplate for new files based
on file type or project structure.

## Template Location

Templates MUST be loaded from these directories (highest
priority first):

1. `.kjxlkj/templates/` -- project-local templates
2. `~/.config/kjxlkj/templates/` -- user templates

Each template file MUST be named `{filetype}.template` (e.g.
`rust.template`, `python.template`). A `default.template` MAY
exist as a fallback for unrecognized file types.

## Creating from Template

### Command

`:New {filename}` -- create a new buffer with the given filename
and populate it from the matching template. The filetype MUST be
inferred from the extension. If no template matches, the buffer
MUST be created empty.

`:NewFromTemplate {template} {filename}` -- create a new buffer
using a specific named template regardless of file extension.

### Keybinding

No default keybinding. Users MAY map via `[keys.normal]` in
`config.toml` (e.g. `"<C-n>" = ":New "`).

## Template Variables

### Substitution

When a template is applied, all `{{variable}}` tokens MUST be
replaced with their resolved values before inserting into the
buffer. Unrecognized variables MUST be left as literal text.

### Built-in Variables

| Variable | Value |
|----------|-------|
| `{{filename}}` | File name with extension |
| `{{basename}}` | File name without extension |
| `{{date}}` | Current date (YYYY-MM-DD) |
| `{{year}}` | Current four-digit year |
| `{{author}}` | From `[user]` config section |
| `{{project}}` | Project name from nearest `.kjxlkj.toml` |
| `{{path}}` | Full path of the new file |
| `{{cursor}}` | Removed on insert; cursor placed here |

## Configuration

### User Info

In `~/.config/kjxlkj/config.toml`, define a `[user]` table with `name` and `email` keys. `{{author}}` resolves to the `name` value. If absent, falls back to `$USER`.

### Default Templates

The `[templates]` config section has an `auto_apply` boolean (default `false`). When `true`, opening a new file via `:edit newfile.rs` MUST auto-populate from the matching template. When `false`, templates only apply via `:New`.

## Project Templates

### .kjxlkj.toml

The project-local `.kjxlkj.toml` SHOULD have a `[templates]` table with `directory` (default `.kjxlkj/templates`), `auto_apply` (boolean), and a `[templates.variables]` sub-table defining custom variables like `license = "MIT"`. Custom variables are available as `{{license}}`, `{{org}}`, etc.

### Project-Specific

Project templates in `.kjxlkj/templates/` MUST override user
templates when the same filetype matches.

## Interactive Templates

### Prompts

Templates MAY include `{{prompt:Label}}` variables. The editor
MUST display an input prompt with the given label and substitute
the user's response.

### Template Content

A template body MAY contain `{{prompt:Label}}` tokens. The editor MUST prompt once per unique label and reuse the value for duplicate labels in the same template.

Example body: a line `// Module: {{prompt:Module name}}` followed by `pub mod {{prompt:Module name}} { {{cursor}} }`.

## Example Templates

| Template file | Language | Description |
|---|---|---|
| `rust.template` | Rust | Contains copyright header with `{{basename}}`, `{{year}}`, `{{author}}`, a `fn main()` body, and `{{cursor}}` inside |
| `python.template` | Python | Contains shebang `#!/usr/bin/env python3`, docstring with `{{basename}}`, `def main()`, `if __name__` guard, and `{{cursor}}` |
| `typescriptreact.template` | React/TSX | Contains import, interface `{{basename}}Props`, functional component `{{basename}}`, and `{{cursor}}` |

## Conditional Content

### If Blocks

`{{#if variable}}...{{/if}}` blocks MUST include content only when the variable is defined and non-empty. For example, `{{#if license}} // SPDX-License-Identifier: {{license}} {{/if}}` inserts the license header only when the `license` variable is set.

## Template Discovery

### List Templates

`:Templates` -- list all available templates showing filetype,
source path, and auto-apply status.

## Best Practices

1. Include license headers
2. Add documentation stubs
3. Use consistent naming
4. Keep templates updated

# Abbreviations

Text abbreviations that expand automatically. See
[/docs/spec/modes/insert/completion/insert-abbreviations.md]
for full abbreviation specification.
