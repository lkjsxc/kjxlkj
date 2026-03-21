function byId(id) {
  return document.getElementById(id);
}

function status(msg) {
  byId('status').textContent = msg;
}

function currentPayload() {
  return {
    slug: byId('slug').value.trim(),
    title: byId('title').value.trim(),
    body: byId('body').value,
    private: byId('private').checked,
  };
}

async function openArticle(slug) {
  const res = await fetch(`/admin/open/${encodeURIComponent(slug)}`);
  if (!res.ok) return status('open_failed');
  const a = await res.json();
  byId('slug').value = a.slug;
  byId('title').value = a.title;
  byId('body').value = a.body;
  byId('private').checked = a.private;
  status('opened');
}

async function createArticle() {
  const payload = currentPayload();
  const res = await fetch('/admin/create', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  status(res.ok ? 'created' : 'create_failed');
  if (res.ok) location.reload();
}

async function saveArticle() {
  const payload = currentPayload();
  const res = await fetch('/admin/save', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  status(res.ok ? 'saved' : 'save_failed');
  if (res.ok) location.reload();
}

async function renameArticle() {
  const from = byId('slug').value.trim();
  const to = prompt('New slug', from);
  if (!to) return;
  const res = await fetch('/admin/rename', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ from_slug: from, to_slug: to.trim() }),
  });
  status(res.ok ? 'renamed' : 'rename_failed');
  if (res.ok) {
    byId('slug').value = to.trim();
    location.reload();
  }
}

async function deleteArticle() {
  const slug = byId('slug').value.trim();
  if (!slug || !confirm(`Delete ${slug}?`)) return;
  const res = await fetch(`/admin/delete/${encodeURIComponent(slug)}`, { method: 'POST' });
  status(res.ok ? 'deleted' : 'delete_failed');
  if (res.ok) location.reload();
}

async function togglePrivate() {
  const slug = byId('slug').value.trim();
  if (!slug) return status('slug_required');
  const res = await fetch(`/admin/toggle-private/${encodeURIComponent(slug)}`, { method: 'POST' });
  status(res.ok ? 'toggled' : 'toggle_failed');
  if (res.ok) location.reload();
}
