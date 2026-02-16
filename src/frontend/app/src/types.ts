/**
 * API client types per /docs/spec/api/types.md
 *
 * These interfaces mirror the backend Rust types exactly.
 * No `any` is used anywhere per /docs/spec/technical/type-safety.md.
 */

/** Error envelope per /docs/spec/api/errors.md */
export interface ErrorResponse {
  readonly code: string;
  readonly message: string;
  readonly details: Record<string, unknown> | null;
  readonly request_id: string;
}

/** Note kind enum per /docs/spec/domain/note-types.md */
export type NoteKind = 'markdown' | 'settings' | 'media_image' | 'media_video';

/** Access scope per /docs/spec/domain/notes.md */
export type AccessScope = 'workspace' | 'project';

/** Note state per /docs/spec/domain/notes.md */
export type NoteState = 'active' | 'soft_deleted';

/** Note stream identity per /docs/spec/domain/notes.md */
export interface NoteStream {
  readonly id: string;
  readonly workspace_id: string;
  readonly project_id: string | null;
  readonly title: string;
  readonly note_kind: NoteKind;
  readonly access_scope: AccessScope;
  readonly state: NoteState;
  readonly current_version: number;
  readonly created_at: string;
  readonly updated_at: string;
}

/** Note projection per /docs/spec/domain/notes.md */
export interface NoteProjection {
  readonly note_id: string;
  readonly title: string;
  readonly version: number;
  readonly markdown: string;
  readonly metadata_json: Record<string, unknown>;
  readonly updated_at: string;
}

/** Search result per /docs/spec/domain/search.md */
export interface SearchResult {
  readonly note_id: string;
  readonly title: string;
  readonly snippet: string;
  readonly score: number;
}

/** Search mode per /docs/spec/domain/search.md */
export type SearchMode = 'hybrid' | 'lexical' | 'semantic';

/** Workspace per /docs/spec/domain/workspaces.md */
export interface Workspace {
  readonly id: string;
  readonly slug: string;
  readonly name: string;
  readonly created_at: string;
}

/** Automation rule per /docs/spec/domain/automation.md */
export interface AutomationRule {
  readonly id: string;
  readonly workspace_id: string;
  readonly name: string;
  readonly trigger_kind: string;
  readonly enabled: boolean;
}

/** Automation run per /docs/spec/domain/automation.md */
export interface AutomationRun {
  readonly id: string;
  readonly rule_id: string;
  readonly status: 'queued' | 'running' | 'awaiting_review' | 'applied' | 'rejected' | 'failed';
}

/** Session info per /docs/spec/api/http.md */
export interface SessionInfo {
  readonly authenticated: boolean;
  readonly user_id?: string;
  readonly username?: string;
  readonly role?: string;
}

/** Health response per /docs/spec/architecture/deployment.md */
export interface HealthResponse {
  readonly status: string;
}
