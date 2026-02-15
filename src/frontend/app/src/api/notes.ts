/**
 * Notes API calls.
 */
import type { Note, NoteEvent } from "../types";
import { get, post, patch, del } from "./client";

interface NoteStreamDto {
  id: string;
  workspace_id: string;
  project_id: string | null;
  title: string;
  note_kind: string;
  current_version: number;
  created_at: string;
  updated_at: string;
}

interface NoteProjectionDto {
  note_id: string;
  workspace_id: string;
  title: string;
  note_kind: string;
  version: number;
  markdown: string;
}

interface NoteMutationResult {
  note_id: string;
  version: number;
  request_id: string;
}

interface CreateNoteResult {
  note_id: string;
  request_id: string;
}

interface NoteHistoryDto {
  event_id: string;
  note_id: string;
  seq: number;
  event_type: string;
  payload_json: string;
  created_at: string;
}

function streamToNote(dto: NoteStreamDto): Note {
  return {
    id: dto.id,
    workspace_id: dto.workspace_id,
    title: dto.title,
    body: "",
    kind: dto.note_kind,
    version: dto.current_version,
    created_at: dto.created_at,
    updated_at: dto.updated_at,
  };
}

function projectionToNote(dto: NoteProjectionDto): Note {
  return {
    id: dto.note_id,
    workspace_id: dto.workspace_id,
    title: dto.title,
    body: dto.markdown,
    kind: dto.note_kind,
    version: dto.version,
    created_at: "",
    updated_at: "",
  };
}

export async function listNotes(workspaceId: string): Promise<Note[]> {
  const rows = await get<NoteStreamDto[]>(
    `/notes?workspace_id=${workspaceId}`,
  );
  return rows.map(streamToNote);
}

export async function getNote(id: string): Promise<Note> {
  const row = await get<NoteProjectionDto>(`/notes/${id}`);
  return projectionToNote(row);
}

export interface CreateNotePayload {
  workspace_id: string;
  title: string;
  note_kind?: string;
}

export async function createNote(payload: CreateNotePayload): Promise<Note> {
  const result = await post<CreateNoteResult>("/notes", payload);
  const notes = await listNotes(payload.workspace_id);
  const created = notes.find((n) => n.id === result.note_id);
  if (created) return created;
  return getNote(result.note_id);
}

export interface PatchNotePayload {
  base_body: string;
  body: string;
  expected_version: number;
}

export async function patchNote(
  id: string,
  payload: PatchNotePayload,
): Promise<Note> {
  const bodyLen = [...payload.base_body].length;
  const ops: Array<Record<string, number | string>> = [];
  if (bodyLen > 0) {
    ops.push({ delete: bodyLen });
  }
  if (payload.body.length > 0) {
    ops.push({ insert: payload.body });
  }
  await patch<NoteMutationResult>(`/notes/${id}`, {
    base_version: payload.expected_version,
    ops,
  });
  return getNote(id);
}

export interface PatchTitlePayload {
  title: string;
  expected_version: number;
}

export async function patchTitle(
  id: string,
  payload: PatchTitlePayload,
): Promise<Note> {
  await patch<NoteMutationResult>(`/notes/${id}/title`, {
    base_version: payload.expected_version,
    title: payload.title,
  });
  return getNote(id);
}

export async function deleteNote(id: string): Promise<void> {
  await del<void>(`/notes/${id}`);
}

export async function noteHistory(id: string): Promise<NoteEvent[]> {
  const rows = await get<NoteHistoryDto[]>(`/notes/${id}/history`);
  return rows.map((row) => ({
    id: row.event_id,
    note_id: row.note_id,
    event_seq: row.seq,
    version: row.seq,
    event_type: row.event_type,
    payload: row.payload_json,
    created_at: row.created_at,
  }));
}

export async function rollbackNote(
  id: string,
  targetVersion: number,
): Promise<Note> {
  await post<NoteMutationResult>(`/notes/${id}/rollback`, {
    target_version: targetVersion,
  });
  return getNote(id);
}

export async function searchNotes(
  workspaceId: string,
  query: string,
): Promise<Note[]> {
  const q = encodeURIComponent(query);
  const rows = await get<Array<{ note_id: string; title: string }>>(
    `/search?workspace_id=${workspaceId}&q=${q}`,
  );
  return rows.map((row) => ({
    id: row.note_id,
    workspace_id: workspaceId,
    title: row.title,
    body: "",
    kind: "markdown",
    version: 0,
    created_at: "",
    updated_at: "",
  }));
}
