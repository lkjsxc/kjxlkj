# Call Hierarchy

Back: [/docs/spec/features/lsp/navigation/README.md](/docs/spec/features/lsp/navigation/README.md)

View incoming and outgoing calls for a function.

## Overview

Call hierarchy shows which functions call the current function (incoming) and which functions are called by the current function (outgoing).

## Usage

| Command | Description |
|---|---|
| `:CallHierarchyIncoming` | Show incoming calls |
| `:CallHierarchyOutgoing` | Show outgoing calls |

## Incoming Calls

Lists all functions/methods that call the function under the cursor. Each entry shows the caller's name, file, and line.

## Outgoing Calls

Lists all functions/methods called by the function under the cursor.

## Navigation

Select an entry and press `<CR>` to jump to it. The hierarchy can be expanded to show deeper levels.

## LSP Requirements

Uses `textDocument/prepareCallHierarchy`, `callHierarchy/incomingCalls`, and `callHierarchy/outgoingCalls` LSP methods.

## Related

- LSP: [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)
- Type hierarchy: [/docs/spec/features/lsp/navigation/type-hierarchy.md](/docs/spec/features/lsp/navigation/type-hierarchy.md)
