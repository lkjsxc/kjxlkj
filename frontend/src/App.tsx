import React, { useEffect, useMemo, useState } from 'react';

type Session = { user_id: string; email: string; csrf_token: string };
type Note = { id: string; title: string; current_version: number };
type Projection = { note_id: string; title: string; version: number; markdown: string };

const api = async (path: string, init?: RequestInit) => {
  const res = await fetch(`/api/v1${path}`, { credentials: 'include', ...init });
  if (!res.ok) throw new Error(await res.text());
  if (res.status === 204) return null;
  return await res.json();
};

export function App() {
  const [session, setSession] = useState<Session | null>(null);
  const [email, setEmail] = useState('admin@example.com');
  const [password, setPassword] = useState('change-me-1234');
  const [notes, setNotes] = useState<Note[]>([]);
  const [current, setCurrent] = useState<Projection | null>(null);
  const [status, setStatus] = useState('idle');

  const ws = useMemo(() => {
    if (!session || !current) return null;
    const s = new WebSocket(`${location.protocol === 'https:' ? 'wss' : 'ws'}://${location.host}/ws/v1/notes`);
    s.onopen = () => s.send(JSON.stringify({ type: 'subscribe_note', note_id: current.note_id }));
    s.onmessage = (ev) => {
      const msg = JSON.parse(ev.data);
      if (msg.type === 'note_event' && msg.note_id === current.note_id && msg.payload?.markdown) {
        setCurrent({ ...current, version: msg.version, markdown: msg.payload.markdown });
      }
    };
    return s;
  }, [session, current?.note_id]);

  useEffect(() => {
    void loadSession();
  }, []);

  useEffect(() => {
    if (!session) return;
    void loadNotes();
  }, [session]);

  async function loadSession() {
    try {
      const s = await api('/auth/session');
      setSession(s);
    } catch {
      setSession(null);
    }
  }

  async function registerOrLogin(setup: boolean) {
    setStatus('auth');
    try {
      const path = setup ? '/setup/register' : '/auth/login';
      const s = await api(path, {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ email, password }),
      });
      setSession(s);
      setStatus('ok');
    } catch (e) {
      setStatus(String(e));
    }
  }

  async function loadNotes() {
    const data = await api('/notes');
    setNotes(data);
  }

  async function openNote(id: string) {
    const p = await api(`/notes/${id}`);
    setCurrent(p);
  }

  async function createNote() {
    if (!session) return;
    const p = await api('/notes', {
      method: 'POST',
      headers: { 'content-type': 'application/json', 'x-csrf-token': session.csrf_token },
      body: JSON.stringify({ title: `Note ${new Date().toISOString()}`, markdown: '' }),
    });
    await loadNotes();
    setCurrent(p);
  }

  async function saveMarkdown(markdown: string) {
    if (!session || !current) return;
    const ops = [{ delete: current.markdown.length }, { insert: markdown }];
    const p = await api(`/notes/${current.note_id}`, {
      method: 'PATCH',
      headers: { 'content-type': 'application/json', 'x-csrf-token': session.csrf_token },
      body: JSON.stringify({ base_version: current.version, patch_ops: ops, idempotency_key: crypto.randomUUID() }),
    });
    setCurrent(p);
    await loadNotes();
  }

  return (
    <main style={{ fontFamily: 'ui-sans-serif,system-ui', display: 'grid', gridTemplateColumns: '280px 1fr', height: '100vh' }}>
      <aside style={{ padding: 16, borderRight: '1px solid #ddd' }}>
        <h1>kjxlkj</h1>
        {!session && (
          <>
            <input value={email} onChange={(e) => setEmail(e.target.value)} placeholder="email" style={{ width: '100%', marginBottom: 8 }} />
            <input value={password} onChange={(e) => setPassword(e.target.value)} type="password" placeholder="password" style={{ width: '100%', marginBottom: 8 }} />
            <button onClick={() => void registerOrLogin(true)}>First-Run Register</button>
            <button onClick={() => void registerOrLogin(false)} style={{ marginLeft: 8 }}>Login</button>
            <p>{status}</p>
          </>
        )}
        {session && (
          <>
            <p>{session.email}</p>
            <button onClick={() => void createNote()}>New Note</button>
            <ul>
              {notes.map((n) => (
                <li key={n.id}><button onClick={() => void openNote(n.id)}>{n.title}</button></li>
              ))}
            </ul>
          </>
        )}
      </aside>
      <section style={{ padding: 16 }}>
        {current ? (
          <>
            <h2>{current.title}</h2>
            <textarea
              value={current.markdown}
              onChange={(e) => setCurrent({ ...current, markdown: e.target.value })}
              style={{ width: '100%', height: '70vh' }}
            />
            <div>
              <button onClick={() => void saveMarkdown(current.markdown)}>Save</button>
              <span style={{ marginLeft: 8 }}>v{current.version}</span>
            </div>
          </>
        ) : (
          <p>Select a note.</p>
        )}
      </section>
      {ws ? null : null}
    </main>
  );
}
