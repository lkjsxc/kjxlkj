// Domain types mirroring /docs/spec/api/types.md

export interface User {
  id: string;
  email: string;
  display_name: string;
  role: 'owner' | 'admin' | 'editor' | 'viewer';
  status: 'active' | 'disabled';
}

export interface SessionInfo {
  user_id: string;
  email: string;
  display_name: string;
  role: string;
  csrf_token: string;
}

export interface Workspace {
  id: string;
  slug: string;
  name: string;
  owner_user_id: string;
}

export interface Project {
  id: string;
  workspace_id: string;
  name: string;
  description?: string;
}

export interface NoteProjection {
  note_id: string;
  workspace_id: string;
  project_id?: string;
  title: string;
  note_kind: 'markdown' | 'settings' | 'media_image' | 'media_video';
  version: number;
  markdown: string;
  metadata_json: Record<string, unknown>;
}

export interface NoteEvent {
  event_id: string;
  note_id: string;
  seq: number;
  event_type: string;
  payload_json: unknown;
  actor_id: string;
}

export interface AutomationRule {
  id: string;
  workspace_id: string;
  trigger: string;
  condition_json: unknown;
  action_json: unknown;
  enabled: boolean;
}

export interface AutomationRun {
  id: string;
  rule_id: string;
  status: 'pending' | 'running' | 'completed' | 'failed' | 'cancelled';
}

export interface SavedView {
  id: string;
  workspace_id: string;
  query_json: unknown;
  sort?: string;
  filters?: unknown;
  owner_user_id: string;
}
