// kjxlkj — WebSocket client
'use strict';

const WsClient = {
  onMessage(msg) {
    switch (msg.type) {
      case 'patch_committed':
        EditorUI.onPatchCommitted(msg);
        break;
      case 'patch_rejected':
        EditorUI.onPatchRejected(msg);
        break;
      case 'note_event':
        this.handleNoteEvent(msg);
        break;
      case 'workspace_event':
        // Refresh workspace data on relevant events
        App.loadNotes();
        break;
      case 'heartbeat':
        // Connection alive
        break;
      case 'error':
        console.warn('WS error:', msg.code, msg.message);
        break;
      default:
        // UX: tolerate unknown event types per websocket.md
        break;
    }
  },

  handleNoteEvent(msg) {
    // If another user edited the note we're viewing, update synced state
    const current = App.state.currentNote;
    if (!current || current.id !== msg.note_id) return;
    if (msg.version > App.state.syncedVersion) {
      // Remote edit detected; if no local unsaved changes, auto-update
      if (App.state.draft === App.state.syncedBody) {
        // Auto-refresh from server
        EditorUI.openNote(msg.note_id);
      } else {
        // Local draft differs — conflict state
        App.state.saveStatus = 'conflict';
        EditorUI.updateStatus();
      }
    }
  },
};
