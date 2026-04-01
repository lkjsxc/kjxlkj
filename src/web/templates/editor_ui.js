function bindPreviewEvents() {
    editorState.media.addEventListener('change', syncPreviewMode);
    document.addEventListener('keydown', handlePreviewEscape);
}

function togglePreview() {
    if (!editorState.shell || !editorState.previewPanel) return;
    editorState.previewOpen = !editorState.previewOpen;
    syncPreviewMode();
    if (editorState.previewOpen) renderPreview();
}

function closePreview() {
    if (!editorState.previewOpen) return;
    editorState.previewOpen = false;
    syncPreviewMode();
    if (editorState.previewToggle) editorState.previewToggle.focus();
}

function syncPreviewMode() {
    if (!editorState.shell || !editorState.previewPanel) return;
    var compact = editorState.media.matches;
    editorState.shell.classList.toggle('preview-open', editorState.previewOpen);
    editorState.shell.classList.toggle('preview-closed', !editorState.previewOpen);
    editorState.shell.classList.toggle('preview-compact', compact);
    editorState.previewPanel.hidden = !editorState.previewOpen;
    if (editorState.previewToggle) {
        editorState.previewToggle.textContent = editorState.previewOpen ? 'Hide preview' : 'Show preview';
        editorState.previewToggle.setAttribute('aria-expanded', String(editorState.previewOpen));
    }
    if (editorState.previewState) editorState.previewState.textContent = editorState.previewOpen ? 'Open' : 'Closed';
    if (editorState.previewBackdrop) editorState.previewBackdrop.hidden = !(editorState.previewOpen && compact);
}

function queuePreviewRender() {
    clearTimeout(editorState.previewTimer);
    if (!editorState.previewOpen) return;
    editorState.previewTimer = setTimeout(renderPreview, 180);
}

function renderPreview() {
    if (!editorState.previewOpen || !editorState.previewPanel) return;
    var requestId = ++editorState.previewRequest;
    if (editorState.previewState) editorState.previewState.textContent = 'Loading';
    fetch('/preview', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ body: currentBody() })
    })
        .then(function (response) {
            if (!response.ok) throw new Error('Preview unavailable.');
            return response.json();
        })
        .then(function (payload) {
            if (requestId !== editorState.previewRequest || !editorState.previewPanel) return;
            editorState.previewPanel.innerHTML = payload.html;
            if (editorState.previewState) editorState.previewState.textContent = 'Live';
        })
        .catch(function () {
            if (requestId !== editorState.previewRequest || !editorState.previewPanel) return;
            editorState.previewPanel.innerHTML = '<p class="surface-empty">Preview unavailable. Continue editing and try again.</p>';
            if (editorState.previewState) editorState.previewState.textContent = 'Unavailable';
        });
}

function handlePreviewEscape(event) {
    if (event.key === 'Escape' && editorState.previewOpen) closePreview();
}
