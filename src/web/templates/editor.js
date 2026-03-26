var saveTimer = null;

function createNote() {
    fetch('/records', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({})
    })
    .then(function (response) { return response.json(); })
    .then(function (note) { window.location.href = '/' + note.id; })
    .catch(function (err) { alert('Failed to create note: ' + err.message); });
}

function initEditor() {
    syncNoteChrome();
    var editor = document.getElementById('editor');
    if (!editor) return;
    editor.addEventListener('input', function () {
        syncNoteChrome();
        queueSave();
    });
}

function queueSave() {
    clearTimeout(saveTimer);
    saveTimer = setTimeout(saveNote, 500);
}

function saveNote() {
    var editor = document.getElementById('editor');
    if (!editor || typeof currentId === 'undefined') return;
    var status = document.getElementById('save-status');
    status.textContent = 'Saving';
    status.dataset.state = 'saving';
    fetch('/records/' + currentId, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ body: editor.value, is_private: isPrivate })
    })
    .then(function (response) {
        if (!response.ok) throw new Error('save failed');
        status.textContent = 'Saved';
        status.dataset.state = 'saved';
        setTimeout(function () { status.textContent = ''; }, 1500);
    })
    .catch(function () {
        status.textContent = 'Save failed';
        status.dataset.state = 'error';
    });
}

function togglePublic() {
    var checkbox = document.getElementById('public-toggle');
    isPrivate = !checkbox.checked;
    syncNoteChrome();
    queueSave();
}

function syncNoteChrome() {
    var editor = document.getElementById('editor');
    var title = deriveTitle(editor ? editor.value : '');
    var visibility = isPrivate ? 'Private' : 'Public';
    document.querySelectorAll('[data-live-title]').forEach(function (node) { node.textContent = title; });
    document.querySelectorAll('[data-live-visibility]').forEach(function (node) { node.textContent = visibility; });
    document.title = title + ' - kjxlkj';
}

function deriveTitle(body) {
    var match = body.match(/^\s*#\s+(.+)$/m);
    return match && match[1] ? match[1].trim() : 'Untitled note';
}

function deleteNote(id) {
    if (!confirm('Delete this note?')) return;
    fetch('/records/' + id, { method: 'DELETE' })
    .then(function (response) {
        if (!response.ok) throw new Error('delete failed');
        window.location.href = '/admin';
    })
    .catch(function () { alert('Failed to delete note'); });
}
