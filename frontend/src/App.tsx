import React, { useEffect, useMemo, useState } from 'react';
import './App.css';

type Session = { user_id: string; email: string; csrf_token: string };
type Workspace = { id: string; slug: string; name: string };
type Project = { id: string; workspace_id: string; name: string; description: string };
type Note = {
  id: string;
  workspace_id?: string;
  project_id?: string;
  title: string;
  note_kind: string;
  access_scope: string;
  current_version: number;
};
type Projection = {
  note_id: string;
  workspace_id?: string;
  project_id?: string;
  title: string;
  note_kind: string;
  version: number;
  markdown: string;
  metadata_json: Record<string, unknown>;
  tags: string[];
};
type DashboardWidget = { id: string; workspace_id: string; widget_type: string; config_json: Record<string, unknown> };
type Backlink = Note;
type ApiError = Error & { status?: number; body?: any };

const createApiError = async (res: Response): Promise<ApiError> => {
  const text = await res.text();
  let body: any = undefined;
  try {
    body = text ? JSON.parse(text) : undefined;
  } catch {
    body = text;
  }
  const err = new Error(body?.message || text || `HTTP ${res.status}`) as ApiError;
  err.status = res.status;
  err.body = body;
  return err;
};

const idempotencyKey = () => {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID();
  }
  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
};

const api = async <T,>(path: string, init?: RequestInit): Promise<T> => {
  const res = await fetch(`/api${path}`, { credentials: 'include', ...init });
  if (!res.ok) throw await createApiError(res);
  if (res.status === 204) return null as T;
  return await res.json();
};

type Command = { id: string; label: string; run: () => Promise<void> };

export function App() {
  const [session, setSession] = useState<Session | null>(null);
  const [email, setEmail] = useState('owner@example.com');
  const [password, setPassword] = useState('change-me-1234');
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const [workspaceId, setWorkspaceId] = useState<string>('');
  const [projects, setProjects] = useState<Project[]>([]);
  const [newProjectName, setNewProjectName] = useState('');
  const [notes, setNotes] = useState<Note[]>([]);
  const [search, setSearch] = useState('');
  const [current, setCurrent] = useState<Projection | null>(null);
  const [draftTitle, setDraftTitle] = useState('');
  const [draftMarkdown, setDraftMarkdown] = useState('');
  const [backlinks, setBacklinks] = useState<Backlink[]>([]);
  const [widgets, setWidgets] = useState<DashboardWidget[]>([]);
  const [activity, setActivity] = useState<string[]>([]);
  const [status, setStatus] = useState('idle');
  const [showPalette, setShowPalette] = useState(false);
  const [paletteQuery, setPaletteQuery] = useState('');
  const [saveTick, setSaveTick] = useState(0);

  const dirty = !!current && (draftTitle !== current.title || draftMarkdown !== current.markdown);

  useEffect(() => {
    const onKey = (ev: KeyboardEvent) => {
      if ((ev.ctrlKey || ev.metaKey) && ev.key.toLowerCase() === 'k') {
        ev.preventDefault();
        setShowPalette((x) => !x);
      }
      if (ev.key === 'Escape') setShowPalette(false);
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, []);

  useEffect(() => {
    void loadSession();
  }, []);

  useEffect(() => {
    if (!session) return;
    void loadWorkspaces();
  }, [session]);

  useEffect(() => {
    if (!session || !workspaceId) return;
    void Promise.all([loadProjects(workspaceId), loadNotes(workspaceId), loadWidgets(workspaceId)]);
  }, [session, workspaceId]);

  useEffect(() => {
    if (!current) {
      setBacklinks([]);
      return;
    }
    setDraftTitle(current.title);
    setDraftMarkdown(current.markdown);
    void loadBacklinks(current.note_id);
  }, [current?.note_id]);

  useEffect(() => {
    if (!dirty || !current || !session) return;
    const timeout = window.setTimeout(() => {
      void saveDraft();
    }, 800);
    return () => window.clearTimeout(timeout);
  }, [dirty, draftTitle, draftMarkdown, current?.note_id, saveTick]);

  useEffect(() => {
    if (!session || !workspaceId || !current) return;
    const ws = new WebSocket(`${location.protocol === 'https:' ? 'wss' : 'ws'}://${location.host}/ws`);
    ws.onopen = () => {
      ws.send(JSON.stringify({ type: 'subscribe_workspace', workspace_id: workspaceId }));
      ws.send(JSON.stringify({ type: 'subscribe_note', note_id: current.note_id }));
    };
    ws.onmessage = (ev) => {
      const msg = JSON.parse(ev.data);
      if (msg.type === 'note_event' && msg.note_id === current.note_id) {
        const payload = msg.payload || {};
        const nextMarkdown = payload.markdown ?? payload.markdown_after;
        const nextTitle = payload.title ?? payload.title_after;
        setCurrent((prev) => {
          if (!prev || prev.note_id !== msg.note_id) return prev;
          return {
            ...prev,
            version: msg.version ?? prev.version,
            markdown: typeof nextMarkdown === 'string' ? nextMarkdown : prev.markdown,
            title: typeof nextTitle === 'string' ? nextTitle : prev.title,
          };
        });
      }
      if (msg.type === 'workspace_event') {
        setActivity((prev) => [
          `${new Date().toLocaleTimeString()} ${msg.event_type}`,
          ...prev.slice(0, 19),
        ]);
      }
      if (msg.type === 'automation_event') {
        setActivity((prev) => [
          `${new Date().toLocaleTimeString()} automation ${msg.status}`,
          ...prev.slice(0, 19),
        ]);
      }
    };
    return () => ws.close();
  }, [session?.user_id, workspaceId, current?.note_id]);

  const commands = useMemo<Command[]>(() => {
    return [
      {
        id: 'new-note',
        label: 'Create Note',
        run: async () => {
          await createNote();
          setShowPalette(false);
        },
      },
      {
        id: 'refresh-notes',
        label: 'Refresh Notes',
        run: async () => {
          if (workspaceId) await loadNotes(workspaceId);
          setShowPalette(false);
        },
      },
      {
        id: 'tag-inbox',
        label: 'Tag Note #inbox',
        run: async () => {
          if (!current || !session) return;
          await api(`/notes/${current.note_id}/tags`, {
            method: 'PUT',
            headers: withCsrf(session),
            body: JSON.stringify({ tags: ['inbox'] }),
          });
          setStatus('tagged as inbox');
          setShowPalette(false);
        },
      },
      {
        id: 'run-rule',
        label: 'Run Demo Rule',
        run: async () => {
          if (!workspaceId || !session) return;
          await api('/automation/rules', {
            method: 'POST',
            headers: withCsrf(session),
            body: JSON.stringify({
              workspace_id: workspaceId,
              trigger: 'manual',
              condition_json: { source: 'palette' },
              action_json: { type: 'noop' },
              enabled: true,
            }),
          });
          setStatus('rule created');
          setShowPalette(false);
        },
      },
    ];
  }, [session, workspaceId, current?.note_id]);

  const filteredCommands = commands.filter((cmd) =>
    cmd.label.toLowerCase().includes(paletteQuery.toLowerCase().trim()),
  );

  async function loadSession() {
    try {
      const s = await api<Session>('/auth/session');
      setSession(s);
    } catch (err) {
      const e = err as ApiError;
      if (e.status === 401) {
        setSession(null);
        return;
      }
      setStatus(e.message);
      setSession(null);
    }
  }

  async function registerOrLogin(setup: boolean) {
    setStatus('auth...');
    try {
      const path = setup ? '/setup/register' : '/auth/login';
      const s = await api<Session>(path, {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ email, password, display_name: email.split('@')[0] }),
      });
      setSession(s);
      setStatus('authenticated');
    } catch (err) {
      setStatus((err as Error).message);
    }
  }

  async function loadWorkspaces() {
    const rows = await api<Workspace[]>('/workspaces');
    setWorkspaces(rows);
    if (!workspaceId && rows.length > 0) setWorkspaceId(rows[0].id);
  }

  async function loadProjects(wid: string) {
    const rows = await api<Project[]>(`/projects?workspace_id=${wid}`);
    setProjects(rows);
  }

  async function loadNotes(wid: string) {
    const endpoint = search.trim()
      ? `/search?q=${encodeURIComponent(search.trim())}&workspace_id=${wid}`
      : `/notes?workspace_id=${wid}`;
    const rows = await api<Note[]>(endpoint);
    setNotes(rows);
  }

  async function loadWidgets(wid: string) {
    const rows = await api<DashboardWidget[]>(`/dashboards?workspace_id=${wid}`);
    setWidgets(rows);
  }

  async function loadBacklinks(noteId: string) {
    const rows = await api<Backlink[]>(`/notes/${noteId}/backlinks`);
    setBacklinks(rows);
  }

  async function createProject() {
    if (!session || !workspaceId || !newProjectName.trim()) return;
    await api('/projects', {
      method: 'POST',
      headers: withCsrf(session),
      body: JSON.stringify({ workspace_id: workspaceId, name: newProjectName.trim(), description: '' }),
    });
    setNewProjectName('');
    await loadProjects(workspaceId);
  }

  async function createNote(projectId?: string) {
    if (!session || !workspaceId) return;
    const projection = await api<Projection>('/notes', {
      method: 'POST',
      headers: withCsrf(session),
      body: JSON.stringify({
        workspace_id: workspaceId,
        project_id: projectId ?? null,
        title: `Note ${new Date().toISOString()}`,
        markdown: '',
        note_kind: 'markdown',
        access_scope: 'workspace',
      }),
    });
    setCurrent(projection);
    await loadNotes(workspaceId);
  }

  async function openNote(id: string) {
    const projection = await api<Projection>(`/notes/${id}`);
    setCurrent(projection);
  }

  async function saveDraft() {
    if (!session || !current || !dirty) return;
    setStatus('autosaving...');
    try {
      let latest = current;
      if (draftTitle !== current.title) {
        latest = await api<Projection>(`/notes/${current.note_id}/title`, {
          method: 'PATCH',
          headers: withCsrf(session),
          body: JSON.stringify({
            base_version: current.version,
            title: draftTitle,
            idempotency_key: idempotencyKey(),
          }),
        });
      }
      if (draftMarkdown !== latest.markdown) {
        latest = await api<Projection>(`/notes/${current.note_id}`, {
          method: 'PATCH',
          headers: withCsrf(session),
          body: JSON.stringify({
            base_version: latest.version,
            patch_ops: [{ delete: latest.markdown.length }, { insert: draftMarkdown }],
            idempotency_key: idempotencyKey(),
          }),
        });
      }
      setCurrent(latest);
      setStatus(`saved v${latest.version}`);
      await loadNotes(workspaceId);
    } catch (err) {
      const e = err as ApiError;
      if (e.status === 409 && e.body?.latest) {
        const latest = e.body.latest as Projection;
        setCurrent(latest);
        setDraftTitle(latest.title);
        setDraftMarkdown(latest.markdown);
        setStatus('conflict detected; reloaded latest');
        return;
      }
      setStatus(e.message);
    }
  }

  async function deleteCurrentNote() {
    if (!session || !current) return;
    const ok = window.confirm('Delete this note?');
    if (!ok) return;
    await api(`/notes/${current.note_id}`, {
      method: 'DELETE',
      headers: withCsrf(session),
    });
    setCurrent(null);
    await loadNotes(workspaceId);
    setStatus('note deleted');
  }

  async function upsertWidget() {
    if (!session || !workspaceId) return;
    await api('/dashboards/widgets', {
      method: 'POST',
      headers: withCsrf(session),
      body: JSON.stringify({
        workspace_id: workspaceId,
        type: 'recent_changes',
        config_json: { title: 'Recent Changes' },
        layout: { x: 0, y: 0, w: 4, h: 3 },
      }),
    });
    await loadWidgets(workspaceId);
  }

  const selectedProjectNotes = notes.filter((n) => !n.project_id || projects.some((p) => p.id === n.project_id));

  return (
    <div className="app-shell">
      <aside className="nav-pane">
        <header>
          <h1>kjxlkj Workspace</h1>
          <p>{status}</p>
        </header>

        {!session && (
          <section className="card">
            <input value={email} onChange={(e) => setEmail(e.target.value)} placeholder="email" />
            <input value={password} onChange={(e) => setPassword(e.target.value)} type="password" placeholder="password" />
            <div className="row">
              <button onClick={() => void registerOrLogin(true)}>Setup</button>
              <button onClick={() => void registerOrLogin(false)}>Login</button>
            </div>
          </section>
        )}

        {session && (
          <>
            <section className="card">
              <label>Workspace</label>
              <select value={workspaceId} onChange={(e) => setWorkspaceId(e.target.value)}>
                {workspaces.map((w) => (
                  <option key={w.id} value={w.id}>
                    {w.name}
                  </option>
                ))}
              </select>
              <button onClick={() => setShowPalette(true)}>Command Palette (Ctrl/Cmd+K)</button>
            </section>

            <section className="card">
              <label>Project Rail</label>
              <div className="row">
                <input
                  value={newProjectName}
                  onChange={(e) => setNewProjectName(e.target.value)}
                  placeholder="new project"
                />
                <button onClick={() => void createProject()}>Add</button>
              </div>
              <ul>
                {projects.map((p) => (
                  <li key={p.id}>{p.name}</li>
                ))}
              </ul>
            </section>

            <section className="card">
              <label>Notes</label>
              <div className="row">
                <input
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  onBlur={() => void loadNotes(workspaceId)}
                  placeholder="search"
                />
                <button onClick={() => void createNote()}>New</button>
              </div>
              <ul>
                {selectedProjectNotes.map((n) => (
                  <li key={n.id}>
                    <button className="linklike" onClick={() => void openNote(n.id)}>
                      {n.title}
                    </button>
                  </li>
                ))}
              </ul>
            </section>
          </>
        )}
      </aside>

      <main className="editor-pane">
        {!session && <p className="empty">Authenticate to start.</p>}

        {session && !current && (
          <section className="empty card">
            <h2>Dashboard</h2>
            <p>Workspace widgets, jobs, and activity are shown here.</p>
            <button onClick={() => void upsertWidget()}>Add Recent Changes Widget</button>
            <ul>
              {widgets.map((w) => (
                <li key={w.id}>{w.widget_type}</li>
              ))}
            </ul>
          </section>
        )}

        {session && current && (
          <section className="editor-grid">
            <article className="card edit-surface">
              <input
                className="title-input"
                value={draftTitle}
                onChange={(e) => {
                  setDraftTitle(e.target.value);
                  setSaveTick((x) => x + 1);
                }}
              />
              <textarea
                value={draftMarkdown}
                onChange={(e) => {
                  setDraftMarkdown(e.target.value);
                  setSaveTick((x) => x + 1);
                }}
              />
              <div className="row">
                <span>version {current.version}</span>
                <button onClick={() => void saveDraft()}>Save Now</button>
                <button className="danger" onClick={() => void deleteCurrentNote()}>
                  Delete
                </button>
              </div>
            </article>

            <aside className="card side-surfaces">
              <h3>Graph Explorer (Backlinks)</h3>
              <ul>
                {backlinks.map((b) => (
                  <li key={b.id}>
                    <button className="linklike" onClick={() => void openNote(b.id)}>
                      {b.title}
                    </button>
                  </li>
                ))}
              </ul>

              <h3>Workspace Activity</h3>
              <ul>
                {activity.map((a, idx) => (
                  <li key={idx}>{a}</li>
                ))}
              </ul>
            </aside>
          </section>
        )}
      </main>

      {showPalette && (
        <div className="palette-overlay" onClick={() => setShowPalette(false)}>
          <div className="palette" onClick={(e) => e.stopPropagation()}>
            <input
              autoFocus
              value={paletteQuery}
              onChange={(e) => setPaletteQuery(e.target.value)}
              placeholder="Type a command"
            />
            <ul>
              {filteredCommands.map((cmd) => (
                <li key={cmd.id}>
                  <button className="linklike" onClick={() => void cmd.run()}>
                    {cmd.label}
                  </button>
                </li>
              ))}
            </ul>
          </div>
        </div>
      )}
    </div>
  );
}

function withCsrf(session: Session) {
  return {
    'content-type': 'application/json',
    'x-csrf-token': session.csrf_token,
  };
}
