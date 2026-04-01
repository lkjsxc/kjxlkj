var VIM_MODE_KEY = 'kjxlkj.vim-mode';

function configureVimMode() {
    editorState.vimPreference = readVimPreference();
    editorState.vimEnabled = resolveVimMode();
    editorState.vimNormal = editorState.vimEnabled;
    bindVimSurface();
    updateVimModeLabel();
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

function readVimPreference() {
    var stored = window.localStorage.getItem(VIM_MODE_KEY);
    return stored === 'on' || stored === 'off' ? stored : 'default';
}

function resolveVimMode() {
    var preference = readVimPreference();
    if (preference === 'on') return true;
    if (preference === 'off') return false;
    return !!defaultVimMode;
}

function bindVimSurface() {
    var surface = editorState.editor
        ? document.querySelector('.toastui-editor-md-container .ProseMirror')
        : editorState.fallbackField;
    if (!surface) return;
    surface.addEventListener('keydown', handleVimKeydown, true);
}

function handleVimKeydown(event) {
    if (!editorState.vimEnabled) return;
    if (!editorState.vimNormal) {
        if (event.key === 'Escape') {
            event.preventDefault();
            editorState.vimNormal = true;
            updateVimModeLabel();
        }
        return;
    }
    if (event.key === 'i') return enterInsertMode(event);
    if (event.key === 'a') return moveThenInsert(event, 'forward', 'character');
    if (event.key === 'h') return moveSelection(event, 'backward', 'character');
    if (event.key === 'l') return moveSelection(event, 'forward', 'character');
    if (event.key === 'j') return moveSelection(event, 'forward', 'line');
    if (event.key === 'k') return moveSelection(event, 'backward', 'line');
    if (event.key === '0') return moveSelection(event, 'backward', 'lineboundary');
    if (event.key === '$') return moveSelection(event, 'forward', 'lineboundary');
    if (event.key === 'x') return deleteSelection(event);
    if (event.key === 'o') return openLineBelow(event);
    if (event.key === 'Escape') return event.preventDefault();
    if (event.key.length === 1) event.preventDefault();
}

function enterInsertMode(event) {
    event.preventDefault();
    editorState.vimNormal = false;
    updateVimModeLabel();
}

function moveThenInsert(event, direction, granularity) {
    moveSelection(event, direction, granularity);
    editorState.vimNormal = false;
    updateVimModeLabel();
}

function moveSelection(event, direction, granularity) {
    event.preventDefault();
    var selection = window.getSelection();
    if (selection && typeof selection.modify === 'function') {
        selection.modify('move', direction, granularity);
    }
}

function deleteSelection(event) {
    event.preventDefault();
    document.execCommand('delete');
}

function openLineBelow(event) {
    event.preventDefault();
    document.execCommand('insertLineBreak');
    editorState.vimNormal = false;
    updateVimModeLabel();
}

function updateVimModeLabel() {
    var text = !editorState.vimEnabled
        ? 'Vim off'
        : editorState.vimNormal
          ? 'Vim normal'
          : 'Vim insert';
    document.querySelectorAll('[data-vim-mode-state]').forEach(function (node) {
        node.textContent = text;
    });
}
