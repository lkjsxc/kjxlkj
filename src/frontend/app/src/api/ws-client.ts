export type WsClient = {
  send(message: string): void;
  close(): void;
};

export function createWsClient(url: string): WsClient {
  const socket = new WebSocket(url);
  return {
    send(message: string): void {
      socket.send(message);
    },
    close(): void {
      socket.close();
    },
  };
}
