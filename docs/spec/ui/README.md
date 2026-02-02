# UI Architecture

## Documents

| Document | Content |
|----------|---------|
| [components.md](components.md) | UI components |
| [views.md](views.md) | View system |
| [themes.md](themes.md) | Theme system |

## Map

```mermaid
graph TD
  UI[ui]
  UI --> C[components]
  UI --> V[views]
  UI --> T[themes]

  C --> CC[components.md]
  V --> VV[views.md]
  T --> TT[themes.md]
```
