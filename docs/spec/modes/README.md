# Modes

Back: [/docs/spec/README.md](/docs/spec/README.md)
Modes are deterministic state machines driving intent generation.

## Mode Transition Diagram

```mermaid
stateDiagram-v2
    [*] --> Normal
    Normal --> Insert: i, a, o, O, A, I
    Normal --> Visual: v, V, Ctrl-v
    Normal --> Command: colon, /, ?
    Normal --> Replace: R
    Normal --> OperatorPending: d, c, y, >, <, =
    Normal --> TerminalInsert: i (terminal window)
    Insert --> Normal: Esc
    Insert --> InsertNormal: Ctrl-O
    InsertNormal --> Insert: after command
    Visual --> Normal: Esc, operator
    Command --> Normal: Esc, Enter
    Replace --> Normal: Esc
    OperatorPending --> Normal: motion/Esc
    TerminalInsert --> Normal: Ctrl-\ Ctrl-n
```

## Directory Structure

| Directory | Content |
|-----------|---------|
| [insert/](insert/README.md) | Insert mode |
| [replace/](replace/README.md) | Replace mode |

## Core Mode Documents

| Document | Content |
|----------|---------|
| [normal.md](normal.md) | Normal mode |
| [visual.md](visual.md) | Visual mode |
| [command.md](command.md) | Command mode |
| [transitions.md](transitions.md) | Mode transitions |
| [configuration.md](configuration.md) | Mode configuration |

## Required Modes

| Mode | Purpose |
|---|---|
| Normal | Navigation, operators, composition |
| Insert | Text entry, completion |
| Visual(Char/Line/Block) | Selection with three sub-modes |
| Command(Ex/Search) | Ex commands (`:`) and search (`/`, `?`) |
| Replace | Overwrite semantics |
| OperatorPending | Awaiting motion after operator key (d, c, y, etc.) |
| TerminalInsert | Forwarding input to PTY in terminal windows |
| InsertNormal | Single normal command via `Ctrl-O` from insert |

## Cross-cutting Rules

| Rule | Requirement |
|------|-------------|
| Intent emission | Modes emit intents; core applies them |
| Predictable escape | Esc returns to Normal |
| No blocking | Transitions never wait on services |

## Related

- Editing: [docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Commands: [docs/spec/commands/README.md](/docs/spec/commands/README.md)
