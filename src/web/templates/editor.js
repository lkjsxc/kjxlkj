var editorState = {
    editor: null,
    sourceField: null,
    fallbackField: null,
    aliasField: null,
    publicToggle: null,
    favoriteToggle: null,
    shell: null,
    previewToggle: null,
    previewBackdrop: null,
    media: window.matchMedia('(max-width: 900px)'),
    previewOpen: false,
    previewStyle: 'tab',
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
    var root = document.getElementById('editor-root');
    if (!editorState.sourceField || !root || !editorState.shell) return;
    bindEditorInputs();
    bindPreviewEvents();
    editorState.lastSavedBody = editorState.sourceField.value;
    editorState.lastSavedAlias = currentAlias;
    editorState.lastSavedFavorite = isFavorite;
    editorState.lastSavedPrivate = isPrivate;
    if (window.toastui && window.toastui.Editor) {
        try {
            createEditor(root);
        } catch (_) {
            enableFallback(root);
        }
    } else {
        enableFallback(root);
    }
    syncNoteChrome();
}

function cacheEditorNodes() {
    editorState.sourceField = document.getElementById('editor-source');
    editorState.fallbackField = document.getElementById('editor-fallback');
    editorState.aliasField = document.getElementById('alias-input');
    editorState.publicToggle = document.getElementById('public-toggle');
    editorState.favoriteToggle = document.getElementById('favorite-toggle');
    editorState.shell = document.getElementById('editor-shell');
    editorState.previewToggle = document.getElementById('preview-toggle');
    editorState.previewBackdrop = document.getElementById('preview-backdrop');
}

function bindEditorInputs() {
    editorState.aliasField?.addEventListener('input', onAliasInput);
    editorState.publicToggle?.addEventListener('change', onPublicToggle);
    editorState.favoriteToggle?.addEventListener('change', onFavoriteToggle);
}

function createEditor(root) {
    editorState.editor = new window.toastui.Editor({
        el: root,
        height: 'auto',
        minHeight: editorMinHeight(),
        initialValue: editorState.sourceField.value,
        initialEditType: 'markdown',
        previewStyle: 'tab',
        hideModeSwitch: true,
        theme: 'dark',
        autofocus: false,
        usageStatistics: false,
        toolbarItems: toolbarItems()
    });
    window.editorInstance = editorState.editor;
    editorState.editor.on('change', onEditorInput);
    setPreviewEnabled(true);
    syncPreviewMode();
    focusEditor();
}

function enableFallback(root) {
    editorState.editor = null;
    window.editorInstance = null;
    setPreviewEnabled(false);
    if (root) root.hidden = true;
    if (!editorState.fallbackField) return;
    editorState.fallbackField.hidden = false;
    editorState.fallbackField.addEventListener('input', onEditorInput);
    requestAnimationFrame(function () { editorState.fallbackField.focus(); });
}

function onEditorInput() {
    syncNoteChrome();
    queueSave();
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
    if (editorState.editor) {
        editorState.sourceField.value = editorState.editor.getMarkdown();
    } else if (editorState.fallbackField && !editorState.fallbackField.hidden) {
        editorState.sourceField.value = editorState.fallbackField.value;
    }
    return editorState.sourceField ? editorState.sourceField.value : '';
}

function normalizeAliasValue(value) {
    var cleaned = value
        .trim()
        .toLowerCase()
        .replace(/[_\s]+/g, '-')
        .replace(/[^a-z0-9-]/g, '')
        .replace(/--+/g, '-')
        .replace(/^-+|-+$/g, '');
    return cleaned || null;
}
