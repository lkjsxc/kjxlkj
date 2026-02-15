/**
 * WebSocket message types matching server protocol.
 * Spec: /docs/spec/api/websocket.md
 */

// --- Client messages ---

export interface SubscribeNote {
  type: "subscribe_note";
  note_id: string;
}

export interface UnsubscribeNote {
  type: "unsubscribe_note";
  note_id: string;
}

export interface SubscribeWorkspace {
  type: "subscribe_workspace";
  workspace_id: string;
}

export interface UnsubscribeWorkspace {
  type: "unsubscribe_workspace";
  workspace_id: string;
}

export interface ApplyPatch {
  type: "apply_patch";
  note_id: string;
  base_version: number;
  patch_ops: unknown[];
  idempotency_key: string;
  client_ts: string;
}

export interface AckMessage {
  type: "ack";
  stream_id: string;
  event_seq: number;
}

export interface PresencePing {
  type: "presence_ping";
  workspace_id: string;
  note_id: string;
  cursor: unknown;
}

export type ClientMessage =
  | SubscribeNote
  | UnsubscribeNote
  | SubscribeWorkspace
  | UnsubscribeWorkspace
  | ApplyPatch
  | AckMessage
  | PresencePing;

// --- Server messages ---

export interface Subscribed {
  type: "subscribed";
  stream_id: string;
  current_version: number;
  replay_cursor: number;
}

export interface PatchCommitted {
  type: "patch_committed";
  note_id: string;
  version: number;
  event_seq: number;
  idempotency_key: string;
}

export interface PatchRejected {
  type: "patch_rejected";
  note_id: string;
  expected_version: number;
  current_version: number;
  reason: string;
}

export interface NoteEventMsg {
  type: "note_event";
  note_id: string;
  event_seq: number;
  version: number;
  event_type: string;
  payload: string;
}

export interface WorkspaceEventMsg {
  type: "workspace_event";
  workspace_id: string;
  event_seq: number;
  event_type: string;
  payload: string;
}

export interface PresenceEventMsg {
  type: "presence_event";
  workspace_id: string;
  note_id: string;
  user_id: string;
  state: string;
  server_ts: string;
}

export interface Heartbeat {
  type: "heartbeat";
  server_ts: string;
}

export interface WsError {
  type: "error";
  code: string;
  message: string;
  request_id?: string;
}

export type ServerMessage =
  | Subscribed
  | PatchCommitted
  | PatchRejected
  | NoteEventMsg
  | WorkspaceEventMsg
  | PresenceEventMsg
  | Heartbeat
  | WsError;
