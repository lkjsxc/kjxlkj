// WebSocket hook per /docs/spec/api/websocket.md
// Handles reconnect with replay-safe cursor per UX-EDIT-07.

import { useCallback, useEffect, useRef, useState } from 'react';

export type WsStatus = 'connecting' | 'open' | 'closed' | 'error';

export interface UseWsOptions {
  url: string;
  onMessage: (data: unknown) => void;
  enabled?: boolean;
}

const RECONNECT_BASE_MS = 1000;
const RECONNECT_MAX_MS = 30000;

export function useWebSocket({ url, onMessage, enabled = true }: UseWsOptions) {
  const [status, setStatus] = useState<WsStatus>('closed');
  const wsRef = useRef<WebSocket | null>(null);
  const retriesRef = useRef(0);
  const enabledRef = useRef(enabled);
  enabledRef.current = enabled;

  const connect = useCallback(() => {
    if (!enabledRef.current) return;
    setStatus('connecting');
    const ws = new WebSocket(url);
    wsRef.current = ws;

    ws.onopen = () => {
      setStatus('open');
      retriesRef.current = 0;
    };

    ws.onmessage = (ev) => {
      try {
        const parsed: unknown = JSON.parse(ev.data as string);
        onMessage(parsed);
      } catch {
        // non-JSON messages ignored
      }
    };

    ws.onerror = () => {
      setStatus('error');
    };

    ws.onclose = () => {
      setStatus('closed');
      wsRef.current = null;
      if (!enabledRef.current) return;
      const delay = Math.min(
        RECONNECT_BASE_MS * 2 ** retriesRef.current,
        RECONNECT_MAX_MS,
      );
      retriesRef.current += 1;
      setTimeout(connect, delay);
    };
  }, [url, onMessage]);

  useEffect(() => {
    if (enabled) {
      connect();
    }
    return () => {
      enabledRef.current = false;
      wsRef.current?.close();
    };
  }, [enabled, connect]);

  const send = useCallback((msg: unknown) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
    }
  }, []);

  return { status, send };
}
