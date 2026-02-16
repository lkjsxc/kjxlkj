import { LitElement, html, css, PropertyValues } from "lit";
import { customElement, state } from "lit/decorators.js";
import { api, SessionInfo, NoteStream, Workspace } from "../api.js";
import { WsClient } from "../ws.js";

type AppView = "setup" | "login" | "notes" | "note-detail";

/**
 * Root application shell for kjxlkj.
 * Manages auth state, navigation, and view routing.
 */
@customElement("kjxlkj-app")
export class AppShell extends LitElement {
  @state() private view: AppView = "login";
  @state() private session: SessionInfo | null = null;
  @state() private workspaces: Workspace[] = [];
  @state() private activeWorkspace: Workspace | null = null;
  @state() private notes: NoteStream[] = [];
  @state() private selectedNoteId: string | null = null;
  @state() private noteMarkdown = "";
  @state() private noteVersion = 0;
  @state() private searchQuery = "";
  @state() private menuOpen = false;
  @state() private error = "";

  private ws = new WsClient();
  private autosaveTimer: ReturnType<typeof setTimeout> | null = null;

  static styles = css`
    :host {
      display: flex;
      flex-direction: column;
      height: 100vh;
      font-family: system-ui, -apple-system, sans-serif;
      color: #e0e0e0;
      background: #1a1a2e;
    }
    header {
      display: flex;
      align-items: center;
      padding: 0.5rem 1rem;
      background: #16213e;
      border-bottom: 1px solid #0f3460;
      flex-shrink: 0;
    }
    header h1 {
      font-size: 1.2rem;
      margin: 0;
      flex: 1;
    }
    .menu-btn {
      display: none;
      background: none;
      border: 1px solid #0f3460;
      color: #e0e0e0;
      padding: 0.4rem 0.8rem;
      cursor: pointer;
      border-radius: 4px;
    }
    @media (max-width: 1280px) {
      .menu-btn { display: block; }
    }
    .main {
      display: flex;
      flex: 1;
      overflow: hidden;
    }
    .sidebar {
      width: 280px;
      background: #16213e;
      border-right: 1px solid #0f3460;
      display: flex;
      flex-direction: column;
      overflow-y: auto;
      flex-shrink: 0;
    }
    @media (max-width: 1280px) {
      .sidebar {
        position: absolute;
        left: 0;
        top: 48px;
        bottom: 0;
        z-index: 10;
        transform: translateX(-100%);
        transition: transform 0.2s;
      }
      .sidebar.open { transform: translateX(0); }
    }
    .sidebar input {
      margin: 0.5rem;
      padding: 0.4rem;
      background: #0f3460;
      border: 1px solid #533483;
      color: #e0e0e0;
      border-radius: 4px;
    }
    .note-list { list-style: none; margin: 0; padding: 0; }
    .note-list li {
      padding: 0.6rem 1rem;
      cursor: pointer;
      border-bottom: 1px solid #0f3460;
    }
    .note-list li:hover { background: #0f3460; }
    .note-list li.active { background: #533483; }
    .content {
      flex: 1;
      display: flex;
      flex-direction: column;
      overflow: hidden;
    }
    .editor-area {
      flex: 1;
      padding: 1rem;
      overflow-y: auto;
    }
    textarea {
      width: 100%;
      height: 100%;
      background: transparent;
      color: #e0e0e0;
      border: none;
      resize: none;
      font-family: 'JetBrains Mono', 'Fira Code', monospace;
      font-size: 0.95rem;
      line-height: 1.6;
      outline: none;
    }
    .auth-form {
      max-width: 360px;
      margin: 4rem auto;
      padding: 2rem;
      background: #16213e;
      border-radius: 8px;
      border: 1px solid #0f3460;
    }
    .auth-form input {
      display: block;
      width: 100%;
      margin-bottom: 1rem;
      padding: 0.5rem;
      background: #0f3460;
      border: 1px solid #533483;
      color: #e0e0e0;
      border-radius: 4px;
      box-sizing: border-box;
    }
    .auth-form button, .toolbar button {
      padding: 0.5rem 1rem;
      background: #533483;
      color: #e0e0e0;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }
    .auth-form button:hover, .toolbar button:hover {
      background: #e94560;
    }
    .toolbar {
      padding: 0.5rem 1rem;
      display: flex;
      gap: 0.5rem;
      border-bottom: 1px solid #0f3460;
      flex-shrink: 0;
    }
    .error { color: #e94560; margin: 0.5rem 0; font-size: 0.9rem; }
    .user-info { font-size: 0.85rem; color: #a0a0c0; }
  `;

  async connectedCallback(): Promise<void> {
    super.connectedCallback();
    await this.checkSession();
  }

  disconnectedCallback(): void {
    super.disconnectedCallback();
    this.ws.disconnect();
    if (this.autosaveTimer) clearTimeout(this.autosaveTimer);
  }

  private async checkSession(): Promise<void> {
    try {
      this.session = await api.getSession();
      this.view = "notes";
      await this.loadWorkspaces();
      this.ws.connect();
    } catch {
      this.session = null;
      this.view = "login";
    }
  }

  private async loadWorkspaces(): Promise<void> {
    try {
      this.workspaces = await api.listWorkspaces();
      if (this.workspaces.length > 0 && !this.activeWorkspace) {
        this.activeWorkspace = this.workspaces[0];
        await this.loadNotes();
      }
    } catch (e) {
      this.error = String(e);
    }
  }

  private async loadNotes(): Promise<void> {
    if (!this.activeWorkspace) return;
    try {
      this.notes = await api.listNotes(this.activeWorkspace.id);
    } catch (e) {
      this.error = String(e);
    }
  }

  private async handleLogin(e: Event): Promise<void> {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const data = new FormData(form);
    try {
      this.error = "";
      this.session = await api.login(
        data.get("username") as string,
        data.get("password") as string
      );
      this.view = "notes";
      await this.loadWorkspaces();
      this.ws.connect();
    } catch (err) {
      this.error = String(err);
    }
  }

  private async handleSetup(e: Event): Promise<void> {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const data = new FormData(form);
    try {
      this.error = "";
      this.session = await api.register(
        data.get("username") as string,
        data.get("password") as string,
        data.get("display_name") as string
      );
      this.view = "notes";
      await this.loadWorkspaces();
      this.ws.connect();
    } catch (err) {
      this.error = String(err);
    }
  }

  private async selectNote(noteId: string): Promise<void> {
    this.selectedNoteId = noteId;
    this.view = "note-detail";
    this.menuOpen = false;
    try {
      const data = await api.getNote(noteId);
      this.noteMarkdown = data.projection?.markdown ?? "";
      this.noteVersion = data.projection?.version ?? 0;
    } catch (e) {
      this.error = String(e);
    }
  }

  private async createNote(): Promise<void> {
    if (!this.activeWorkspace || !this.session) return;
    try {
      const note = await api.createNote(
        this.activeWorkspace.id,
        "",
        "markdown",
        this.session.csrf_token
      );
      await this.loadNotes();
      await this.selectNote(note.id);
    } catch (e) {
      this.error = String(e);
    }
  }

  private onEditorInput(e: Event): void {
    this.noteMarkdown = (e.target as HTMLTextAreaElement).value;
    this.scheduleAutosave();
  }

  private scheduleAutosave(): void {
    if (this.autosaveTimer) clearTimeout(this.autosaveTimer);
    this.autosaveTimer = setTimeout(() => this.saveNote(), 600);
  }

  private async saveNote(): Promise<void> {
    if (!this.selectedNoteId || !this.session) return;
    try {
      const result = await api.patchNote(
        this.selectedNoteId,
        this.noteVersion,
        this.noteMarkdown,
        this.session.csrf_token
      );
      this.noteVersion = result.version;
    } catch (e) {
      this.error = String(e);
    }
  }

  private async handleLogout(): Promise<void> {
    if (!this.session) return;
    try {
      await api.logout(this.session.csrf_token);
    } catch {
      // ignore
    }
    this.session = null;
    this.view = "login";
    this.ws.disconnect();
  }

  render() {
    switch (this.view) {
      case "setup":
        return this.renderSetup();
      case "login":
        return this.renderLogin();
      case "notes":
      case "note-detail":
        return this.renderMain();
    }
  }

  private renderLogin() {
    return html`
      <div class="auth-form">
        <h2>Login</h2>
        ${this.error ? html`<p class="error">${this.error}</p>` : ""}
        <form @submit=${this.handleLogin}>
          <input name="username" placeholder="Username" required />
          <input name="password" type="password" placeholder="Password" required />
          <button type="submit">Login</button>
        </form>
        <p style="margin-top:1rem;font-size:0.85rem;">
          First time? <a href="#" @click=${() => { this.view = "setup"; }} style="color:#533483">Setup owner</a>
        </p>
      </div>
    `;
  }

  private renderSetup() {
    return html`
      <div class="auth-form">
        <h2>Owner Setup</h2>
        ${this.error ? html`<p class="error">${this.error}</p>` : ""}
        <form @submit=${this.handleSetup}>
          <input name="username" placeholder="Username" required />
          <input name="display_name" placeholder="Display Name" required />
          <input name="password" type="password" placeholder="Password" required />
          <button type="submit">Create Owner</button>
        </form>
        <p style="margin-top:1rem;font-size:0.85rem;">
          Already set up? <a href="#" @click=${() => { this.view = "login"; }} style="color:#533483">Login</a>
        </p>
      </div>
    `;
  }

  private renderMain() {
    const filteredNotes = this.searchQuery
      ? this.notes.filter((n) =>
          n.title.toLowerCase().includes(this.searchQuery.toLowerCase())
        )
      : this.notes;

    return html`
      <header>
        <button class="menu-btn" @click=${() => { this.menuOpen = !this.menuOpen; }}>Menu</button>
        <h1>kjxlkj</h1>
        <span class="user-info">${this.session?.display_name ?? ""}</span>
        <button class="menu-btn" style="margin-left:0.5rem" @click=${this.handleLogout}>Logout</button>
      </header>
      <div class="main">
        <aside class="sidebar ${this.menuOpen ? "open" : ""}">
          <input
            type="text"
            placeholder="Search notes..."
            .value=${this.searchQuery}
            @input=${(e: Event) => { this.searchQuery = (e.target as HTMLInputElement).value; }}
          />
          <div class="toolbar">
            <button @click=${this.createNote}>+ New Note</button>
          </div>
          <ul class="note-list">
            ${filteredNotes.map(
              (n) => html`
                <li
                  class=${n.id === this.selectedNoteId ? "active" : ""}
                  @click=${() => this.selectNote(n.id)}
                >
                  ${n.title || "Untitled"}
                </li>
              `
            )}
          </ul>
        </aside>
        <div class="content">
          ${this.view === "note-detail" && this.selectedNoteId
            ? this.renderEditor()
            : html`<div class="editor-area"><p style="color:#a0a0c0">Select or create a note</p></div>`}
        </div>
      </div>
    `;
  }

  private renderEditor() {
    return html`
      <div class="editor-area">
        ${this.error ? html`<p class="error">${this.error}</p>` : ""}
        <textarea
          .value=${this.noteMarkdown}
          @input=${this.onEditorInput}
          placeholder="Start writing..."
        ></textarea>
      </div>
    `;
  }
}
