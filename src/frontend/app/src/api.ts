/**
 * API client layer for kjxlkj backend.
 * Per /docs/spec/api/http.md and /docs/spec/api/types.md.
 */

export interface SessionInfo {
  user_id: string;
  email: string;
  display_name: string;
  role: string;
  csrf_token: string;
}

export interface NoteProjection {
  note_id: string;
  title: string;
  note_kind: string;
  body_text: string;
  settings_json: string;
  version: number;
  workspace_id: string;
  updated_at: string;
}

export interface NoteListItem {
  id: string;
  title: string;
  note_kind: string;
  workspace_id: string;
  created_at: string;
}

async function request<T>(
  method: string,
  url: string,
  body?: unknown,
  csrf?: string,
): Promise<T> {
  const headers: Record<string, string> = {
    Accept: "application/json",
  };
  if (body !== undefined) {
    headers["Content-Type"] = "application/json";
  }
  if (csrf && method !== "GET") {
    headers["X-CSRF-Token"] = csrf;
  }
  const res = await fetch(url, {
    method,
    headers,
    credentials: "same-origin",
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });
  if (!res.ok) {
    const text = await res.text();
    throw new ApiError(res.status, text);
  }
  if (res.status === 204) return undefined as T;
  return res.json() as Promise<T>;
}

export class ApiError extends Error {
  constructor(
    public status: number,
    public body: string,
  ) {
    super(`HTTP ${status}: ${body}`);
    this.name = "ApiError";
  }
}

export function getSession(): Promise<SessionInfo> {
  return request("GET", "/api/auth/session");
}

export function register(
  email: string,
  password: string,
  display_name: string,
): Promise<SessionInfo> {
  return request("POST", "/api/auth/register", {
    email,
    password,
    display_name,
  });
}

export function login(
  email: string,
  password: string,
): Promise<SessionInfo> {
  return request("POST", "/api/auth/login", { email, password });
}

export function logout(csrf: string): Promise<void> {
  return request("POST", "/api/auth/logout", undefined, csrf);
}

export function listNotes(
  wsId: string,
  csrf: string,
): Promise<NoteListItem[]> {
  return request("GET", `/api/workspaces/${wsId}/notes`, undefined, csrf);
}

export function getNote(
  wsId: string,
  noteId: string,
  csrf: string,
): Promise<NoteProjection> {
  return request(
    "GET",
    `/api/workspaces/${wsId}/notes/${noteId}`,
    undefined,
    csrf,
  );
}

export function createNote(
  wsId: string,
  title: string,
  noteKind: string,
  csrf: string,
): Promise<{ id: string }> {
  return request(
    "POST",
    `/api/workspaces/${wsId}/notes`,
    { title, note_kind: noteKind },
    csrf,
  );
}

export function patchNote(
  wsId: string,
  noteId: string,
  ops: unknown[],
  baseVersion: number,
  csrf: string,
): Promise<{ version: number }> {
  return request(
    "PATCH",
    `/api/workspaces/${wsId}/notes/${noteId}`,
    { ops, base_version: baseVersion },
    csrf,
  );
}

export function updateTitle(
  wsId: string,
  noteId: string,
  title: string,
  baseVersion: number,
  csrf: string,
): Promise<void> {
  return request(
    "PUT",
    `/api/workspaces/${wsId}/notes/${noteId}/title`,
    { title, base_version: baseVersion },
    csrf,
  );
}
