# Plug Mappings

Plugin-style mapping indirection.

## Overview

`<Plug>` mappings provide named intermediate mappings
for customization. They create a stable action layer
between implementation and user-facing keybindings.

## Purpose

### Indirection Layer

A `<Plug>` mapping is a pseudo-key that represents an
action. The actual implementation is mapped to the
`<Plug>` key, and the user-visible key is mapped to
the `<Plug>` key. This allows users to remap actions
without knowing implementation details.

### Benefits

1. Stable action names across versions
2. User customization without breaking internals
3. Plugin interoperability via named actions
4. Self-documenting keybindings

## Naming Convention

### Standard Format

`<Plug>(ActionName)` - parenthesized action name.
Example: `<Plug>(comment-toggle-line)`.

### Examples

| Plug Mapping | Action |
|-------------|--------|
| `<Plug>(lsp-definition)` | Go to definition |
| `<Plug>(lsp-references)` | Find references |
| `<Plug>(comment-toggle)` | Toggle comment |
| `<Plug>(surround-add)` | Add surrounding |
| `<Plug>(git-blame-line)` | Show line blame |

## Defining Plug Mappings

### Simple Action

In TOML config, plug mappings are defined under
`[plug_mappings]`: the key is the plug name, the value
is the command or action to execute.

### Complex Action

Plug mappings can reference multiple commands chained
together, or call internal Lua/script functions.

### Multi-Key Sequence

A plug mapping can expand to a sequence of normal mode
keys: `<Plug>(select-function) = "vaf"`.

## User Remapping

### Default Bindings

Built-in features define a plug mapping AND a default
user binding. For example:
- `<Plug>(lsp-definition)` is bound to `gd` by default
- Users can remap: `gd = "<Plug>(my-custom-goto)"`

### User Override

To override: set the desired key to the plug mapping name
in `[keys.normal]`. To disable a default plug binding,
map it to `"<Nop>"`.

## Common Patterns

### LSP Actions

| Plug | Default Key |
|------|-------------|
| `<Plug>(lsp-definition)` | `gd` |
| `<Plug>(lsp-references)` | `gr` |
| `<Plug>(lsp-hover)` | `K` |
| `<Plug>(lsp-rename)` | `<Leader>rn` |
| `<Plug>(lsp-code-action)` | `<Leader>ca` |

### Git Actions

| Plug | Default Key |
|------|-------------|
| `<Plug>(git-blame-line)` | `<Leader>gb` |
| `<Plug>(git-hunk-stage)` | `<Leader>gs` |
| `<Plug>(git-hunk-reset)` | `<Leader>gr` |

### Navigation

| Plug | Default Key |
|------|-------------|
| `<Plug>(file-picker)` | `<Leader>ff` |
| `<Plug>(buffer-picker)` | `<Leader>fb` |
| `<Plug>(grep-picker)` | `<Leader>fg` |

### Text Objects

Plug mappings can define custom text objects:
`<Plug>(textobj-function-inner)` mapped to `if`.

## Chaining Plug Mappings

### Sequential

Multiple plug mappings can be chained:
a key can map to a sequence of plug references.

### Conditional

Plug mappings do not support conditions directly.
Use script-level branching instead.

## Mode-Specific Plugs

### Visual Mode

Plug mappings can be mode-specific. Define under
`[keys.visual]` to create visual-mode-only plugs.

### Insert Mode

Insert-mode plug mappings are defined under
`[keys.insert]`. Commonly used for snippet expansion.

## Repeatable Plug

Plug mappings that modify text are automatically
repeatable with `.` (dot repeat) if they use the
standard operator framework.
