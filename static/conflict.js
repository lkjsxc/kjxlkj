// kjxlkj — Conflict dialog (UX-EDIT-06)
'use strict';

const ConflictDialog = {
  renderContainer() {
    return `
    <div class="conflict-dialog" id="conflict-dialog" role="alertdialog"
         aria-label="Version conflict detected">
      <h3>Version Conflict</h3>
      <p>Another edit was saved while you were working. Choose an action:</p>
      <div class="actions">
        <button id="conflict-reload" aria-label="Reload latest version">Reload Latest</button>
        <button id="conflict-reapply" aria-label="Reapply your draft">Reapply Draft</button>
        <button id="conflict-copy" aria-label="Copy draft to clipboard">Copy Draft</button>
      </div>
    </div>`;
  },

  show() {
    const el = document.getElementById('conflict-dialog');
    if (el) el.classList.add('open');
    this.bind();
  },

  hide() {
    const el = document.getElementById('conflict-dialog');
    if (el) el.classList.remove('open');
    App.state.conflictOpen = false;
  },

  bind() {
    const reload = document.getElementById('conflict-reload');
    const reapply = document.getElementById('conflict-reapply');
    const copy = document.getElementById('conflict-copy');
    if (reload) reload.onclick = async () => {
      // Discard local draft, reload from server
      if (App.state.currentNote) {
        await EditorUI.openNote(App.state.currentNote.id);
      }
      this.hide();
    };
    if (reapply) reapply.onclick = async () => {
      // Re-fetch latest version, then re-apply draft on top
      if (App.state.currentNote) {
        const r = await fetch(`/api/notes/${App.state.currentNote.id}`);
        if (r.ok) {
          const latest = await r.json();
          App.state.syncedVersion = latest.version;
          App.state.syncedBody = latest.body;
          // Keep draft as-is, attempt save again
          EditorUI.saveNow();
        }
      }
      this.hide();
    };
    if (copy) copy.onclick = () => {
      // Copy draft to clipboard for manual recovery
      if (navigator.clipboard) {
        navigator.clipboard.writeText(App.state.draft);
      }
      this.hide();
    };
  },
};
