# Workspace Suite UX

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Required Modules

| Module | Required behavior |
|---|---|
| Saved views | persist/search/filter presets |
| Command palette | keyboard-first create/open/move/tag actions |
| Graph explorer | backlink neighborhood navigation |
| Agent runs | launch `kjxlkj-agent` runs and inspect operation outcomes |

## Optional Modules

| Module | Optional behavior |
|---|---|
| Workspace switcher | multi-workspace selection |
| Dashboards | configurable widgets |

## UX Safety Rules

- Optional modules MUST be off by default in baseline note-first mode.
- Advanced panels MUST NOT block baseline note editing flow.
- Agent run controls MUST expose clear mode (`reviewed` vs `yolo`).

## Related

- Web app shell: [web-app.md](web-app.md)
- Agent domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
