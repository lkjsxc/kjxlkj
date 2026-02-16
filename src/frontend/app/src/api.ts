/**
 * HTTP API client per /docs/spec/api/http.md
 *
 * All methods are typed and return structured results.
 * No `any` is used per /docs/spec/technical/type-safety.md.
 */

import type {
  ErrorResponse,
  NoteProjection,
  NoteStream,
  SearchResult,
  SessionInfo,
  Workspace,
  AutomationRule,
  AutomationRun,
  HealthResponse,
  SearchMode,
} from './types.js';

/** API result type: either success data or error envelope */
export type ApiResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: ErrorResponse };

/** Base URL for API requests */
const API_BASE = '/api';

async function request<T>(
  path: string,
  init?: RequestInit,
): Promise<ApiResult<T>> {
  const url = `${API_BASE}${path}`;
  const response = await fetch(url, {
    credentials: 'same-origin',
    headers: { 'Content-Type': 'application/json' },
    ...init,
  });
  const body: unknown = await response.json();
  if (!response.ok) {
    return { ok: false, error: body as ErrorResponse };
  }
  return { ok: true, data: body as T };
}

/** POST /api/setup/register */
export function setupRegister(
  username: string,
  password: string,
): Promise<ApiResult<{ message: string; username: string }>> {
  return request('/setup/register', {
    method: 'POST',
    body: JSON.stringify({ username, password }),
  });
}

/** POST /api/auth/login */
export function authLogin(
  username: string,
  password: string,
): Promise<ApiResult<{ message: string }>> {
  return request('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ username, password }),
  });
}

/** POST /api/auth/logout */
export function authLogout(): Promise<ApiResult<void>> {
  return request('/auth/logout', { method: 'POST' });
}

/** GET /api/auth/session */
export function authSession(): Promise<ApiResult<SessionInfo>> {
  return request('/auth/session');
}

/** GET /api/workspaces */
export function listWorkspaces(): Promise<ApiResult<ReadonlyArray<Workspace>>> {
  return request('/workspaces');
}

/** POST /api/workspaces */
export function createWorkspace(
  slug: string,
  name: string,
): Promise<ApiResult<Workspace>> {
  return request('/workspaces', {
    method: 'POST',
    body: JSON.stringify({ slug, name }),
  });
}

/** GET /api/notes */
export function listNotes(params: {
  workspace_id?: string;
  include_deleted?: boolean;
}): Promise<ApiResult<ReadonlyArray<NoteStream>>> {
  const query = new URLSearchParams();
  if (params.workspace_id) query.set('workspace_id', params.workspace_id);
  if (params.include_deleted) query.set('include_deleted', 'true');
  const qs = query.toString();
  return request(`/notes${qs ? `?${qs}` : ''}`);
}

/** POST /api/notes */
export function createNote(input: {
  workspace_id: string;
  project_id?: string;
  title?: string;
  note_kind?: string;
  markdown?: string;
}): Promise<ApiResult<NoteStream>> {
  return request('/notes', {
    method: 'POST',
    body: JSON.stringify(input),
  });
}

/** GET /api/notes/{id} */
export function getNote(id: string): Promise<ApiResult<NoteProjection>> {
  return request(`/notes/${id}`);
}

/** PATCH /api/notes/{id} */
export function patchNote(
  id: string,
  baseVersion: number,
  markdown?: string,
): Promise<ApiResult<{ note_id: string; version: number }>> {
  return request(`/notes/${id}`, {
    method: 'PATCH',
    body: JSON.stringify({ base_version: baseVersion, markdown }),
  });
}

/** DELETE /api/notes/{id} */
export function deleteNote(
  id: string,
): Promise<ApiResult<{ note_id: string; state: string }>> {
  return request(`/notes/${id}`, { method: 'DELETE' });
}

/** PATCH /api/notes/{id}/title */
export function updateTitle(
  id: string,
  baseVersion: number,
  title: string,
): Promise<ApiResult<{ note_id: string; title: string; version: number }>> {
  return request(`/notes/${id}/title`, {
    method: 'PATCH',
    body: JSON.stringify({ base_version: baseVersion, title }),
  });
}

/** GET /api/notes/{id}/history */
export function noteHistory(
  id: string,
): Promise<ApiResult<{ note_id: string; events: ReadonlyArray<unknown> }>> {
  return request(`/notes/${id}/history`);
}

/** GET /api/notes/{id}/backlinks */
export function noteBacklinks(
  id: string,
): Promise<ApiResult<{ note_id: string; backlinks: ReadonlyArray<string> }>> {
  return request(`/notes/${id}/backlinks`);
}

/** GET /api/search */
export function searchNotes(params: {
  q: string;
  workspace_id: string;
  project_id?: string;
  limit?: number;
  mode?: SearchMode;
}): Promise<ApiResult<ReadonlyArray<SearchResult>>> {
  const query = new URLSearchParams();
  query.set('q', params.q);
  query.set('workspace_id', params.workspace_id);
  if (params.project_id) query.set('project_id', params.project_id);
  if (params.limit !== undefined) query.set('limit', String(params.limit));
  if (params.mode) query.set('mode', params.mode);
  return request(`/search?${query.toString()}`);
}

/** GET /api/healthz */
export function healthz(): Promise<ApiResult<HealthResponse>> {
  return request('/healthz');
}

/** GET /api/readyz */
export function readyz(): Promise<ApiResult<HealthResponse>> {
  return request('/readyz');
}
