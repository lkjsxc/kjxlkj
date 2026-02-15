/** Shared domain types for the frontend. */

export interface Session {
  user_id: string;
  email: string;
  display_name: string;
  role: string;
  csrf_token: string;
}

export interface Note {
  id: string;
  workspace_id: string;
  title: string;
  body: string;
  kind: string;
  version: number;
  created_at: string;
  updated_at: string;
}

export interface NoteEvent {
  id: string;
  note_id: string;
  event_seq: number;
  version: number;
  event_type: string;
  payload: string;
  created_at: string;
}

export type SaveStatus =
  | "idle"
  | "saving"
  | "saved"
  | "conflict"
  | "offline"
  | "error";

export interface ApiError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
}
