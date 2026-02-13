CREATE TABLE IF NOT EXISTS admin_jobs (
    id UUID PRIMARY KEY,
    requested_by UUID NOT NULL REFERENCES users(id),
    workspace_id UUID NULL REFERENCES workspaces(id) ON DELETE SET NULL,
    job_type TEXT NOT NULL CHECK (job_type IN ('export_markdown', 'backup_sql')),
    status TEXT NOT NULL CHECK (status IN ('queued', 'running', 'succeeded', 'failed')),
    artifact_path TEXT NULL,
    error_code TEXT NULL,
    error_detail TEXT NULL,
    started_at TIMESTAMPTZ NULL,
    finished_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_admin_jobs_created_at
    ON admin_jobs(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_admin_jobs_workspace_id
    ON admin_jobs(workspace_id, created_at DESC);