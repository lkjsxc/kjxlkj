# Editor Model

Core editor state shared across all features.

## Documents

| Document | Content |
|----------|---------|
| [buffers.md](buffers.md) | Buffer management and rope structure |
| [windows.md](windows.md) | Window system and layout |

## Overview

The editor core is a single-writer task that owns:

- **Buffer List** - All open text buffers (rope-based)
- **Window Tree** - Hierarchical split layout
- **Mode State** - Current editing mode
- **Jumplist** - Navigation history
- **Registers** - Text storage

## Architecture Context

- Runtime: [docs/spec/architecture/runtime.md](docs/spec/architecture/runtime.md)
- System shape: [docs/spec/architecture/README.md](docs/spec/architecture/README.md)

## Related

- Features: [docs/spec/features/README.md](docs/spec/features/README.md)
- Modes: [docs/spec/modes/README.md](docs/spec/modes/README.md)
