function bindUploadEvents() {
    if (!editorState.uploadButton || !editorState.uploadInput || !editorState.bodyField) return;
    editorState.uploadButton.addEventListener('click', function () {
        if (editorState.uploading) return;
        editorState.uploadSelection = currentSelectionRange();
        editorState.uploadInput.click();
    });
    editorState.uploadInput.addEventListener('change', function () {
        if (!editorState.uploadInput?.files?.length || editorState.uploading) {
            editorState.uploadSelection = null;
            if (editorState.uploadInput) editorState.uploadInput.value = '';
            return;
        }
        uploadSelectedMedia(Array.from(editorState.uploadInput.files));
        editorState.uploadInput.value = '';
    });
}

async function uploadSelectedMedia(files) {
    if (!files.length || typeof currentId === 'undefined') return;
    clearTimeout(editorState.saveTimer);
    if (!await flushPendingSave()) {
        setUploadStatus(
            editorState.composing
                ? 'Finish the current text composition before uploading media.'
                : 'Save failed. Retry on the next change.',
            'error'
        );
        return;
    }
    editorState.uploading = true;
    setUploadBusy(true);
    setUploadStatus(
        files.length === 1 ? 'Uploading 1 media item...' : 'Uploading ' + files.length + ' media items...',
        ''
    );
    var selection = editorState.uploadSelection || currentSelectionRange();
    editorState.uploadSelection = null;
    var request = draftSnapshot();
    var body = request.body;
    var formData = new FormData();
    for (const file of files) formData.append('file', file);
    formData.append('body', body);
    formData.append('is_favorite', request.isFavorite ? 'true' : 'false');
    formData.append('is_private', request.isPrivate ? 'true' : 'false');
    formData.append('insert_start', String(utf8Offset(body, selection.start)));
    formData.append('insert_end', String(utf8Offset(body, selection.end)));
    if (request.alias) formData.append('alias', request.alias);
    try {
        var response = await fetch('/resources/' + currentId + '/media-attachments', {
            method: 'POST',
            body: formData
        });
        var payload = await readUploadResponse(response);
        if (!response.ok) throw new Error(payload.message || 'Media upload failed.');
        var cursor = codeUnitIndexFromUtf8(
            payload.current_resource.body,
            typeof payload.cursor_utf8 === 'number'
                ? payload.cursor_utf8
                : utf8Offset(payload.current_resource.body, payload.current_resource.body.length)
        );
        applySavedResource(payload.current_resource, request, {
            selectionStart: cursor,
            selectionEnd: cursor,
            activeBody: true
        });
        setSaveError('');
        queuePreviewRender(true);
        setUploadStatus(
            payload.selection_fallback
                ? 'Selection changed; inserted at end.'
                : files.length === 1
                    ? 'Uploaded 1 media item.'
                    : 'Uploaded ' + files.length + ' media items.',
            ''
        );
    } catch (error) {
        setUploadStatus(error.message || 'Media upload failed.', 'error');
    } finally {
        editorState.uploading = false;
        setUploadBusy(false);
    }
}

async function readUploadResponse(response) {
    var text = await response.text();
    if (!text) return {};
    try {
        return JSON.parse(text);
    } catch (_) {
        return { message: plainErrorText(text) || 'Media upload failed.' };
    }
}

function plainErrorText(text) {
    return text
        .replace(/<script[\s\S]*?<\/script>/gi, ' ')
        .replace(/<style[\s\S]*?<\/style>/gi, ' ')
        .replace(/<[^>]*>/g, ' ')
        .replace(/\s+/g, ' ')
        .trim();
}

function currentSelectionRange() {
    var selection = currentSelection();
    return {
        start: selection?.selectionStart ?? 0,
        end: selection?.selectionEnd ?? 0,
        activeBody: selection?.activeBody ?? false
    };
}

function utf8Offset(value, codeUnitIndex) {
    return new TextEncoder().encode(value.slice(0, codeUnitIndex)).length;
}

function codeUnitIndexFromUtf8(value, utf8Index) {
    var total = 0;
    for (var index = 0; index < value.length; index += 1) {
        var codePoint = value.codePointAt(index);
        var segment = String.fromCodePoint(codePoint);
        var bytes = new TextEncoder().encode(segment).length;
        if (total + bytes > utf8Index) return index;
        total += bytes;
        if (codePoint > 0xFFFF) index += 1;
    }
    return value.length;
}

function setUploadStatus(message, tone) {
    var node = document.getElementById('upload-media-status');
    if (!node) return;
    node.textContent = message;
    node.dataset.tone = tone || '';
    node.hidden = !message;
}

function setUploadBusy(busy) {
    if (editorState.uploadButton) editorState.uploadButton.disabled = busy;
    if (editorState.bodyField) editorState.bodyField.readOnly = busy;
    if (editorState.aliasField) editorState.aliasField.readOnly = busy;
    if (editorState.publicToggle) editorState.publicToggle.disabled = busy;
    if (editorState.favoriteToggle) editorState.favoriteToggle.disabled = busy;
}
