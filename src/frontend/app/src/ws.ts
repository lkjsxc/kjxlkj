/**
 * WebSocket client per /docs/spec/api/websocket.md
 *
 * Handles connection, reconnection, message typing, and ack cursor tracking.
 * No `any` per /docs/spec/technical/type-safety.md.
 */

/** Client -> Server message types per /docs/spec/api/websocket.md */
export type ClientMessage =
  | { type: 'subscribe_note'; note_id: string }
  | { type: 'unsubscribe_note'; note_id: string }
  | { type: 'subscribe_workspace'; workspace_id: string }
  | { type: 'ack'; stream_id: string; event_seq: number }
  | {
      type: 'apply_patch';
      note_id: string;
      base_version: number;
      patch_ops: Record<string, unknown>;
      idempotency_key: string;
      client_ts: string;
    };

/** Server -> Client message types per /docs/spec/api/websocket.md */
export type ServerMessage =
  | { type: 'subscribed'; stream_id: string; current_version: number; replay_cursor: number }
  | { type: 'patch_committed'; note_id: string; version: number; event_seq: number; idempotency_key: string }
  | { type: 'patch_rejected'; note_id: string; expected_version: number; current_version: number; reason: string }
  | { type: 'note_event'; note_id: string; event_seq: number; version: number; event_type: string; payload: Record<string, unknown> }
  | { type: 'workspace_event'; workspace_id: string; event_seq: number; event_type: string; payload: Record<string, unknown> }
  | { type: 'automation_event'; workspace_id: string; run_id: string; status: string; event_seq: number; payload: Record<string, unknown> }
  | { type: 'error'; code: string; message: string; details: Record<string, unknown> | null; request_id: string };

/** Listener for server messages */
export type MessageListener = (msg: ServerMessage) => void;

/**
 * WebSocket client with reconnection and ack cursor tracking.
 * Per /docs/spec/api/websocket.md: reconnect replays from last ack cursor.
 */
export class WsClient {
  private ws: WebSocket | null = null;
  private readonly url: string;
  private readonly listeners: Set<MessageListener> = new Set();
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private readonly ackCursors: Map<string, number> = new Map();

  constructor(url: string) {
    this.url = url;
  }

  /** Connect to WebSocket endpoint */
  connect(): void {
    if (this.ws) return;
    this.ws = new WebSocket(this.url);
    this.ws.onmessage = (event: MessageEvent) => {
      const msg = JSON.parse(String(event.data)) as ServerMessage;
      this.listeners.forEach((fn) => fn(msg));
    };
    this.ws.onclose = () => {
      this.ws = null;
      this.scheduleReconnect();
    };
    this.ws.onerror = () => {
      this.ws?.close();
    };
  }

  /** Disconnect */
  disconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    this.ws?.close();
    this.ws = null;
  }

  /** Send typed client message */
  send(msg: ClientMessage): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(msg));
    }
  }

  /** Subscribe to server messages */
  onMessage(listener: MessageListener): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  /** Track ack cursor per stream for replay on reconnect */
  ack(streamId: string, eventSeq: number): void {
    this.ackCursors.set(streamId, eventSeq);
    this.send({ type: 'ack', stream_id: streamId, event_seq: eventSeq });
  }

  /** Get last ack cursor for a stream */
  getAckCursor(streamId: string): number {
    return this.ackCursors.get(streamId) ?? 0;
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimer) return;
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      this.connect();
    }, 2000);
  }
}
