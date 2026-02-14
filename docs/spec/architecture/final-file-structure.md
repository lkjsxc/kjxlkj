# Final File Structure Contract

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Defines the full repository structure required for completion claims.

## Completion Definition

A completion claim is valid only when:

- canonical docs tree matches this contract
- derived runtime tree exists and follows typed-language rules
- no direct JavaScript runtime source exists

## Root Tree (Canonical + Derived)

```text
.
├── README.md
├── LICENSE
├── .gitignore
├── .github/
├── docs/
├── Cargo.toml
├── Cargo.lock
├── package.json
├── tsconfig.json
├── Dockerfile
├── docker-compose.yml
├── .dockerignore
└── src/
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
├── log/
│   ├── README.md
│   ├── proposals/
│   │   ├── README.md
│   │   └── YYYY-MM-DD-*.md
│   └── audits/
│       ├── README.md
│       └── YYYY-MM-DD-*.md
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
        └── stage-09-ci-performance-release/{README.md,wave-090.md,wave-091.md,wave-092.md}
```

## Derived Runtime Tree (Required At Completion)

```text
src/
├── backend/
│   └── crates/
│       ├── app/kjxlkj-server/{Cargo.toml,src/main.rs}
│       ├── http/kjxlkj-http/{Cargo.toml,src/lib.rs,src/dto.rs,src/handlers.rs,src/middleware.rs,src/error.rs}
│       ├── ws/kjxlkj-ws/{Cargo.toml,src/lib.rs}
│       ├── domain/kjxlkj-domain/{Cargo.toml,src/lib.rs}
│       ├── db/kjxlkj-db/{Cargo.toml,migrations/001_initial.sql,src/lib.rs}
│       ├── security/kjxlkj-security/{Cargo.toml,src/lib.rs}
│       └── automation/kjxlkj-automation/{Cargo.toml,src/lib.rs}
└── frontend/
    └── app/
        ├── package.json
        ├── tsconfig.json
        └── src/
            ├── main.ts
            ├── app.ts
            ├── routes/{setup.ts,login.ts,workspace.ts}
            ├── state/{session.ts,notes.ts,librarian.ts}
            ├── api/{http-client.ts,ws-client.ts}
            └── ui/{shell.ts,editor.ts,librarian.ts}
```

## Enforcement

- Runtime file extensions MUST be `.rs`, `.ts`, or `.tsx` only.
- Runtime directories and files above are mandatory once reconstruction starts.
- Missing required paths block completion and release closure.

## Related

- Source layout: [source-layout.md](source-layout.md)
- Workspace manifest: [workspace-manifest.md](workspace-manifest.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
