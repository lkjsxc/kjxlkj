var editorState = {
    bodyField: null,
    aliasField: null,
    publicToggle: null,
    favoriteToggle: null,
    uploadButton: null,
    uploadInput: null,
    shell: null,
    previewToggle: null,
    previewBackdrop: null,
    previewPanel: null,
    previewRoot: null,
    media: window.matchMedia('(max-width: 900px)'),
    previewOpen: false,
    saveTimer: null,
    previewTimer: null,
    latestRequest: 0,
    latestPreview: 0,
    lastSavedBody: '',
    lastSavedAlias: null,
    lastSavedFavorite: false,
    lastSavedPrivate: null,
    uploading: false,
    lastPreviewBody: null,
    renderedTitle: '',
    renderedVisibility: '',
    renderedAlias: ''
};

function initEditor() {
    cacheEditorNodes();
    if (!editorState.bodyField || !editorState.shell) return;
    bindEditorInputs();
    bindPreviewEvents();
    if (typeof bindUploadEvents === 'function') bindUploadEvents();
    editorState.lastSavedBody = editorState.bodyField.value;
    editorState.lastSavedAlias = currentAlias;
    editorState.lastSavedFavorite = isFavorite;
    editorState.lastSavedPrivate = isPrivate;
    syncResourceChrome();
    syncPreviewMode();
    focusEditor();
}

function cacheEditorNodes() {
    editorState.bodyField = document.getElementById('editor-body');
    editorState.aliasField = document.getElementById('alias-input');
    editorState.publicToggle = document.getElementById('public-toggle');
    editorState.favoriteToggle = document.getElementById('favorite-toggle');
    editorState.uploadButton = document.getElementById('upload-media-trigger');
    editorState.uploadInput = document.getElementById('upload-media-input');
    editorState.shell = document.getElementById('editor-shell');
    editorState.previewToggle = document.getElementById('preview-toggle');
    editorState.previewBackdrop = document.getElementById('preview-backdrop');
    editorState.previewPanel = document.getElementById('editor-preview-panel');
    editorState.previewRoot = document.getElementById('editor-preview');
}

function bindEditorInputs() {
    editorState.bodyField.addEventListener('input', onEditorInput);
    editorState.aliasField.addEventListener('input', onAliasInput);
    editorState.publicToggle.addEventListener('change', onPublicToggle);
    editorState.favoriteToggle.addEventListener('change', onFavoriteToggle);
}

function onEditorInput() {
    syncResourceChrome();
    queueSave();
    queuePreviewRender(false);
}

function onAliasInput() {
    setSaveError('');
    queueSave();
}

function onPublicToggle() {
    isPrivate = !editorState.publicToggle.checked;
    syncResourceChrome();
    queueSave();
}

function onFavoriteToggle() {
    isFavorite = !!editorState.favoriteToggle.checked;
    syncResourceChrome();
    queueSave();
}

function currentBody() {
    return editorState.bodyField ? editorState.bodyField.value : '';
}

function draftAliasValue() {
    if (!editorState.aliasField) return null;
    var value = editorState.aliasField.value.trim();
    return value ? value : null;
}
