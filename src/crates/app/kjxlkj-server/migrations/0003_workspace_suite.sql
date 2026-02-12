alter table users
  add column if not exists display_name text not null default '',
  add column if not exists role text not null default 'viewer',
  add column if not exists status text not null default 'active';

create table if not exists workspaces (
  id uuid primary key,
  slug text not null unique,
  name text not null,
  owner_user_id uuid not null references users(id) on delete restrict,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz
);

create table if not exists workspace_members (
  workspace_id uuid not null references workspaces(id) on delete cascade,
  user_id uuid not null references users(id) on delete cascade,
  role text not null,
  joined_at timestamptz not null default now(),
  primary key (workspace_id, user_id)
);
create index if not exists idx_workspace_members_user on workspace_members (user_id);

create table if not exists projects (
  id uuid primary key,
  workspace_id uuid not null references workspaces(id) on delete cascade,
  name text not null,
  description text not null default '',
  archived_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (workspace_id, name)
);
create index if not exists idx_projects_workspace on projects (workspace_id);

create table if not exists saved_views (
  id uuid primary key,
  workspace_id uuid not null references workspaces(id) on delete cascade,
  query_json jsonb not null default '{}'::jsonb,
  sort text not null default 'updated_desc',
  filters jsonb not null default '{}'::jsonb,
  owner_user_id uuid not null references users(id) on delete restrict,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists dashboard_widgets (
  id uuid primary key,
  workspace_id uuid not null references workspaces(id) on delete cascade,
  type text not null,
  config_json jsonb not null default '{}'::jsonb,
  layout jsonb not null default '{}'::jsonb,
  owner_user_id uuid not null references users(id) on delete restrict,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists automation_rules (
  id uuid primary key,
  workspace_id uuid not null references workspaces(id) on delete cascade,
  trigger text not null,
  condition_json jsonb not null default '{}'::jsonb,
  action_json jsonb not null default '{}'::jsonb,
  enabled boolean not null default true,
  created_by uuid not null references users(id) on delete restrict,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists idx_automation_rules_workspace on automation_rules (workspace_id);

create table if not exists automation_runs (
  id uuid primary key,
  rule_id uuid not null references automation_rules(id) on delete cascade,
  workspace_id uuid not null references workspaces(id) on delete cascade,
  triggering_event_id uuid not null,
  status text not null,
  result_json jsonb,
  error text,
  started_at timestamptz not null default now(),
  finished_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (rule_id, triggering_event_id)
);
create index if not exists idx_automation_runs_workspace on automation_runs (workspace_id);

create table if not exists workspace_events (
  event_id uuid primary key,
  workspace_id uuid not null references workspaces(id) on delete cascade,
  seq bigint not null,
  event_type text not null,
  payload_json jsonb not null,
  actor_id uuid not null references users(id) on delete restrict,
  created_at timestamptz not null default now(),
  unique (workspace_id, seq)
);
create index if not exists idx_workspace_events_stream on workspace_events (workspace_id, seq);

alter table note_streams
  add column if not exists workspace_id uuid references workspaces(id) on delete restrict,
  add column if not exists project_id uuid references projects(id) on delete set null,
  add column if not exists note_kind text not null default 'markdown',
  add column if not exists access_scope text not null default 'workspace';
create index if not exists idx_note_streams_workspace on note_streams (workspace_id);
create index if not exists idx_note_streams_project on note_streams (project_id);

alter table note_projections
  add column if not exists workspace_id uuid references workspaces(id) on delete restrict,
  add column if not exists project_id uuid references projects(id) on delete set null,
  add column if not exists note_kind text not null default 'markdown';
create index if not exists idx_note_proj_workspace on note_projections (workspace_id);
create index if not exists idx_note_proj_project on note_projections (project_id);
