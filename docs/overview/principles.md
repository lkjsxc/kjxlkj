# Design Principles Graph

Back: [/docs/overview/README.md](/docs/overview/README.md)

Visual overview of how design principles relate to each other.

## Principles Topology

```mermaid
graph TD
    Root["Design Principles"]
    
    subgraph CoreValues ["Core Values"]
        Vim["Vim-Compatible"]
        Native["All-Native (No Plugins)"]
        Perf["Performance First"]
        Det["Deterministic"]
    end

    subgraph Interaction ["Interaction Model"]
        Kbd["Keyboard-Only (No Mouse)"]
        Modal["Modal Editing"]
        Prog["Progressive Disclosure"]
    end

    subgraph Visuals ["Visual Identity"]
        Term["Terminal-Native (Square Corners)"]
        Minimal["Minimal Chrome"]
        Semantic["Semantic Coloring"]
    end

    Root --> CoreValues
    Root --> Interaction
    Root --> Visuals

```

## Related

- Overview index: [README.md](README.md)
- Design rationale: [docs/design/README.md](/docs/design/README.md)
