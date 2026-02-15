# Final File Structure Contract

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Defines required repository structure for canonical completion and optional
runtime reconstruction.

## Completion Definitions

A canonical documentation completion claim is valid only when:

- root tree matches docs-only canonical contract
- `docs/` tree matches this contract
- TODO, reference, and spec files are synchronized

A runtime reconstruction claim is valid only when:

- canonical docs completion is already satisfied
- derived runtime tree exists and follows typed-language rules
- no handwritten JavaScript runtime source exists

## Root Tree (Canonical Completion)

```text
.
├── AGENTS.md
├── GEMINI.md
├── README.md
├── LICENSE
├── .gitignore
├── .env.example
├── data/
│   └── config.json
├── .github/
└── docs/
```

## Canonical Docs Tree

```text
docs/
├── README.md
├── guides/
│   ├── README.md
│   ├── API.md
│   ├── DOCKER.md
│   ├── LIBRARIAN.md
│   ├── QUICKSTART.md
│   └── RECONSTRUCTION_BOOTSTRAP.md
├── overview/
│   ├── README.md
│   ├── all-in-docs.md
│   ├── glossary.md
│   └── principles.md
├── policy/
│   ├── README.md
│   ├── INSTRUCT.md
│   ├── ROOT_LAYOUT.md
│   ├── STRUCTURE.md
│   └── WORKFLOW.md
├── reference/
│   ├── README.md
│   ├── CI.md
│   ├── CONFORMANCE.md
│   ├── DRIFT_MATRIX.md
│   ├── EVIDENCE_INDEX.md
│   ├── IMPROVEMENT_BACKLOG.md
│   ├── LIMITATIONS.md
│   └── RELEASE.md
├── spec/
│   ├── README.md
│   ├── api/
│   │   ├── README.md
│   │   ├── errors.md
│   │   ├── http.md
│   │   ├── librarian-xml.md
│   │   ├── openapi.md
│   │   ├── openapi.yaml
│   │   ├── types.md
│   │   └── websocket.md
│   ├── architecture/
│   │   ├── README.md
│   │   ├── completion-file-map.md
│   │   ├── configuration.md
│   │   ├── crates.md
│   │   ├── deployment.md
│   │   ├── final-file-structure.md
│   │   ├── runtime.md
│   │   ├── source-layout.md
│   │   └── workspace-manifest.md
│   ├── domain/
│   │   ├── README.md
│   │   ├── attachments.md
│   │   ├── automation.md
│   │   ├── events.md
│   │   ├── export.md
│   │   ├── metadata.md
│   │   ├── note-types.md
│   │   ├── notes.md
│   │   ├── permissions.md
│   │   ├── projects.md
│   │   ├── search.md
│   │   └── workspaces.md
│   ├── security/
│   │   ├── README.md
│   │   ├── auth.md
│   │   ├── csrf.md
│   │   ├── sessions.md
│   │   └── transport.md
│   ├── technical/
│   │   ├── README.md
│   │   ├── librarian-agent.md
│   │   ├── librarian-prompts/
│   │   │   ├── README.md
│   │   │   ├── manifest.json
│   │   │   ├── stage-ingest.json
│   │   │   ├── stage-plan.json
│   │   │   ├── stage-propose.json
│   │   │   └── stage-validate-repair.json
│   │   ├── migrations.md
│   │   ├── operations.md
│   │   ├── performance.md
│   │   ├── testing.md
│   │   └── type-safety.md
│   └── ui/
│       ├── README.md
│       ├── editor-flow.md
│       ├── findings-traceability.md
│       ├── layout-and-interaction.md
│       ├── reconstruction-ux-requirements.md
│       ├── web-app.md
│       └── workspace-suite.md
└── todo/
    ├── README.md
    └── waves/
        ├── README.md
        ├── stage-00-pivot-governance/{README.md,wave-000.md,wave-001.md,wave-002.md}
        ├── stage-01-spec-rebuild/{README.md,wave-010.md,wave-011.md,wave-012.md}
        ├── stage-02-workspace-bootstrap/{README.md,wave-020.md,wave-021.md,wave-022.md}
        ├── stage-03-single-container-runtime/{README.md,wave-030.md,wave-031.md,wave-032.md}
        ├── stage-04-schema-and-projections/{README.md,wave-040.md,wave-041.md,wave-042.md}
        ├── stage-05-auth-and-security/{README.md,wave-050.md,wave-051.md,wave-052.md}
        ├── stage-06-rest-api/{README.md,wave-060.md,wave-061.md,wave-062.md}
        ├── stage-07-websocket-sync/{README.md,wave-070.md,wave-071.md,wave-072.md}
        ├── stage-08-frontend-and-static-hosting/{README.md,wave-080.md,wave-081.md,wave-082.md}
        ├── stage-09-ci-performance-release/{README.md,wave-090.md,wave-091.md,wave-092.md}
        └── stage-10-hardening-and-investigation/{README.md,wave-100.md,wave-101.md,wave-102.md}
```

## Derived Runtime Projection Tree (Optional)

When reconstruction is active, the repository MAY additionally contain:

```text
.
├── Cargo.toml
├── Cargo.lock
├── package.json
├── package-lock.json
├── tsconfig.json
├── Dockerfile
├── docker-compose.yml
├── .dockerignore
├── .env.example
├── data/config.json
└── src/
    ├── crates/
    │   ├── app/kjxlkj-server/
    │   ├── http/kjxlkj-http/
    │   ├── ws/kjxlkj-ws/
    │   ├── domain/kjxlkj-domain/
    │   ├── db/kjxlkj-db/
    │   ├── auth/kjxlkj-auth/
    │   ├── search/kjxlkj-search/
    │   ├── rbac/kjxlkj-rbac/
    │   ├── automation/kjxlkj-automation/
    │   └── workspace/kjxlkj-workspace/
    └── frontend/app/
        ├── src/
        ├── test/
        └── dist/
```

## Enforcement

- Runtime source file extensions MUST be `.rs`, `.ts`, or `.tsx`.
- Generated frontend bundle outputs MAY exist under `src/frontend/app/dist/`.
- Handwritten runtime `.js` source files are forbidden.
- Missing derived runtime artifacts are allowed in canonical docs-only completion.

## Related

- Source layout: [source-layout.md](source-layout.md)
- Workspace manifest: [workspace-manifest.md](workspace-manifest.md)
- Completion map: [completion-file-map.md](completion-file-map.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
