create extension if not exists pgcrypto;

create table if not exists users (
  id uuid primary key,
  email text not null unique,
  password_hash text not null,
  created_at timestamptz not null default now()
);

create table if not exists sessions (
  id uuid primary key,
  user_id uuid not null references users(id) on delete cascade,
  csrf_token text not null,
  expires_at timestamptz not null,
  created_at timestamptz not null default now(),
  last_seen_at timestamptz not null default now()
);
create index if not exists idx_sessions_expires_at on sessions (expires_at);

create table if not exists note_streams (
  id uuid primary key,
  title text not null,
  current_version bigint not null default 0,
  deleted_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists note_events (
  event_id uuid primary key,
  note_id uuid not null references note_streams(id) on delete cascade,
  seq bigint not null,
  event_type text not null,
  payload_json jsonb not null,
  actor_id uuid not null,
  idempotency_key text,
  created_at timestamptz not null default now(),
  unique (note_id, seq)
);
create unique index if not exists uq_note_events_note_idempotency
  on note_events (note_id, idempotency_key)
  where idempotency_key is not null;

create table if not exists note_projections (
  note_id uuid primary key references note_streams(id) on delete cascade,
  title text not null,
  version bigint not null,
  markdown text not null,
  rendered_html text,
  metadata_json jsonb not null default '{}'::jsonb,
  tags text[] not null default '{}'::text[],
  search_vector tsvector not null default ''::tsvector,
  updated_at timestamptz not null default now()
);
create index if not exists idx_note_projections_search on note_projections using gin(search_vector);
create index if not exists idx_note_projections_tags on note_projections using gin(tags);

create table if not exists note_backlinks (
  note_id uuid not null references note_streams(id) on delete cascade,
  source_note_id uuid not null references note_streams(id) on delete cascade,
  created_at timestamptz not null default now(),
  primary key (note_id, source_note_id)
);

create table if not exists attachments (
  id uuid primary key,
  note_id uuid not null references note_streams(id) on delete cascade,
  filename text not null,
  mime text not null,
  size_bytes bigint not null,
  sha256 text not null,
  chunk_count integer not null,
  created_at timestamptz not null default now()
);

create table if not exists attachment_chunks (
  attachment_id uuid not null references attachments(id) on delete cascade,
  chunk_index integer not null,
  bytes bytea not null,
  sha256 text not null,
  primary key (attachment_id, chunk_index)
);

create table if not exists jobs (
  id uuid primary key,
  kind text not null,
  status text not null,
  artifact_path text,
  error text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
