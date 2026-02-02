# Editor Comparison

Comparing kjxlkj with other terminal editors.

## Feature Matrix

| Feature | kjxlkj | Neovim | Helix | Kakoune |
|---------|--------|--------|-------|---------|
| Modal Editing | ✓ | ✓ | ✓ | ✓ |
| Built-in LSP | ✓ | Plugin | ✓ | Plugin |
| Tree-sitter | ✓ | Plugin | ✓ | ✗ |
| Fuzzy Finder | ✓ | Plugin | ✓ | Plugin |
| File Explorer | ✓ | Plugin | ✓ | ✗ |
| Git Integration | ✓ | Plugin | ✓ | Plugin |
| Terminal | ✓ | ✓ | ✗ | ✗ |
| Multiplexing | ✓ | ✗ | ✗ | ✗ |

## Keybinding Philosophy

### kjxlkj / Neovim

Vim-style: Verb → Motion


### Helix / Kakoune

Selection-first: Select → Act


## Configuration

| Editor | Config Format | Scripting |
|--------|---------------|-----------|
| kjxlkj | TOML | None |
| Neovim | Lua | Full |
| Helix | TOML | None |
| Kakoune | Custom | Shell |

## Startup Performance

Approximate cold start times:

| Editor | Minimal | Typical |
|--------|---------|---------|
| kjxlkj | ~10ms | ~15ms |
| Neovim | ~50ms | ~200ms+ |
| Helix | ~20ms | ~30ms |
| Kakoune | ~10ms | ~20ms |

## Memory Usage

Opening 10 small files:

| Editor | RAM |
|--------|-----|
| kjxlkj | ~15MB |
| Neovim | ~50MB |
| Helix | ~30MB |
| Kakoune | ~20MB |

## Why Choose kjxlkj

### Over Neovim

- Batteries included (no plugin config)
- Faster startup
- Simpler configuration
- Single binary

### Over Helix

- Vim keybindings (familiar)
- Built-in terminal
- More extensible config

### Over Kakoune

- Vim keybindings
- Built-in features
- Easier to get started

## Trade-offs

### kjxlkj Advantages

- Zero configuration to start
- No plugin management
- Consistent experience
- Fast and light

### kjxlkj Disadvantages

- Less extensible than Neovim
- Smaller community
- Fewer edge-case features

## Ideal Users

| User Type | Recommended |
|-----------|-------------|
| Vim veterans wanting simplicity | kjxlkj |
| Heavy customizers | Neovim |
| Modern modal learners | Helix |
| Minimalists | Kakoune |

## Migration Paths

### From Vim/Neovim

Easiest. Same keybindings, TOML config.

### From Helix

Medium. Learn vim keybindings.

### From Kakoune

Medium. Different paradigm.

### From VSCode/etc

Learn modal editing first.
