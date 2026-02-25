-- Migration 006: Automation rules and runs
-- Spec: /docs/spec/domain/automation.md

CREATE TABLE IF NOT EXISTS automation_rules (
    rule_id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id    UUID NOT NULL REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    trigger         TEXT NOT NULL,
    condition_json  JSONB NOT NULL DEFAULT '{}',
    action_json     JSONB NOT NULL,
    enabled         BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_automation_rules_workspace ON automation_rules(workspace_id);
CREATE INDEX IF NOT EXISTS idx_automation_rules_enabled ON automation_rules(enabled) WHERE enabled = true;

CREATE TABLE IF NOT EXISTS automation_runs (
    run_id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id         UUID NOT NULL REFERENCES automation_rules(rule_id) ON DELETE CASCADE,
    triggering_event_id UUID,
    status          TEXT NOT NULL DEFAULT 'queued' CHECK (status IN ('queued', 'running', 'succeeded', 'failed')),
    started_at      TIMESTAMP,
    completed_at    TIMESTAMP,
    error_message   TEXT,
    audit_metadata  JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_automation_runs_rule ON automation_runs(rule_id);
CREATE INDEX IF NOT EXISTS idx_automation_runs_status ON automation_runs(status);

-- Agent KV memory store
CREATE TABLE IF NOT EXISTS agent_kv_store (
    workspace_id    UUID NOT NULL REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    agent_name      TEXT NOT NULL DEFAULT 'kjxlkj-agent',
    key             TEXT NOT NULL,
    value           JSONB NOT NULL,
    updated_at      TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (workspace_id, agent_name, key)
);

-- Agent state store
CREATE TABLE IF NOT EXISTS agent_state (
    workspace_id    UUID NOT NULL REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    agent_name      TEXT NOT NULL DEFAULT 'kjxlkj-agent',
    state           TEXT NOT NULL,
    updated_at      TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (workspace_id, agent_name)
);
