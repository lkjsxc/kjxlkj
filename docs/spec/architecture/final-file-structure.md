# Final File Structure Contract

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Defines canonical repository structure for:

1. docs-only baseline (current authoritative state)
2. fully reconstructed runtime state (completion target)

## State A: Docs-Only Baseline

```text
.
├── AGENTS.md
├── GEMINI.md
├── README.md
├── LICENSE
├── .gitignore
├── .env.example
├── data/
│   ├── config.json
│   └── agent-prompt.json
└── docs/
    ├── README.md
    ├── guides/
    ├── overview/
    ├── policy/
    ├── reference/
    ├── spec/
    └── todo/
```

## State B: Reconstructed Runtime Target

```text
.
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── docker-compose.yml
├── .dockerignore
├── src/
│   ├── crates/
│   │   ├── app/kjxlkj-server/
│   │   ├── http/kjxlkj-http/
│   │   ├── ws/kjxlkj-ws/
│   │   ├── domain/kjxlkj-domain/
│   │   ├── db/kjxlkj-db/
│   │   ├── auth/kjxlkj-auth/
│   │   ├── search/kjxlkj-search/
│   │   ├── rbac/kjxlkj-rbac/
│   │   ├── automation/kjxlkj-automation/
│   │   └── workspace/kjxlkj-workspace/
│   └── frontend/app/
│       ├── src/
│       ├── test/
│       └── dist/
├── data/
│   ├── config.json
│   └── agent-prompt.json
└── docs/
    └── (same canonical tree)
```

## Completion Interpretation

A completion claim is valid only when:

- runtime tree matches State B
- behavior conforms to `/docs/spec`
- ledgers in `/docs/reference` show synchronized evidence
- TODO checklists are completed with linked proofs

## Prohibitions

- `tmp/` MUST NOT exist in completion state.
- `log/` and `docs/logs/` MUST NOT exist in completion state.
- handwritten runtime `.js` source is forbidden.

## Related

- Completion map: [completion-file-map.md](completion-file-map.md)
- Root layout policy: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- TODO execution: [/docs/todo/README.md](/docs/todo/README.md)
