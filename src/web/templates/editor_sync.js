function queueSave() {
    clearTimeout(editorState.saveTimer);
    if (!isDirty(currentBody(), isPrivate)) return;
    editorState.saveTimer = setTimeout(saveNote, 500);
}

function isDirty(body, nextPrivate) {
    return body !== editorState.lastSavedBody || nextPrivate !== editorState.lastSavedPrivate;
}

function saveNote() {
    if (!editorState.sourceField || typeof currentId === 'undefined') return;
    var body = currentBody();
    var nextPrivate = isPrivate;
    if (!isDirty(body, nextPrivate)) return;
    var requestId = ++editorState.latestRequest;
    fetch('/records/' + currentId, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ body: body, is_private: nextPrivate })
    })
        .then(function (response) {
            if (!response.ok) throw new Error('save failed');
            if (requestId !== editorState.latestRequest) return;
            editorState.lastSavedBody = body;
            editorState.lastSavedPrivate = nextPrivate;
            setSaveError('');
        })
        .catch(function () {
            if (requestId !== editorState.latestRequest) return;
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
    var nextPrivate = !checkbox.checked;
    if (nextPrivate === isPrivate) return;
    isPrivate = nextPrivate;
    syncNoteChrome();
    queueSave();
}

function syncNoteChrome() {
    var title = deriveTitle(currentBody());
    var visibility = isPrivate ? 'Private' : 'Public';
    updateLiveText('[data-live-title]', title, 'renderedTitle');
    updateLiveText('[data-live-visibility]', visibility, 'renderedVisibility');
    document.title = title + ' - kjxlkj';
}

function updateLiveText(selector, value, key) {
    if (editorState[key] === value) return;
    editorState[key] = value;
    document.querySelectorAll(selector).forEach(function (node) {
        node.textContent = value;
    });
}

function deriveTitle(body) {
    var match = body.match(/^\s*#\s+(.+)$/m);
    return match && match[1] ? match[1].trim() : 'Untitled note';
}
