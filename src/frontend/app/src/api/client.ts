import type { Note, SearchResponse, BacklinkResponse } from './types'

const API_BASE = '/api'

export async function fetchNotes(workspaceId: string): Promise<Note[]> {
  const response = await fetch(`${API_BASE}/notes?workspace_id=${workspaceId}`)
  if (!response.ok) throw new Error('Failed to fetch notes')
  const data = await response.json()
  return data.notes || []
}

export async function fetchNote(noteId: string): Promise<Note> {
  const response = await fetch(`${API_BASE}/notes/${noteId}`)
  if (!response.ok) throw new Error('Failed to fetch note')
  return response.json()
}

export async function createNote(workspaceId: string, data?: Partial<Note>): Promise<Note> {
  const response = await fetch(`${API_BASE}/notes`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace_id: workspaceId, ...data }),
  })
  if (!response.ok) throw new Error('Failed to create note')
  return response.json()
}

export async function updateNote(
  noteId: string,
  baseVersion: number,
  updates: Partial<Note>
): Promise<Note> {
  const response = await fetch(`${API_BASE}/notes/${noteId}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ base_version: baseVersion, ...updates }),
  })
  if (!response.ok) throw new Error('Failed to update note')
  return response.json()
}

export async function deleteNote(noteId: string, baseVersion: number): Promise<void> {
  const response = await fetch(`${API_BASE}/notes/${noteId}`, {
    method: 'DELETE',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ base_version: baseVersion }),
  })
  if (!response.ok) throw new Error('Failed to delete note')
}

export async function searchNotes(query: string, workspaceId: string): Promise<SearchResponse> {
  const params = new URLSearchParams({ q: query, workspace_id: workspaceId })
  const response = await fetch(`${API_BASE}/search?${params}`)
  if (!response.ok) throw new Error('Search failed')
  return response.json()
}

export async function fetchBacklinks(noteId: string): Promise<BacklinkResponse> {
  const response = await fetch(`${API_BASE}/notes/${noteId}/backlinks`)
  if (!response.ok) throw new Error('Failed to fetch backlinks')
  return response.json()
}

export async function login(email: string, password: string): Promise<{ user_id: string; email: string }> {
  const response = await fetch(`${API_BASE}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password }),
  })
  if (!response.ok) throw new Error('Login failed')
  return response.json()
}

export async function register(
  email: string,
  password: string,
  passwordConfirm: string
): Promise<{ user_id: string; email: string }> {
  const response = await fetch(`${API_BASE}/setup/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password, password_confirm: passwordConfirm }),
  })
  if (!response.ok) throw new Error('Registration failed')
  return response.json()
}

export async function logout(): Promise<void> {
  await fetch(`${API_BASE}/auth/logout`, { method: 'POST' })
}
