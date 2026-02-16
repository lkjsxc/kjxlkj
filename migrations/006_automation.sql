-- Migration 006: Automation rules, runs, agent KV store
-- Spec: /docs/spec/domain/automation.md
-- Spec: /docs/spec/technical/librarian-agent.md
-- Idempotent: uses IF NOT EXISTS

CREATE TABLE IF NOT EXISTS automation_rules (
    id              UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    trigger         TEXT NOT NULL,
    condition_json  JSONB NOT NULL DEFAULT '{}',
    action_json     JSONB NOT NULL DEFAULT '{}',
    enabled         BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS automation_runs (
    id              UUID PRIMARY KEY,
    rule_id         UUID NOT NULL REFERENCES automation_rules(id) ON DELETE CASCADE,
    status          TEXT NOT NULL DEFAULT 'queued'
                    CHECK (status IN ('queued','running','succeeded','failed')),
    started_at      TIMESTAMP,
    finished_at     TIMESTAMP,
    result_json     JSONB,
    created_at      TIMESTAMP NOT NULL DEFAULT now()
);

-- Agent KV memory store (DB-backed alternative to file-based)
CREATE TABLE IF NOT EXISTS agent_kv_store (
    key             TEXT PRIMARY KEY,
    value           TEXT NOT NULL,
    updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

-- Agent state table
CREATE TABLE IF NOT EXISTS agent_states (
    state           TEXT PRIMARY KEY,
    created_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_automation_rules_workspace ON automation_rules(workspace_id);
CREATE INDEX IF NOT EXISTS idx_automation_runs_rule ON automation_runs(rule_id);
CREATE INDEX IF NOT EXISTS idx_automation_runs_status ON automation_runs(status);
