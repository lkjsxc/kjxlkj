-- Migration 008: librarian run reports, operation audit, status alignment
-- Per /docs/spec/technical/librarian-agent.md, /docs/spec/domain/automation.md

-- Fix automation_runs.status to match spec vocabulary
-- Spec states: queued, running, succeeded, failed
ALTER TABLE automation_runs DROP CONSTRAINT IF EXISTS automation_runs_status_check;
ALTER TABLE automation_runs ADD CONSTRAINT automation_runs_status_check
    CHECK (status IN ('queued','running','succeeded','failed'));
UPDATE automation_runs SET status = 'queued' WHERE status = 'pending';
UPDATE automation_runs SET status = 'succeeded' WHERE status = 'completed';

-- Add triggering_event_id for idempotency per (rule_id, triggering_event_id)
ALTER TABLE automation_runs
    ADD COLUMN IF NOT EXISTS triggering_event_id UUID;
CREATE UNIQUE INDEX IF NOT EXISTS idx_automation_runs_idempotent
    ON automation_runs(rule_id, triggering_event_id);

-- Librarian run reports per /docs/spec/api/types.md
CREATE TABLE IF NOT EXISTS librarian_run_reports (
    run_id              UUID PRIMARY KEY REFERENCES automation_runs(id) ON DELETE CASCADE,
    provider_kind       TEXT NOT NULL,
    model               TEXT NOT NULL,
    prompt_hash         TEXT NOT NULL,
    parsed_operations   INT NOT NULL DEFAULT 0,
    applied_operations  INT NOT NULL DEFAULT 0,
    rejected_operations INT NOT NULL DEFAULT 0,
    warnings            JSONB NOT NULL DEFAULT '[]',
    raw_prompt          TEXT,
    raw_response        TEXT,
    parser_version      TEXT NOT NULL DEFAULT '1',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Librarian operation audit log per /docs/spec/technical/librarian-agent.md
CREATE TABLE IF NOT EXISTS librarian_operations (
    id                  UUID PRIMARY KEY,
    run_id              UUID NOT NULL REFERENCES automation_runs(id) ON DELETE CASCADE,
    operation_index     INT NOT NULL,
    kind                TEXT NOT NULL
                        CHECK (kind IN ('create_note','rewrite_note','retitle_note',
                                        'relink_note','retag_note','defer')),
    target_note_id      UUID,
    target_path         TEXT,
    title               TEXT,
    body_markdown       TEXT,
    reason              TEXT,
    confidence          REAL,
    status              TEXT NOT NULL DEFAULT 'pending'
                        CHECK (status IN ('pending','applied','rejected')),
    reject_reason       TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_librarian_operations_run
    ON librarian_operations(run_id);

-- Fix export_jobs status to match spec vocabulary
ALTER TABLE export_jobs DROP CONSTRAINT IF EXISTS export_jobs_status_check;
ALTER TABLE export_jobs ADD CONSTRAINT export_jobs_status_check
    CHECK (status IN ('queued','running','succeeded','failed'));
UPDATE export_jobs SET status = 'queued' WHERE status = 'pending';
UPDATE export_jobs SET status = 'succeeded' WHERE status = 'completed';

-- Add actor tracking for export jobs
ALTER TABLE export_jobs
    ADD COLUMN IF NOT EXISTS actor_id UUID REFERENCES users(id);
