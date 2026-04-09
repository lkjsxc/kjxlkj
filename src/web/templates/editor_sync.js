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
    var selection = currentSelection();
    var requestId = ++editorState.latestRequest;
    fetch('/resources/' + currentId, {
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
            applySavedResource(note, selection);
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
    node.dataset.tone = message ? 'error' : '';
    node.hidden = !message;
}

function applySavedResource(note, selection) {
    currentAlias = note.alias || null;
    currentHref = currentAlias ? '/' + currentAlias : '/' + note.id;
    isFavorite = !!note.is_favorite;
    isPrivate = !!note.is_private;
    editorState.lastSavedBody = note.body;
    editorState.lastSavedAlias = currentAlias;
    editorState.lastSavedFavorite = isFavorite;
    editorState.lastSavedPrivate = isPrivate;
    if (editorState.bodyField && editorState.bodyField.value !== note.body) {
        editorState.bodyField.value = note.body;
    }
    if (editorState.aliasField) editorState.aliasField.value = currentAlias || '';
    if (editorState.publicToggle) editorState.publicToggle.checked = !isPrivate;
    if (editorState.favoriteToggle) editorState.favoriteToggle.checked = isFavorite;
    syncResourceChrome();
    restoreSelection(selection);
}

function syncResourceChrome() {
    var title = deriveTitle(currentBody());
    var visibility = isPrivate ? 'Private' : 'Public';
    updateLiveText('[data-live-title]', title, 'renderedTitle');
    updateLiveText('[data-live-visibility]', visibility, 'renderedVisibility');
    updateLiveText('[data-live-alias]', currentAlias || 'None', 'renderedAlias');
    syncCanonicalLinks();
    document.title = title + ' | ' + currentSiteName;
}

function syncCanonicalLinks() {
    var historyHref = currentHref + '/history';
    document.querySelectorAll('[data-current-resource-link]').forEach(function (node) { node.href = currentHref; });
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

function currentSelection() {
    if (!editorState.bodyField) return null;
    return {
        selectionStart: editorState.bodyField.selectionStart,
        selectionEnd: editorState.bodyField.selectionEnd
    };
}

function restoreSelection(selection) {
    if (!selection || !editorState.bodyField) return;
    editorState.bodyField.focus();
    editorState.bodyField.setSelectionRange(selection.selectionStart, selection.selectionEnd);
}

function deriveTitle(body) {
    var match = body.match(/^\s*#\s+(.+)$/m);
    return match && match[1] ? match[1].trim() : 'Untitled note';
}
