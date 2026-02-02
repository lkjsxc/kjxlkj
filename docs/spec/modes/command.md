# Command mode
Command mode is the ex-style command line interface.

## Requirements
- Parsing is core-owned and deterministic.
- Executing a command produces typed intents.
- IO-heavy commands delegate to services and surface progress/cancellation.

## Command line state

- Entering command mode MUST present a fresh prompt with an empty input buffer.
- The command-line cursor starts at position 0.
- The previous command text MUST NOT be implicitly reused or appended-to.

## Entry points

- `:` ex commands
- `/` forward search
- `?` backward search

## Related

- Command system: [docs/spec/commands/README.md](docs/spec/commands/README.md)
- Search UI: [docs/spec/features/navigation/finder.md](docs/spec/features/navigation/finder.md)
