var saveTimer = null;
var editorInstance = null;
var sourceField = null;
var fallbackField = null;

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
    sourceField = document.getElementById('editor-source');
    fallbackField = document.getElementById('editor-fallback');
    var root = document.getElementById('editor-root');
    if (!sourceField || !root) return;
    if (window.toastui && window.toastui.Editor) {
        try {
            editorInstance = new window.toastui.Editor({
                el: root,
                initialValue: sourceField.value,
                initialEditType: 'wysiwyg',
                previewStyle: 'tab',
                theme: 'dark',
                usageStatistics: false,
                toolbarItems: [
                    ['heading', 'bold', 'italic', 'strike'],
                    ['ul', 'ol', 'task'],
                    ['quote', 'code', 'codeblock'],
                    ['link']
                ]
            });
            root.querySelector('.toastui-editor-mode-switch')?.remove();
            editorInstance.on('change', onEditorInput);
        } catch (_) {
            enableFallback(root);
        }
    } else {
        enableFallback(root);
    }
    syncNoteChrome();
}

function enableFallback(root) {
    editorInstance = null;
    if (root) root.hidden = true;
    if (!fallbackField) return;
    fallbackField.hidden = false;
    fallbackField.addEventListener('input', onEditorInput);
}

function onEditorInput() {
    syncNoteChrome();
    queueSave();
}

function currentBody() {
    if (editorInstance) {
        sourceField.value = editorInstance.getMarkdown();
        return sourceField.value;
    }
    if (fallbackField && !fallbackField.hidden) {
        sourceField.value = fallbackField.value;
    }
    return sourceField ? sourceField.value : '';
}

function queueSave() {
    clearTimeout(saveTimer);
    saveTimer = setTimeout(saveNote, 500);
}

function saveNote() {
    if (!sourceField || typeof currentId === 'undefined') return;
    fetch('/records/' + currentId, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ body: currentBody(), is_private: isPrivate })
    })
        .then(function (response) {
            if (!response.ok) throw new Error('save failed');
            setSaveError('');
        })
        .catch(function () {
            setSaveError('Save failed. Retry on the next change.');
        });
}

function setSaveError(message) {
    var node = document.getElementById('save-error');
    if (!node) return;
    node.textContent = message;
    node.hidden = !message;
}

function togglePublic() {
    var checkbox = document.getElementById('public-toggle');
    isPrivate = !checkbox.checked;
    syncNoteChrome();
    queueSave();
}

function syncNoteChrome() {
    var body = currentBody();
    var title = deriveTitle(body);
    var visibility = isPrivate ? 'Private' : 'Public';
    document.querySelectorAll('[data-live-title]').forEach(function (node) {
        node.textContent = title;
    });
    document.querySelectorAll('[data-live-visibility]').forEach(function (node) {
        node.textContent = visibility;
    });
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
