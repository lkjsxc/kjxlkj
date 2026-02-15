/**
 * Automation API calls.
 * Spec: /docs/spec/api/http.md — automation rules and runs.
 * Spec: /docs/spec/domain/automation.md — run lifecycle.
 */
import { get, post, patch, del } from "./client";

// --- Rule types ---

export interface AutomationRule {
  id: string;
  workspace_id: string;
  name: string;
  trigger: string;
  condition_json: Record<string, unknown>;
  action_json: Record<string, unknown>;
  enabled: boolean;
}

export interface CreateRulePayload {
  workspace_id: string;
  name: string;
  trigger: string;
  condition_json: Record<string, unknown>;
  action_json: Record<string, unknown>;
}

export async function listRules(
  workspaceId: string,
): Promise<AutomationRule[]> {
  return get<AutomationRule[]>(
    `/automation/rules?workspace_id=${workspaceId}`,
  );
}

export async function createRule(
  payload: CreateRulePayload,
): Promise<AutomationRule> {
  return post<AutomationRule>("/automation/rules", payload);
}

export async function updateRule(
  id: string,
  payload: Partial<CreateRulePayload>,
): Promise<AutomationRule> {
  return patch<AutomationRule>(`/automation/rules/${id}`, payload);
}

export async function deleteRule(id: string): Promise<void> {
  await del<void>(`/automation/rules/${id}`);
}

// --- Run types ---

export interface AutomationRun {
  id: string;
  rule_id: string;
  status: "queued" | "running" | "succeeded" | "failed";
  triggering_event_id: string | null;
  result_json: Record<string, unknown>;
}

export interface LibrarianOperation {
  id: string;
  run_id: string;
  kind: string;
  note_id: string;
  confidence: number;
  detail: Record<string, unknown>;
  decision: string;
}

export async function launchRun(
  ruleId: string,
  triggeringEventId?: string,
): Promise<AutomationRun> {
  return post<AutomationRun>("/automation/runs", {
    rule_id: ruleId,
    triggering_event_id: triggeringEventId,
  });
}

export async function listRuns(
  workspaceId: string,
): Promise<AutomationRun[]> {
  return get<AutomationRun[]>(
    `/automation/runs?workspace_id=${workspaceId}`,
  );
}

export async function getRun(id: string): Promise<AutomationRun> {
  return get<AutomationRun>(`/automation/runs/${id}`);
}

export async function reviewRun(
  id: string,
  decisions: Record<string, string>,
): Promise<AutomationRun> {
  return post<AutomationRun>(`/automation/runs/${id}/review`, {
    decisions,
  });
}
