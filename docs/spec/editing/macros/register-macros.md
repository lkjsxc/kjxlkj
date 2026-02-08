# Register Macros

Back: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)

How macros use registers for storage.

## Overview

Macros are stored in named registers (`a`-`z`). The register content is the exact sequence of keystrokes recorded during macro recording.

## Recording

`q{a-z}` starts recording into the named register. `q` stops recording. The register content is overwritten.

## Appending

`q{A-Z}` appends to the existing register content. This allows building macros incrementally.

## Storing

Macro content can be manually set by yanking text into a register:

`"ayy` — the text of the line becomes the macro content of register `a`.

## Viewing

`:registers a` or `"ap` — view the content of register `a`.

## Playback

`@a` — execute the keystrokes stored in register `a`.

`@@` — replay the last executed macro register.

## Shared Storage

Macros and yank/delete registers share the same storage. Yanking into `"a` overwrites any macro stored there. Use dedicated registers for macros.

## Related

- Macros: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)
- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- Advanced macros: [/docs/spec/editing/macros/macros-advanced.md](/docs/spec/editing/macros/macros-advanced.md)
