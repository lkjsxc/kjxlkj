/**
 * Workspace API calls.
 */
import { get, post } from "./client";

export interface Workspace {
  id: string;
  slug: string;
  name: string;
  owner_user_id: string;
  created_at: string;
}

export interface CreateWorkspacePayload {
  slug: string;
  name: string;
}

interface CreateWorkspaceResult {
  workspace_id: string;
  request_id: string;
}

export async function listWorkspaces(): Promise<Workspace[]> {
  return get<Workspace[]>("/workspaces");
}

export async function createWorkspace(
  payload: CreateWorkspacePayload,
): Promise<string> {
  const result = await post<CreateWorkspaceResult>("/workspaces", payload);
  return result.workspace_id;
}

