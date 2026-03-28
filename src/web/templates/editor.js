var editorState = {
    editor: null,
    sourceField: null,
    fallbackField: null,
    shell: null,
    previewToggle: null,
    previewBackdrop: null,
    media: window.matchMedia('(max-width: 900px)'),
    previewOpen: false,
    previewStyle: 'tab',
    saveTimer: null,
    latestRequest: 0,
    lastSavedBody: '',
    lastSavedPrivate: null,
    renderedTitle: '',
    renderedVisibility: ''
};

function initEditor() {
    cacheEditorNodes();
    var root = document.getElementById('editor-root');
    if (!editorState.sourceField || !root || !editorState.shell) return;
    bindPreviewEvents();
    editorState.lastSavedBody = editorState.sourceField.value;
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
    editorState.shell = document.getElementById('editor-shell');
    editorState.previewToggle = document.getElementById('preview-toggle');
    editorState.previewBackdrop = document.getElementById('preview-backdrop');
}

function bindPreviewEvents() {
    editorState.media.addEventListener('change', syncPreviewMode);
    document.addEventListener('keydown', handlePreviewEscape);
}

function createEditor(root) {
    var options = {
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
    };
    editorState.editor = new window.toastui.Editor(options);
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

function currentBody() {
    if (editorState.editor) {
        editorState.sourceField.value = editorState.editor.getMarkdown();
    } else if (editorState.fallbackField && !editorState.fallbackField.hidden) {
        editorState.sourceField.value = editorState.fallbackField.value;
    }
    return editorState.sourceField ? editorState.sourceField.value : '';
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
    var style = editorState.previewOpen && !compact ? 'vertical' : 'tab';
    if (editorState.editor && editorState.previewStyle !== style) {
        editorState.editor.changePreviewStyle(style);
        editorState.previewStyle = style;
    }
    if (style === 'tab' && editorState.editor && editorState.editor.eventEmitter) {
        editorState.editor.eventEmitter.emit(
            editorState.previewOpen ? 'changePreviewTabPreview' : 'changePreviewTabWrite'
        );
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
    if (event.key === 'Escape') closePreview();
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
