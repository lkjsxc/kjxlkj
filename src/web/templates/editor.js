var editorState = {
    sourceField: null,
    aliasField: null,
    publicToggle: null,
    favoriteToggle: null,
    shell: null,
    previewToggle: null,
    previewBackdrop: null,
    previewPanel: null,
    previewState: null,
    media: window.matchMedia('(max-width: 900px)'),
    previewOpen: false,
    previewTimer: null,
    previewRequest: 0,
    saveTimer: null,
    latestRequest: 0,
    lastSavedBody: '',
    lastSavedAlias: null,
    lastSavedFavorite: false,
    lastSavedPrivate: null,
    renderedTitle: '',
    renderedVisibility: '',
    renderedAlias: ''
};

function initEditor() {
    cacheEditorNodes();
    if (!editorState.sourceField || !editorState.shell) return;
    bindEditorInputs();
    bindPreviewEvents();
    editorState.lastSavedBody = editorState.sourceField.value;
    editorState.lastSavedAlias = currentAlias;
    editorState.lastSavedFavorite = isFavorite;
    editorState.lastSavedPrivate = isPrivate;
    resizeEditor();
    syncNoteChrome();
    syncPreviewMode();
    focusEditor();
}

function cacheEditorNodes() {
    editorState.sourceField = document.getElementById('editor-source');
    editorState.aliasField = document.getElementById('alias-input');
    editorState.publicToggle = document.getElementById('public-toggle');
    editorState.favoriteToggle = document.getElementById('favorite-toggle');
    editorState.shell = document.getElementById('editor-shell');
    editorState.previewToggle = document.getElementById('preview-toggle');
    editorState.previewBackdrop = document.getElementById('preview-backdrop');
    editorState.previewPanel = document.getElementById('editor-preview');
    editorState.previewState = document.getElementById('preview-state');
}

function bindEditorInputs() {
    editorState.sourceField?.addEventListener('input', onEditorInput);
    editorState.aliasField?.addEventListener('input', onAliasInput);
    editorState.publicToggle?.addEventListener('change', onPublicToggle);
    editorState.favoriteToggle?.addEventListener('change', onFavoriteToggle);
    window.addEventListener('resize', resizeEditor);
}

function onEditorInput() {
    resizeEditor();
    syncNoteChrome();
    queueSave();
    queuePreviewRender();
}

function onAliasInput() {
    currentAlias = normalizeAliasValue(editorState.aliasField?.value || '');
    if (editorState.aliasField) editorState.aliasField.value = currentAlias || '';
    syncNoteChrome();
    queueSave();
}

function onPublicToggle() {
    isPrivate = !(editorState.publicToggle && editorState.publicToggle.checked);
    syncNoteChrome();
    queueSave();
}

function onFavoriteToggle() {
    isFavorite = !!(editorState.favoriteToggle && editorState.favoriteToggle.checked);
    syncNoteChrome();
    queueSave();
}

function currentBody() {
    return editorState.sourceField ? editorState.sourceField.value : '';
}

function normalizeAliasValue(value) {
    var cleaned = value.trim().toLowerCase().replace(/[\s]+/g, '-').replace(/[^a-z0-9._-]/g, '')
        .replace(/[._-]{2,}/g, '-').replace(/^[._-]+|[._-]+$/g, '');
    return cleaned || null;
}

function resizeEditor() {
    if (!editorState.sourceField) return;
    editorState.sourceField.style.height = 'auto';
    editorState.sourceField.style.height = editorState.sourceField.scrollHeight + 'px';
}

function focusEditor() {
    requestAnimationFrame(function () {
        editorState.sourceField?.focus();
    });
}
