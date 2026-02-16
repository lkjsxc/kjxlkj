/// API client for kjxlkj REST endpoints.

export interface SessionInfo {
  user_id: string;
  username: string;
  display_name: string;
  is_owner: boolean;
  csrf_token: string;
}

export interface NoteStream {
  id: string;
  workspace_id: string;
  project_id: string | null;
  title: string;
  note_kind: string;
  access_scope: string;
  current_version: number;
  is_deleted: boolean;
  created_at: string;
  updated_at: string;
}

export interface NoteProjection {
  note_id: string;
  version: number;
  markdown: string;
  metadata_json: unknown;
  updated_at: string;
}

export interface Workspace {
  id: string;
  slug: string;
  name: string;
  owner_user_id: string;
  state: string;
  created_at: string;
}

const BASE = "";

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
  csrf?: string
): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (csrf) {
    headers["X-CSRF-Token"] = csrf;
  }
  const resp = await fetch(`${BASE}${path}`, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
    credentials: "same-origin",
  });
  if (!resp.ok) {
    const err = await resp.json().catch(() => ({}));
    throw new Error(err.message || `${resp.status} ${resp.statusText}`);
  }
  if (resp.status === 204) return undefined as unknown as T;
  return resp.json();
}

export const api = {
  // Auth
  register(username: string, password: string, displayName: string) {
    return request<SessionInfo>("POST", "/api/setup/register", {
      username,
      password,
      display_name: displayName,
    });
  },
  login(username: string, password: string) {
    return request<SessionInfo>("POST", "/api/auth/login", {
      username,
      password,
    });
  },
  logout(csrf: string) {
    return request<void>("POST", "/api/auth/logout", undefined, csrf);
  },
  getSession() {
    return request<SessionInfo>("GET", "/api/auth/session");
  },

  // Workspaces
  listWorkspaces() {
    return request<Workspace[]>("GET", "/api/workspaces");
  },
  createWorkspace(slug: string, name: string, csrf: string) {
    return request<Workspace>("POST", "/api/workspaces", { slug, name }, csrf);
  },

  // Notes
  listNotes(workspaceId: string, projectId?: string) {
    const params = new URLSearchParams({ workspace_id: workspaceId });
    if (projectId) params.set("project_id", projectId);
    return request<NoteStream[]>("GET", `/api/notes?${params}`);
  },
  createNote(
    workspaceId: string,
    title: string,
    noteKind: string,
    csrf: string
  ) {
    return request<NoteStream>(
      "POST",
      "/api/notes",
      { workspace_id: workspaceId, title, note_kind: noteKind },
      csrf
    );
  },
  getNote(noteId: string) {
    return request<{
      stream: NoteStream;
      projection: NoteProjection | null;
    }>("GET", `/api/notes/${noteId}`);
  },
  patchNote(
    noteId: string,
    baseVersion: number,
    markdown: string,
    csrf: string
  ) {
    return request<{ version: number }>(
      "PATCH",
      `/api/notes/${noteId}`,
      { base_version: baseVersion, markdown },
      csrf
    );
  },
  updateTitle(
    noteId: string,
    baseVersion: number,
    title: string,
    csrf: string
  ) {
    return request<{ version: number }>(
      "PATCH",
      `/api/notes/${noteId}/title`,
      { base_version: baseVersion, title },
      csrf
    );
  },
  deleteNote(noteId: string, csrf: string) {
    return request<void>("DELETE", `/api/notes/${noteId}`, undefined, csrf);
  },

  // Search
  search(workspaceId: string, query: string, mode = "hybrid") {
    const params = new URLSearchParams({
      workspace_id: workspaceId,
      q: query,
      mode,
    });
    return request<unknown[]>("GET", `/api/search?${params}`);
  },
};
