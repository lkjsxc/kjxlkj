# Script-Local Mappings

SID (Script ID) namespacing for keybinding modules.

## Purpose

`<SID>` prefixed mappings provide namespace isolation between configuration modules. Each module (TOML config file) receives a unique script ID, so internal helper mappings never collide across modules.

## Mechanism

When a module defines a mapping with `<SID>`, the editor replaces `<SID>` with a unique numeric prefix at load time. Two modules can both define `<SID>helper` without conflict because they resolve to different internal names.

## Public vs Private

| Prefix | Visibility | Purpose |
|---|---|---|
| `<SID>name` | Module-private | Internal helper mappings |
| `<Plug>(name)` | User-visible | Public mappings for user remapping |

### Private (SID)

Used inside a module for chaining or internal dispatch. Not accessible to users.

### Public (Plug)

Exposed via `<Plug>` for user remapping. The module maps `<Plug>(ModuleName)` to an internal `<SID>` handler.

## Module Organization

Each module (e.g., `git.toml`, `lsp.toml`) is loaded with its own script context. Mappings defined inside use `<SID>` for private helpers and `<Plug>` for the public interface.

## Advantages

- **No collisions**: Multiple modules can use same internal names
- **Clean namespace**: Only `<Plug>` mappings visible to users
- **Refactoring**: Change internal implementation without affecting public interface

## Debugging

Use `:map <SID>` to list all script-local mappings. Use `:scriptnames` to see script IDs and their source files.

## Related

- Expression mappings: [/docs/spec/scripting/mappings/expr-mappings.md](/docs/spec/scripting/mappings/expr-mappings.md)
- Operator mappings: [/docs/spec/scripting/mappings/operator-mappings.md](/docs/spec/scripting/mappings/operator-mappings.md)
