function queueSave() {
    clearTimeout(editorState.saveTimer);
    if (!isDirty(currentBody(), draftAliasValue(), isFavorite, isPrivate)) return;
    editorState.saveTimer = setTimeout(saveNote, 500);
}

function isDirty(body, alias, favorite, nextPrivate) {
    return body !== editorState.lastSavedBody ||
        alias !== editorState.lastSavedAlias ||
        favorite !== editorState.lastSavedFavorite ||
        nextPrivate !== editorState.lastSavedPrivate;
}

function saveNote() {
    if (!editorState.bodyField || typeof currentId === 'undefined') return;
    var body = currentBody();
    var alias = draftAliasValue();
    var requestId = ++editorState.latestRequest;
    fetch('/records/' + currentId, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            body: body,
            alias: alias,
            is_favorite: isFavorite,
            is_private: isPrivate
        })
    })
        .then(readSaveResponse)
        .then(function (note) {
            if (requestId !== editorState.latestRequest) return;
            currentAlias = note.alias || null;
            currentHref = currentAlias ? '/' + currentAlias : '/' + note.id;
            isFavorite = !!note.is_favorite;
            isPrivate = !!note.is_private;
            editorState.lastSavedBody = body;
            editorState.lastSavedAlias = currentAlias;
            editorState.lastSavedFavorite = isFavorite;
            editorState.lastSavedPrivate = isPrivate;
            editorState.aliasField.value = currentAlias || '';
            editorState.publicToggle.checked = !isPrivate;
            editorState.favoriteToggle.checked = isFavorite;
            syncNoteChrome();
            setSaveError('');
        })
        .catch(function (error) {
            if (requestId !== editorState.latestRequest) return;
            setSaveError(error.message || 'Save failed. Retry on the next change.');
        });
}

function readSaveResponse(response) {
    if (response.ok) return response.json();
    return response.json()
        .then(
            function (payload) {
                throw new Error(payload.message || 'Save failed. Retry on the next change.');
            },
            function () { throw new Error('Save failed. Retry on the next change.'); }
        );
}

function setSaveError(message) {
    var node = document.getElementById('save-error');
    if (!node) return;
    node.textContent = message;
    node.hidden = !message;
}

function syncNoteChrome() {
    var title = deriveTitle(currentBody());
    var visibility = isPrivate ? 'Private' : 'Public';
    updateLiveText('[data-live-title]', title, 'renderedTitle');
    updateLiveText('[data-live-visibility]', visibility, 'renderedVisibility');
    updateLiveText('[data-live-alias]', currentAlias || 'None', 'renderedAlias');
    syncCanonicalLinks();
    document.title = title + ' - kjxlkj';
}

function syncCanonicalLinks() {
    var historyHref = currentHref + '/history';
    document.querySelectorAll('[data-current-note-link]').forEach(function (node) { node.href = currentHref; });
    document.querySelectorAll('[data-history-link]').forEach(function (node) { node.href = historyHref; });
    document.querySelectorAll('[data-current-url]').forEach(function (node) {
        node.href = currentHref;
        node.textContent = currentHref;
    });
    if (window.location.pathname !== currentHref) {
        window.history.replaceState({}, '', currentHref);
    }
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
