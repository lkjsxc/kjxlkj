create table if not exists note_snapshots (
  note_id uuid not null references note_streams(id) on delete cascade,
  version bigint not null,
  markdown text not null,
  metadata_json jsonb not null,
  tags text[] not null default '{}'::text[],
  created_at timestamptz not null default now(),
  primary key (note_id, version)
);

create index if not exists idx_note_snapshots_note_version_desc
  on note_snapshots (note_id, version desc);
