# Frequently Asked Questions

Back: [/docs/guides/README.md](/docs/guides/README.md)

## What is kjxlkj?

`kjxlkj` is a Neovim-inspired terminal editor specified in `/docs/` and reconstructed from documentation.

Target behavior is defined in:

- [/docs/spec/README.md](/docs/spec/README.md)

Current verified behavior is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## How is it different from Neovim?

Key project-level differences:

| Aspect | kjxlkj |
|---|---|
| Plugin model | Built-in integrations only (no external plugin loading) |
| Product contract | Documentation-first reconstruction |
| Runtime model | Async task topology defined in architecture spec |

See:

- [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)
- [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)

## What platforms are supported?

Platform targets and limitations depend on the currently reconstructed state.

Use:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Does it support plugins?

No external runtime plugin loading is supported.

- [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)

## How do I configure it?

Configuration behavior is defined in the config feature spec and may be partial depending on current reconstruction.

- [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## How do I install it?

Start here:

- [/docs/guides/INSTALL_WINDOWS.md](/docs/guides/INSTALL_WINDOWS.md)
- [/docs/guides/QUICKSTART.md](/docs/guides/QUICKSTART.md)
