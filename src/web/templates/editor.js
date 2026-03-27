var saveTimer = null;
var editorInstance = null;
var sourceField = null;
var fallbackField = null;
var editorShell = null;
var previewToggle = null;
var previewBackdrop = null;
var compactEditor = window.matchMedia('(max-width: 900px)');

function initEditor() {
    sourceField = document.getElementById('editor-source');
    fallbackField = document.getElementById('editor-fallback');
    editorShell = document.getElementById('editor-shell');
    previewToggle = document.getElementById('preview-toggle');
    previewBackdrop = document.getElementById('preview-backdrop');
    var root = document.getElementById('editor-root');
    if (!sourceField || !root || !editorShell) return;
    bindPreviewEvents();
    if (window.toastui && window.toastui.Editor) {
        try {
            var options = {
                el: root,
                height: 'auto',
                minHeight: editorMinHeight(),
                initialValue: sourceField.value,
                initialEditType: 'markdown',
                previewStyle: 'vertical',
                hideModeSwitch: true,
                theme: 'dark',
                autofocus: false,
                usageStatistics: false
            };
            options.toolbarItems = toolbarItems();
            editorInstance = new window.toastui.Editor(options);
            window.editorInstance = editorInstance;
            setPreviewEnabled(true);
            syncPreviewState(false);
            editorInstance.on('change', onEditorInput);
            focusEditor();
            syncNoteChrome();
            return;
        } catch (_) {
            enableFallback(root);
        }
    } else {
        enableFallback(root);
    }
    syncNoteChrome();
}

function enableFallback(root) {
    editorInstance = null;
    window.editorInstance = null;
    setPreviewEnabled(false);
    if (root) root.hidden = true;
    if (!fallbackField) return;
    fallbackField.hidden = false;
    fallbackField.addEventListener('input', onEditorInput);
    requestAnimationFrame(function () { fallbackField.focus(); });
}

function onEditorInput() {
    syncNoteChrome();
    queueSave();
}

function currentBody() {
    if (editorInstance) {
        sourceField.value = editorInstance.getMarkdown();
        return sourceField.value;
    }
    if (fallbackField && !fallbackField.hidden) {
        sourceField.value = fallbackField.value;
    }
    return sourceField ? sourceField.value : '';
}

function queueSave() {
    clearTimeout(saveTimer);
    saveTimer = setTimeout(saveNote, 500);
}

function saveNote() {
    if (!sourceField || typeof currentId === 'undefined') return;
    fetch('/records/' + currentId, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ body: currentBody(), is_private: isPrivate })
    })
        .then(function (response) {
            if (!response.ok) throw new Error('save failed');
            setSaveError('');
        })
        .catch(function () {
            setSaveError('Save failed. Retry on the next change.');
        });
}

function setSaveError(message) {
    var node = document.getElementById('save-error');
    if (!node) return;
    node.textContent = message;
    node.hidden = !message;
}

function togglePublic() {
    var checkbox = document.getElementById('public-toggle');
    isPrivate = !checkbox.checked;
    syncNoteChrome();
    queueSave();
}

function syncNoteChrome() {
    var body = currentBody();
    var title = deriveTitle(body);
    var visibility = isPrivate ? 'Private' : 'Public';
    document.querySelectorAll('[data-live-title]').forEach(function (node) {
        node.textContent = title;
    });
    document.querySelectorAll('[data-live-visibility]').forEach(function (node) {
        node.textContent = visibility;
    });
    document.title = title + ' - kjxlkj';
}

function deriveTitle(body) {
    var match = body.match(/^\s*#\s+(.+)$/m);
    return match && match[1] ? match[1].trim() : 'Untitled note';
}

function bindPreviewEvents() {
    window.addEventListener('resize', syncPreviewBackdrop);
    document.addEventListener('keydown', handlePreviewEscape);
}

function togglePreview() {
    if (!editorShell || !editorInstance) return;
    syncPreviewState(!editorShell.classList.contains('preview-open'));
}

function closePreview() {
    syncPreviewState(false);
    if (previewToggle) previewToggle.focus();
}

function syncPreviewState(open) {
    if (!editorShell) return;
    editorShell.classList.toggle('preview-open', open);
    editorShell.classList.toggle('preview-closed', !open);
    if (previewToggle) {
        previewToggle.textContent = open ? 'Hide preview' : 'Show preview';
        previewToggle.setAttribute('aria-expanded', String(open));
    }
    syncPreviewBackdrop();
}

function syncPreviewBackdrop() {
    if (!previewBackdrop || !editorShell) return;
    previewBackdrop.hidden = !(editorShell.classList.contains('preview-open') && compactEditor.matches);
}

function setPreviewEnabled(enabled) {
    if (previewToggle) previewToggle.hidden = !enabled;
    if (!enabled) syncPreviewState(false);
}

function handlePreviewEscape(event) { if (event.key === 'Escape') closePreview(); }

function editorMinHeight() {
    return compactEditor.matches ? '360px' : '520px';
}

function toolbarItems() {
    if (compactEditor.matches) {
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
        if (editorInstance) {
            editorInstance.focus();
            if (typeof editorInstance.moveCursorToEnd === 'function') editorInstance.moveCursorToEnd();
            return;
        }
        if (fallbackField && !fallbackField.hidden) fallbackField.focus();
    });
}
