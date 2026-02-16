/**
 * Frontend entry point per /docs/spec/ui/web-app.md
 */
export { App, boot } from './app.js';
export type { AppState, AppView, EditorState } from './state.js';
export type { ClientMessage, ServerMessage, MessageListener } from './ws.js';
export { WsClient } from './ws.js';
export * from './types.js';
export * from './api.js';
