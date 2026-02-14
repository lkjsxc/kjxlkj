export type HttpMethod = "GET" | "POST" | "PATCH";

export type HttpOptions = {
  method?: HttpMethod;
  csrfToken?: string;
  body?: unknown;
};

export async function httpJson<T>(path: string, options?: HttpOptions): Promise<T> {
  const response = await fetch(path, {
    method: options?.method ?? "GET",
    credentials: "same-origin",
    headers: {
      "content-type": "application/json",
      ...(options?.csrfToken ? { "x-csrf-token": options.csrfToken } : {}),
    },
    body: options?.body !== undefined ? JSON.stringify(options.body) : undefined,
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  return (await response.json()) as T;
}

export type SessionPayload = {
  user: { id: string; email: string };
  csrf_token: string;
};

export type NotePayload = {
  id: string;
  title: string;
  markdown: string;
  current_version: number;
  workspace_id: string;
};

export async function getSession(): Promise<SessionPayload> {
  return httpJson<SessionPayload>("/api/auth/session");
}

export async function listNotes(): Promise<NotePayload[]> {
  const response = await httpJson<{ items: NotePayload[] }>("/api/notes");
  return response.items;
}

export async function createNote(csrfToken: string): Promise<NotePayload> {
  const response = await httpJson<{ item: NotePayload }>("/api/notes", {
    method: "POST",
    csrfToken,
    body: {
      workspace_id: "ws-1",
      title: "Untitled",
      markdown: "",
    },
  });
  return response.item;
}

export async function patchNote(
  noteId: string,
  baseVersion: number,
  baseMarkdown: string,
  markdown: string,
  csrfToken: string,
  idempotencyKey: string,
): Promise<NotePayload> {
  const response = await httpJson<{ item: NotePayload }>(`/api/notes/${noteId}`, {
    method: "PATCH",
    csrfToken,
    body: {
      base_version: baseVersion,
      patch_ops: [{ delete: baseMarkdown.length }, { insert: markdown }],
      idempotency_key: idempotencyKey,
    },
  });
  return response.item;
}

export async function patchTitle(
  noteId: string,
  baseVersion: number,
  title: string,
  csrfToken: string,
): Promise<NotePayload> {
  const response = await httpJson<{ item: NotePayload }>(`/api/notes/${noteId}/title`, {
    method: "PATCH",
    csrfToken,
    body: {
      base_version: baseVersion,
      title,
    },
  });
  return response.item;
}
