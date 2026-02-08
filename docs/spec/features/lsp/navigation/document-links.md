# Document Links

Back: [/docs/spec/features/lsp/navigation/README.md](/docs/spec/features/lsp/navigation/README.md)

Clickable links within document content via LSP `textDocument/documentLink`.

## Overview

Document links identify URLs, file paths, and other navigable references within the buffer text. The editor highlights these regions and allows the user to follow them.

## Display

Links are underlined using the `DocumentLink` highlight group. The URL or path is shown in a hover tooltip when the cursor is on the link.

## Following links

| Key | Action |
|---|---|
| `gx` | Open link under cursor (in external browser for URLs, as file in editor for paths) |
| `<C-]>` | Follow link under cursor |

## LSP integration

The editor requests `textDocument/documentLink` when a buffer is opened or modified. The LSP server returns a list of link ranges with their target URIs.

| Field | Type | Description |
|---|---|---|
| `range` | Range | The link span in the document |
| `target` | URI | The link destination |
| `tooltip` | string | Optional tooltip text |

## Link resolution

If the LSP server returns links without targets, the editor sends `documentLink/resolve` to get the target URI before following.

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `links.enabled` | boolean | `true` | Enable document link detection |
| `links.highlight` | boolean | `true` | Underline detected links |

## Related

- Hover: [/docs/spec/features/lsp/hover.md](/docs/spec/features/lsp/hover.md)
- LSP: [/docs/spec/features/lsp/lsp.md](/docs/spec/features/lsp/lsp.md)
