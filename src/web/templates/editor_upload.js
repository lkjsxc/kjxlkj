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
    editorState.uploading = true;
    if (editorState.uploadButton) editorState.uploadButton.disabled = true;
    setUploadStatus(
        files.length === 1 ? 'Uploading 1 media item...' : 'Uploading ' + files.length + ' media items...',
        ''
    );
    var selection = editorState.uploadSelection || currentSelectionRange();
    editorState.uploadSelection = null;
    var body = currentBody();
    var formData = new FormData();
    for (const file of files) formData.append('file', file);
    formData.append('body', body);
    formData.append('is_favorite', isFavorite ? 'true' : 'false');
    formData.append('is_private', isPrivate ? 'true' : 'false');
    formData.append('insert_start', String(utf8Offset(body, selection.start)));
    formData.append('insert_end', String(utf8Offset(body, selection.end)));
    var alias = draftAliasValue();
    if (alias) formData.append('alias', alias);
    var requestId = ++editorState.latestRequest;
    try {
        var response = await fetch('/resources/' + currentId + '/media-attachments', {
            method: 'POST',
            body: formData
        });
        var payload = await response.json();
        if (!response.ok) throw new Error(payload.message || 'Media upload failed.');
        if (requestId !== editorState.latestRequest) return;
        var cursor = payload.selection_fallback
            ? body.length + payload.inserted_markdown.length
            : selection.start + payload.inserted_markdown.length;
        applySavedResource(payload.current_resource, { selectionStart: cursor, selectionEnd: cursor });
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
        if (requestId === editorState.latestRequest) {
            setUploadStatus(error.message || 'Media upload failed.', 'error');
        }
    } finally {
        editorState.uploading = false;
        if (editorState.uploadButton) editorState.uploadButton.disabled = false;
    }
}

function currentSelectionRange() {
    var selection = currentSelection();
    return {
        start: selection?.selectionStart ?? 0,
        end: selection?.selectionEnd ?? 0
    };
}

function utf8Offset(value, codeUnitIndex) {
    return new TextEncoder().encode(value.slice(0, codeUnitIndex)).length;
}

function setUploadStatus(message, tone) {
    var node = document.getElementById('upload-media-status');
    if (!node) return;
    node.textContent = message;
    node.dataset.tone = tone || '';
    node.hidden = !message;
}
