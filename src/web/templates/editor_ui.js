function bindPreviewEvents() {
    if (editorState.media.addEventListener) {
        editorState.media.addEventListener('change', syncPreviewMode);
    } else if (editorState.media.addListener) {
        editorState.media.addListener(syncPreviewMode);
    }
    document.addEventListener('keydown', handlePreviewEscape);
    window.addEventListener('resize', syncPreviewLayout);
}

function togglePreview() {
    editorState.previewOpen = !editorState.previewOpen;
    syncPreviewMode();
    if (editorState.previewOpen) queuePreviewRender(true);
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
    if (editorState.previewBackdrop) {
        editorState.previewBackdrop.hidden = !(editorState.previewOpen && compact);
    }
    if (!editorState.previewOpen) {
        clearTimeout(editorState.previewTimer);
    }
    syncPreviewLayout();
}

function handlePreviewEscape(event) {
    if (event.key === 'Escape' && editorState.previewOpen) closePreview();
}

function syncPreviewLayout() {
    if (!editorState.shell) return;
    var mobileBar = document.querySelector('.mobile-bar');
    var top = mobileBar ? Math.ceil(mobileBar.getBoundingClientRect().bottom + 16) : 92;
    editorState.shell.style.setProperty('--preview-top', top + 'px');
}

function queuePreviewRender(force) {
    if (!editorState.previewOpen || !editorState.previewRoot) return;
    var body = currentBody();
    if (!force && editorState.lastPreviewBody === body) return;
    clearTimeout(editorState.previewTimer);
    editorState.previewTimer = setTimeout(renderPreview, force ? 0 : 160);
}

async function renderPreview() {
    var body = currentBody();
    var requestId = ++editorState.latestPreview;
    try {
        var response = await fetch('/admin/markdown-preview', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ body: body })
        });
        if (!response.ok) throw new Error(await previewError(response));
        var payload = await response.json();
        if (requestId !== editorState.latestPreview || !editorState.previewRoot) return;
        editorState.lastPreviewBody = body;
        editorState.previewRoot.innerHTML = payload.html || '';
    } catch (_) {
        if (requestId !== editorState.latestPreview || !editorState.previewRoot) return;
        editorState.previewRoot.innerHTML = '<p class="surface-empty">Preview unavailable.</p>';
    }
}

async function previewError(response) {
    try {
        var payload = await response.json();
        return payload.message || 'Preview unavailable.';
    } catch {
        return 'Preview unavailable.';
    }
}

function focusEditor() {
    requestAnimationFrame(function () {
        if (!editorState.bodyField) return;
        editorState.bodyField.focus();
        editorState.bodyField.setSelectionRange(
            editorState.bodyField.value.length,
            editorState.bodyField.value.length
        );
    });
}
