/// WebSocket client for kjxlkj real-time sync.

export type ServerMessage =
  | { type: "subscribed"; stream_id: string; current_version: number; replay_cursor: number }
  | { type: "patch_committed"; note_id: string; version: number; event_seq: number; idempotency_key: string }
  | { type: "patch_rejected"; note_id: string; expected_version: number; current_version: number; reason: string }
  | { type: "note_event"; note_id: string; event_seq: number; version: number; event_type: string; payload: unknown }
  | { type: "workspace_event"; workspace_id: string; event_seq: number; event_type: string; payload: unknown }
  | { type: "error"; code: string; message: string; details?: unknown; request_id: string };

export type ClientMessage =
  | { type: "subscribe_note"; note_id: string }
  | { type: "unsubscribe_note"; note_id: string }
  | { type: "subscribe_workspace"; workspace_id: string }
  | { type: "ack"; stream_id: string; event_seq: number }
  | {
      type: "apply_patch";
      note_id: string;
      base_version: number;
      patch_ops: unknown;
      idempotency_key: string;
      client_ts: string;
    };

export class WsClient {
  private ws: WebSocket | null = null;
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private handlers: ((msg: ServerMessage) => void)[] = [];

  constructor(private url: string = `ws://${location.host}/ws`) {}

  connect(): void {
    if (this.ws?.readyState === WebSocket.OPEN) return;

    this.ws = new WebSocket(this.url);
    this.ws.onmessage = (e) => {
      try {
        const msg = JSON.parse(e.data) as ServerMessage;
        for (const h of this.handlers) h(msg);
      } catch {
        // ignore parse errors
      }
    };
    this.ws.onclose = () => {
      this.scheduleReconnect();
    };
    this.ws.onerror = () => {
      this.ws?.close();
    };
  }

  disconnect(): void {
    if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
    this.ws?.close();
    this.ws = null;
  }

  send(msg: ClientMessage): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(msg));
    }
  }

  onMessage(handler: (msg: ServerMessage) => void): void {
    this.handlers.push(handler);
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimer) return;
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      this.connect();
    }, 2000);
  }
}
