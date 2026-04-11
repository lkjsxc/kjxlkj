function queueSave(delay) {
    clearTimeout(editorState.saveTimer);
    if (editorState.composing || !isDirty(currentBody(), draftAliasValue(), isFavorite, isPrivate)) return;
    editorState.pendingSave = true;
    if (editorState.saveInFlight) return;
    editorState.saveTimer = setTimeout(function () {
        saveNote().catch(function () {});
    }, typeof delay === 'number' ? delay : 500);
}

function isDirty(body, alias, favorite, nextPrivate) {
    return body !== editorState.lastSavedBody ||
        alias !== editorState.lastSavedAlias ||
        favorite !== editorState.lastSavedFavorite ||
        nextPrivate !== editorState.lastSavedPrivate;
}

function saveNote() {
    if (!editorState.bodyField || typeof currentId === 'undefined') return Promise.resolve(null);
    if (editorState.saveInFlight) return editorState.savePromise || Promise.resolve(null);
    if (editorState.composing || !isDirty(currentBody(), draftAliasValue(), isFavorite, isPrivate)) {
        return Promise.resolve(null);
    }
    var request = draftSnapshot();
    editorState.pendingSave = false;
    editorState.saveInFlight = true;
    editorState.savePromise = fetch('/resources/' + currentId, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            body: request.body,
            alias: request.alias,
            is_favorite: request.isFavorite,
            is_private: request.isPrivate
        })
    })
        .then(readSaveResponse)
        .then(function (note) {
            applySavedResource(note, request);
            setSaveError('');
            return note;
        })
        .catch(function (error) {
            setSaveError(error.message || 'Save failed. Retry on the next change.');
            throw error;
        })
        .finally(function () {
            editorState.saveInFlight = false;
            editorState.savePromise = null;
            if (editorState.pendingSave || (!editorState.composing &&
                isDirty(currentBody(), draftAliasValue(), isFavorite, isPrivate))) {
                queueSave(0);
            }
        });
    return editorState.savePromise;
}

async function flushPendingSave() {
    clearTimeout(editorState.saveTimer);
    if (editorState.composing) return false;
    while (true) {
        if (editorState.saveInFlight) {
            try {
                await editorState.savePromise;
            } catch {
                return false;
            }
            continue;
        }
        if (editorState.composing) return false;
        if (!isDirty(currentBody(), draftAliasValue(), isFavorite, isPrivate)) return true;
        try {
            await saveNote();
        } catch {
            return false;
        }
    }
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

function applySavedResource(note, request, selection) {
    currentAlias = note.alias || null;
    currentHref = currentAlias ? '/' + currentAlias : '/' + note.id;
    editorState.lastSavedBody = note.body;
    editorState.lastSavedAlias = currentAlias;
    editorState.lastSavedFavorite = !!note.is_favorite;
    editorState.lastSavedPrivate = !!note.is_private;
    var bodyStale = currentBody() !== request.body;
    var aliasStale = draftAliasValue() !== request.alias;
    var favoriteStale = isFavorite !== request.isFavorite;
    var privateStale = isPrivate !== request.isPrivate;
    if (!bodyStale && !editorState.composing && editorState.bodyField && editorState.bodyField.value !== note.body) {
        editorState.bodyField.value = note.body;
    }
    if (!aliasStale && editorState.aliasField) editorState.aliasField.value = currentAlias || '';
    if (!favoriteStale) {
        isFavorite = !!note.is_favorite;
        if (editorState.favoriteToggle) editorState.favoriteToggle.checked = isFavorite;
    }
    if (!privateStale) {
        isPrivate = !!note.is_private;
        if (editorState.publicToggle) editorState.publicToggle.checked = !isPrivate;
    }
    syncResourceChrome();
    if (!bodyStale) restoreSelection(selection || request.selection);
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
    if (shouldReplaceCurrentUrl()) {
        window.kjxlkj?.replaceCurrentUrl?.(currentHref);
    }
}

function shouldReplaceCurrentUrl() {
    if (window.location.pathname === currentHref) return false;
    if (window.kjxlkj?.navigating || window.kjxlkj?.navigationUrl) return false;
    return true;
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
        selectionEnd: editorState.bodyField.selectionEnd,
        activeBody: document.activeElement === editorState.bodyField
    };
}

function restoreSelection(selection) {
    if (!selection || !selection.activeBody || !editorState.bodyField) return;
    editorState.bodyField.focus();
    editorState.bodyField.setSelectionRange(selection.selectionStart, selection.selectionEnd);
}

function deriveTitle(body) {
    var match = body.match(/^\s*#\s+(.+)$/m);
    return match && match[1] ? match[1].trim() : 'Untitled note';
}
