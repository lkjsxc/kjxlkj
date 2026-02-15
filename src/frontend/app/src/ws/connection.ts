/**
 * WebSocket connection manager with reconnect support.
 * Spec: reconnect flows MUST replay from acknowledged cursor.
 */
import type { ClientMessage, ServerMessage } from "./messages";

export type WsListener = (msg: ServerMessage) => void;
export type WsStatusListener = (connected: boolean) => void;

const RECONNECT_DELAY_MS = 2000;
const MAX_RECONNECT_DELAY_MS = 30000;

export class WsConnection {
  private ws: WebSocket | null = null;
  private listeners: Set<WsListener> = new Set();
  private statusListeners: Set<WsStatusListener> = new Set();
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private reconnectDelay = RECONNECT_DELAY_MS;
  private intentionalClose = false;

  connect(): void {
    this.intentionalClose = false;
    const proto = location.protocol === "https:" ? "wss:" : "ws:";
    const url = `${proto}//${location.host}/ws`;
    this.ws = new WebSocket(url);
    this.ws.onopen = () => {
      this.reconnectDelay = RECONNECT_DELAY_MS;
      this.notifyStatus(true);
    };
    this.ws.onmessage = (ev: MessageEvent) => {
      try {
        const msg = JSON.parse(String(ev.data)) as ServerMessage;
        for (const listener of this.listeners) listener(msg);
      } catch {
        /* ignore malformed messages */
      }
    };
    this.ws.onclose = () => {
      this.notifyStatus(false);
      if (!this.intentionalClose) this.scheduleReconnect();
    };
    this.ws.onerror = () => {
      this.ws?.close();
    };
  }

  disconnect(): void {
    this.intentionalClose = true;
    if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
    this.ws?.close();
    this.ws = null;
  }

  send(msg: ClientMessage): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(msg));
    }
  }

  onMessage(listener: WsListener): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  onStatus(listener: WsStatusListener): () => void {
    this.statusListeners.add(listener);
    return () => this.statusListeners.delete(listener);
  }

  get connected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }

  private notifyStatus(connected: boolean): void {
    for (const l of this.statusListeners) l(connected);
  }

  private scheduleReconnect(): void {
    this.reconnectTimer = setTimeout(() => {
      this.connect();
    }, this.reconnectDelay);
    this.reconnectDelay = Math.min(
      this.reconnectDelay * 2,
      MAX_RECONNECT_DELAY_MS,
    );
  }
}

/** Singleton WS connection for the app. */
export const wsConnection = new WsConnection();
