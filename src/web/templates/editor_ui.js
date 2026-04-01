function bindPreviewEvents() {
    editorState.media.addEventListener('change', syncPreviewMode);
    document.addEventListener('keydown', handlePreviewEscape);
}

function togglePreview() {
    if (!editorState.shell || !editorState.editor) return;
    editorState.previewOpen = !editorState.previewOpen;
    syncPreviewMode();
}

function closePreview() {
    if (!editorState.previewOpen) return;
    editorState.previewOpen = false;
    syncPreviewMode();
    if (editorState.previewToggle) editorState.previewToggle.focus();
}

function syncPreviewMode() {
    if (!editorState.shell) return;
    var compact = editorState.media.matches;
    var style = editorState.previewOpen ? 'vertical' : 'tab';
    if (editorState.editor && editorState.previewStyle !== style) {
        editorState.editor.changePreviewStyle(style);
        editorState.previewStyle = style;
    }
    if (style === 'tab' && editorState.editor && editorState.editor.eventEmitter) {
        editorState.editor.eventEmitter.emit('changePreviewTabWrite');
    }
    editorState.shell.classList.toggle('preview-open', editorState.previewOpen);
    editorState.shell.classList.toggle('preview-closed', !editorState.previewOpen);
    editorState.shell.classList.toggle('preview-compact', compact);
    if (editorState.previewToggle) {
        editorState.previewToggle.textContent = editorState.previewOpen ? 'Hide preview' : 'Show preview';
        editorState.previewToggle.setAttribute('aria-expanded', String(editorState.previewOpen));
    }
    if (editorState.previewBackdrop) {
        editorState.previewBackdrop.hidden = !(editorState.previewOpen && compact);
    }
}

function setPreviewEnabled(enabled) {
    if (editorState.previewToggle) editorState.previewToggle.hidden = !enabled;
    if (!enabled) {
        editorState.previewOpen = false;
        syncPreviewMode();
    }
}

function handlePreviewEscape(event) {
    if (event.key === 'Escape' && editorState.previewOpen) closePreview();
}

function editorMinHeight() {
    return editorState.media.matches ? '360px' : '520px';
}

function toolbarItems() {
    if (editorState.media.matches) {
        return [
            ['heading', 'bold', 'italic', 'strike'],
            ['quote', 'ul', 'ol', 'task'],
            ['table', 'link', 'code', 'codeblock']
        ];
    }
    return [
        ['heading'],
        ['bold', 'italic', 'strike', 'hr'],
        ['quote', 'ul', 'ol', 'task'],
        ['indent', 'outdent'],
        ['table', 'link'],
        ['code', 'codeblock']
    ];
}

function focusEditor() {
    requestAnimationFrame(function () {
        if (editorState.editor) {
            editorState.editor.focus();
            if (typeof editorState.editor.moveCursorToEnd === 'function') {
                editorState.editor.moveCursorToEnd();
            }
            return;
        }
        if (editorState.fallbackField && !editorState.fallbackField.hidden) {
            editorState.fallbackField.focus();
        }
    });
}
