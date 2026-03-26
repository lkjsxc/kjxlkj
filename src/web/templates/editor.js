var saveTimer = null;
var currentMode = 'text';

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
    var surface = document.querySelector('[data-initial-mode]');
    var editor = document.getElementById('editor');
    var rich = document.getElementById('rich-editor');
    if (!surface || !editor) return;
    currentMode = surface.dataset.initialMode || 'text';
    editor.addEventListener('input', function () {
        updateRichButton();
        syncNoteChrome();
        queueSave();
    });
    rich?.addEventListener('input', function () {
        syncSourceFromRich();
        syncNoteChrome();
        queueSave();
    });
    document.addEventListener('click', function (event) {
        var mode = event.target.closest('[data-mode-button]')?.dataset.modeButton;
        var block = event.target.closest('[data-add-block]')?.dataset.addBlock;
        if (mode) setMode(mode);
        if (block && rich) {
            setMode('rich');
            window.richMarkdown.addBlock(rich, block);
            syncSourceFromRich();
            syncNoteChrome();
            queueSave();
        }
    });
    setMode(currentMode);
    syncNoteChrome();
}

function queueSave() {
    clearTimeout(saveTimer);
    saveTimer = setTimeout(saveNote, 500);
}

function saveNote() {
    var editor = document.getElementById('editor');
    if (!editor || typeof currentId === 'undefined') return;
    if (currentMode === 'rich') syncSourceFromRich();
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

function setMode(mode) {
    var surface = document.querySelector('[data-initial-mode]');
    var rich = document.getElementById('rich-editor');
    var editor = document.getElementById('editor');
    if (!surface || !editor || !rich) return;
    if (mode === 'rich') {
        var blocks = window.richMarkdown.parseBlocks(editor.value);
        if (!blocks) return updateRichButton();
        rich.innerHTML = blocks.map(window.richMarkdown.blockHtml).join('');
        syncSourceFromRich();
    } else if (currentMode === 'rich') {
        syncSourceFromRich();
    }
    currentMode = mode;
    rich.hidden = mode !== 'rich';
    editor.hidden = mode === 'rich';
    document.querySelector('.editor-actions').hidden = mode !== 'rich';
    document.querySelectorAll('[data-mode-button]').forEach(function (node) {
        node.classList.toggle('active', node.dataset.modeButton === mode);
    });
    syncNoteChrome();
}

function updateRichButton() {
    var editor = document.getElementById('editor');
    var button = document.querySelector('[data-mode-button="rich"]');
    if (!editor || !button) return;
    button.disabled = !window.richMarkdown.parseBlocks(editor.value);
}

function syncSourceFromRich() {
    var editor = document.getElementById('editor');
    var rich = document.getElementById('rich-editor');
    if (editor && rich) editor.value = window.richMarkdown.serializeRich(rich);
}

function togglePublic() {
    var checkbox = document.getElementById('public-toggle');
    isPrivate = !checkbox.checked;
    syncNoteChrome();
    queueSave();
}

function syncNoteChrome() {
    var editor = document.getElementById('editor');
    if (!editor) return;
    if (currentMode === 'rich') syncSourceFromRich();
    var title = deriveTitle(editor.value);
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
