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
    latestPreview: 0,
    lastSavedBody: '',
    lastSavedAlias: null,
    lastSavedFavorite: false,
    lastSavedPrivate: null,
    saveInFlight: false,
    savePromise: null,
    pendingSave: false,
    composing: false,
    uploading: false,
    lastPreviewBody: null,
    renderedTitle: '',
    renderedVisibility: '',
    renderedAlias: '',
    uploadSelection: null
};

function initEditor() {
    disposeEditor();
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
    registerEditorCleanup();
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
    editorState.bodyField.addEventListener('compositionstart', onCompositionStart);
    editorState.bodyField.addEventListener('compositionend', onCompositionEnd);
    editorState.aliasField.addEventListener('input', onAliasInput);
    editorState.publicToggle.addEventListener('change', onPublicToggle);
    editorState.favoriteToggle.addEventListener('change', onFavoriteToggle);
}

function onEditorInput() {
    syncResourceChrome();
    if (editorState.composing) return;
    queueSave();
    queuePreviewRender(false);
}

function onCompositionStart() {
    editorState.composing = true;
}

function onCompositionEnd() {
    editorState.composing = false;
    queueSave();
    queuePreviewRender(true);
}

function onAliasInput() {
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

function draftSnapshot() {
    return {
        body: currentBody(),
        alias: draftAliasValue(),
        isFavorite: isFavorite,
        isPrivate: isPrivate,
        selection: currentSelection()
    };
}

function registerEditorCleanup() {
    editorState.dispose = function () {
        clearTimeout(editorState.saveTimer);
        clearTimeout(editorState.previewTimer);
        if (editorState.bodyField) {
            editorState.bodyField.removeEventListener('input', onEditorInput);
            editorState.bodyField.removeEventListener('compositionstart', onCompositionStart);
            editorState.bodyField.removeEventListener('compositionend', onCompositionEnd);
        }
        if (editorState.aliasField) editorState.aliasField.removeEventListener('input', onAliasInput);
        if (editorState.publicToggle) editorState.publicToggle.removeEventListener('change', onPublicToggle);
        if (editorState.favoriteToggle) editorState.favoriteToggle.removeEventListener('change', onFavoriteToggle);
        if (typeof unbindPreviewEvents === 'function') unbindPreviewEvents();
        if (window.kjxlkj) delete window.kjxlkj.beforeNavigate;
        editorState.dispose = null;
    };
    if (window.kjxlkj?.registerCleanup) window.kjxlkj.registerCleanup(editorState.dispose);
    if (window.kjxlkj) window.kjxlkj.beforeNavigate = beforeEditorNavigate;
}

function disposeEditor() {
    if (typeof editorState.dispose === 'function') editorState.dispose();
}

async function beforeEditorNavigate() {
    if (editorState.uploading) {
        setUploadStatus('Wait for the media upload to finish.', 'error');
        return false;
    }
    return flushPendingSave();
}
