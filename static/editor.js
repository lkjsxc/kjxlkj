// kjxlkj — Editor UI with autosave, conflict handling, and draft management
// UX-EDIT-01 through UX-EDIT-07
'use strict';

const EditorUI = {
  _debounceTimer: null,
  DEBOUNCE_MS: 800,

  renderEmpty() {
    const area = document.getElementById('editor-content');
    if (area) area.innerHTML = '<div style="padding:40px;color:var(--fg2);text-align:center"><p>Select or create a note</p></div>';
  },

  async openNote(noteId) {
    try {
      const r = await fetch(`/api/notes/${noteId}`);
      if (!r.ok) return;
      const note = await r.json();
      App.state.currentNote = note;
      // UX-EDIT-01: separate synced snapshot and local draft
      App.state.syncedBody = note.body;
      App.state.syncedVersion = note.version;
      App.state.draft = note.body;
      App.state.saveStatus = 'saved';
      this.renderEditor();
      NavUI.refresh();
      // Subscribe via WS
      if (App.state.ws && App.state.ws.readyState === 1) {
        App.state.ws.send(JSON.stringify({type:'subscribe_note', note_id: noteId}));
      }
    } catch { /* offline */ }
  },

  renderEditor() {
    const note = App.state.currentNote;
    if (!note) return this.renderEmpty();
    const area = document.getElementById('editor-content');
    if (!area) return;
    area.innerHTML = `
      <div class="editor-toolbar">
        <input class="title-input" id="note-title" value="${esc(note.title)}"
               aria-label="Note title" placeholder="Untitled">
        <span class="save-status ${App.state.saveStatus}" id="save-status"
              role="status" aria-live="polite">${App.state.saveStatus}</span>
      </div>
      <div class="editor-body">
        <textarea id="note-body" aria-label="Note content"
                  placeholder="Start writing...">${esc(App.state.draft)}</textarea>
      </div>`;
    this.bindEditor();
  },

  bindEditor() {
    const body = document.getElementById('note-body');
    const title = document.getElementById('note-title');
    if (body) {
      body.oninput = () => {
        App.state.draft = body.value;
        App.state.saveStatus = 'saving';
        this.updateStatus();
        // UX-EDIT-02: autosave with bounded debounce
        clearTimeout(this._debounceTimer);
        this._debounceTimer = setTimeout(() => this.saveNow(), this.DEBOUNCE_MS);
      };
    }
    if (title) {
      title.onchange = () => this.saveTitle(title.value);
    }
  },

  async saveNow() {
    const note = App.state.currentNote;
    if (!note || App.state.draft === App.state.syncedBody) {
      App.state.saveStatus = 'saved';
      this.updateStatus();
      return;
    }
    App.state.saveStatus = 'saving';
    this.updateStatus();
    try {
      // Try via WS first for real-time, fallback to HTTP
      if (App.state.ws && App.state.ws.readyState === 1) {
        const key = App.genIdempotencyKey();
        App.state.ws.send(JSON.stringify({
          type: 'apply_patch', note_id: note.id,
          base_version: App.state.syncedVersion,
          patch_ops: {body: App.state.draft},
          idempotency_key: key, client_ts: new Date().toISOString(),
        }));
      } else {
        const r = await fetch(`/api/notes/${note.id}`, {
          method: 'PATCH', headers: {'Content-Type':'application/json'},
          body: JSON.stringify({body: App.state.draft, version: App.state.syncedVersion}),
        });
        if (r.ok) {
          const updated = await r.json();
          App.state.syncedBody = App.state.draft;
          App.state.syncedVersion = updated.version;
          App.state.currentNote = updated;
          App.state.saveStatus = 'saved';
        } else if (r.status === 409) {
          App.state.saveStatus = 'conflict';
          App.state.conflictOpen = true;
          ConflictDialog.show();
        } else {
          App.state.saveStatus = 'offline';
        }
        this.updateStatus();
      }
    } catch {
      App.state.saveStatus = 'offline';
      this.updateStatus();
    }
  },

  async saveTitle(newTitle) {
    const note = App.state.currentNote;
    if (!note) return;
    try {
      const r = await fetch(`/api/notes/${note.id}/title`, {
        method: 'PATCH', headers: {'Content-Type':'application/json'},
        body: JSON.stringify({title: newTitle, version: App.state.syncedVersion}),
      });
      if (r.ok) {
        const updated = await r.json();
        App.state.currentNote = updated;
        App.state.syncedVersion = updated.version;
        // UX-EDIT-04: propagate title to nav in same interaction cycle
        const idx = App.state.notes.findIndex(n => n.id === note.id);
        if (idx >= 0) App.state.notes[idx].title = newTitle;
        NavUI.refresh();
      }
    } catch { /* offline */ }
  },

  async createNote() {
    const ws = App.state.currentWorkspace;
    if (!ws) return;
    try {
      const r = await fetch('/api/notes', {
        method: 'POST', headers: {'Content-Type':'application/json'},
        body: JSON.stringify({workspace_id: ws.id, title: 'Untitled', body: ''}),
      });
      if (r.ok) {
        const note = await r.json();
        App.state.notes.unshift(note);
        NavUI.refresh();
        this.openNote(note.id);
      }
    } catch { /* offline */ }
  },

  updateStatus() {
    const el = document.getElementById('save-status');
    if (el) {
      el.textContent = App.state.saveStatus;
      el.className = 'save-status ' + App.state.saveStatus;
    }
  },

  // Called when WS confirms patch
  onPatchCommitted(msg) {
    if (App.state.currentNote && App.state.currentNote.id === msg.note_id) {
      App.state.syncedBody = App.state.draft;
      App.state.syncedVersion = msg.version;
      App.state.saveStatus = 'saved';
      this.updateStatus();
    }
  },

  onPatchRejected(msg) {
    if (App.state.currentNote && App.state.currentNote.id === msg.note_id) {
      App.state.saveStatus = 'conflict';
      App.state.conflictOpen = true;
      this.updateStatus();
      ConflictDialog.show();
    }
  },
};
