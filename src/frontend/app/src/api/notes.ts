/**
 * Notes API calls.
 */
import type { Note, NoteEvent } from "../types";
import { get, post, patch, del } from "./client";

export async function listNotes(workspaceId: string): Promise<Note[]> {
  return get<Note[]>(`/notes?workspace_id=${workspaceId}`);
}

export async function getNote(id: string): Promise<Note> {
  return get<Note>(`/notes/${id}`);
}

export interface CreateNotePayload {
  workspace_id: string;
  title: string;
  kind?: string;
}

export async function createNote(payload: CreateNotePayload): Promise<Note> {
  return post<Note>("/notes", payload);
}

export interface PatchNotePayload {
  body: string;
  expected_version: number;
}

export async function patchNote(
  id: string,
  payload: PatchNotePayload,
): Promise<Note> {
  return patch<Note>(`/notes/${id}`, payload);
}

export interface PatchTitlePayload {
  title: string;
  expected_version: number;
}

export async function patchTitle(
  id: string,
  payload: PatchTitlePayload,
): Promise<Note> {
  return patch<Note>(`/notes/${id}/title`, payload);
}

export async function deleteNote(id: string): Promise<void> {
  await del<void>(`/notes/${id}`);
}

export async function noteHistory(id: string): Promise<NoteEvent[]> {
  return get<NoteEvent[]>(`/notes/${id}/history`);
}

export async function rollbackNote(
  id: string,
  targetVersion: number,
): Promise<Note> {
  return post<Note>(`/notes/${id}/rollback`, {
    target_version: targetVersion,
  });
}

export async function searchNotes(
  workspaceId: string,
  query: string,
): Promise<Note[]> {
  const q = encodeURIComponent(query);
  return get<Note[]>(`/search?workspace_id=${workspaceId}&q=${q}`);
}
