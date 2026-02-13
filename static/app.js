// kjxlkj workspace suite — core application bootstrap
// All modules are loaded via script tags; this is the entry point.
'use strict';

const App = {
  state: {
    user: null,
    setupLocked: null,
    notes: [],
    workspaces: [],
    currentNote: null,
    currentWorkspace: null,
    draft: '',
    syncedBody: '',
    syncedVersion: 0,
    saveStatus: 'saved', // saved | saving | conflict | offline
    navOpen: true,
    cmdOpen: false,
    conflictOpen: false,
    ws: null,
  },

  async init() {
    await this.checkSession();
    this.render();
    this.bindGlobalKeys();
  },

  // UX-AUTH-01: 401 is expected unauthenticated state, not a fatal error.
  async checkSession() {
    try {
      const r = await fetch('/api/auth/session');
      if (r.ok) {
        this.state.user = await r.json();
      } else if (r.status === 401) {
        this.state.user = null;
      }
    } catch { this.state.user = null; }
    // Check setup lock status.
    try {
      const r = await fetch('/api/healthz');
      if (r.ok) this.state.setupLocked = false; // server is up
    } catch { /* offline */ }
  },

  render() {
    const app = document.getElementById('app');
    if (!this.state.user) {
      app.innerHTML = AuthUI.render(this.state);
      AuthUI.bind();
    } else {
      app.innerHTML = ShellUI.render(this.state);
      ShellUI.bind();
      this.connectWs();
      this.loadWorkspaces();
    }
  },

  bindGlobalKeys() {
    document.addEventListener('keydown', (e) => {
      // UX-NAV-02: Ctrl+K opens command palette
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        this.state.cmdOpen = !this.state.cmdOpen;
        CmdPalette.toggle(this.state.cmdOpen);
      }
      // Ctrl+S saves explicitly
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        EditorUI.saveNow();
      }
    });
  },

  connectWs() {
    if (this.state.ws) return;
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    const ws = new WebSocket(`${proto}//${location.host}/ws`);
    ws.onmessage = (evt) => WsClient.onMessage(JSON.parse(evt.data));
    ws.onclose = () => { this.state.ws = null; setTimeout(() => this.connectWs(), 3000); };
    this.state.ws = ws;
  },

  async loadWorkspaces() {
    try {
      const r = await fetch('/api/workspaces');
      if (r.ok) {
        this.state.workspaces = await r.json();
        if (this.state.workspaces.length > 0 && !this.state.currentWorkspace) {
          this.state.currentWorkspace = this.state.workspaces[0];
          await this.loadNotes();
        }
        NavUI.refresh();
      }
    } catch { /* offline */ }
  },

  async loadNotes() {
    if (!this.state.currentWorkspace) return;
    try {
      const r = await fetch(`/api/notes?workspace_id=${this.state.currentWorkspace.id}`);
      if (r.ok) {
        this.state.notes = await r.json();
        NavUI.refresh();
      }
    } catch { /* offline */ }
  },

  // UX-EDIT-03: idempotency key without crypto.randomUUID
  genIdempotencyKey() {
    if (typeof crypto !== 'undefined' && crypto.randomUUID) return crypto.randomUUID();
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
      const r = Math.random() * 16 | 0;
      return (c === 'x' ? r : (r & 0x3 | 0x8)).toString(16);
    });
  },
};

document.addEventListener('DOMContentLoaded', () => App.init());
