var shortcutTimer = null;
var shortcutComposing = false;

function bindShortcutNormalization() {
    var surface = visibleWysiwygSurface();
    if (!surface) return;
    surface.addEventListener('compositionstart', onShortcutCompositionStart);
    surface.addEventListener('compositionend', onShortcutCompositionEnd);
    surface.addEventListener('keyup', onShortcutKeyup);
    surface.addEventListener('paste', queueShortcutNormalization);
}

function clearShortcutNormalization() {
    clearTimeout(shortcutTimer);
    shortcutComposing = false;
}

function onShortcutCompositionStart() {
    shortcutComposing = true;
}

function onShortcutCompositionEnd() {
    shortcutComposing = false;
    queueShortcutNormalization();
}

function onShortcutKeyup(event) {
    if (shortcutComposing || !shouldNormalizeShortcut(event)) return;
    queueShortcutNormalization();
}

function shouldNormalizeShortcut(event) {
    return event.key === ' ' || event.key === 'Enter' || event.key === '`';
}

function queueShortcutNormalization() {
    clearTimeout(shortcutTimer);
    shortcutTimer = setTimeout(normalizeShortcutBlock, 0);
}

function normalizeShortcutBlock() {
    if (shortcutComposing || !editorInstance) return;
    var shortcut = currentShortcutBlock();
    if (!shortcut) return;
    var markdown = editorInstance.getMarkdown();
    var selection = captureShortcutSelection();
    editorInstance.setMarkdown(markdown, false);
    restoreShortcutSelection(selection, shortcut);
}

function captureShortcutSelection() {
    try {
        var ww = editorInstance.getSelection();
        return editorInstance.convertPosToMatchEditorMode(ww[0], ww[1], 'markdown');
    } catch (_) {
        return null;
    }
}

function restoreShortcutSelection(selection, shortcut) {
    if (selection) {
        try {
            var ww = editorInstance.convertPosToMatchEditorMode(selection[0], selection[1], 'wysiwyg');
            editorInstance.focus();
            editorInstance.setSelection(ww[0], ww[1]);
            return;
        } catch (_) {}
    }
    var target = findRestoredShortcutBlock(shortcut);
    if (target) placeCaretAtEnd(target);
}

function currentShortcutBlock() {
    var surface = visibleWysiwygSurface();
    var selection = window.getSelection();
    if (!surface || !selection || !selection.anchorNode || !surface.contains(selection.anchorNode)) return null;
    var block = closestShortcutBlock(selection.anchorNode, surface);
    if (!block || !isParagraphLike(block)) return null;
    var text = (block.textContent || '').replace(/\u200b/g, '').trim();
    if (!text) return null;
    if (/^(#{1,6})\s+/.test(text)) return { kind: 'heading', text: text.replace(/^(#{1,6})\s+/, '').trim() };
    if (/^(?:[-+*]|\d+\.)\s+/.test(text)) return { kind: 'list', text: text.replace(/^(?:[-+*]|\d+\.)\s+/, '').trim() };
    if (/^>\s+/.test(text)) return { kind: 'quote', text: text.replace(/^>\s+/, '').trim() };
    if (/^```[\w-]*$/.test(text)) return { kind: 'code', text: '' };
    return null;
}

function closestShortcutBlock(node, surface) {
    var current = node && node.nodeType === Node.TEXT_NODE ? node.parentElement : node;
    while (current && current !== surface) {
        if (/^(P|DIV)$/.test(current.tagName)) return current;
        current = current.parentElement;
    }
    return null;
}

function isParagraphLike(node) {
    return !!node && /^(P|DIV)$/.test(node.tagName);
}

function findRestoredShortcutBlock(shortcut) {
    var selectors = { heading: 'h1,h2,h3,h4,h5,h6', list: 'li', quote: 'blockquote p,blockquote', code: 'pre code,pre' };
    var matches = Array.from(visibleWysiwygContents().querySelectorAll(selectors[shortcut.kind] || 'p'));
    if (!matches.length) return null;
    if (!shortcut.text) return matches[matches.length - 1];
    return matches.reverse().find(function (node) {
        return (node.textContent || '').trim() === shortcut.text;
    }) || matches[0];
}

function placeCaretAtEnd(node) {
    editorInstance.focus();
    var range = document.createRange();
    range.selectNodeContents(node);
    range.collapse(false);
    var selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
}

function visibleWysiwygSurface() {
    var ww = editorInstance && editorInstance.getEditorElements && editorInstance.getEditorElements().wwEditor;
    return ww ? ww.querySelector('.ProseMirror') : document.querySelector('.toastui-editor-ww-container .ProseMirror');
}

function visibleWysiwygContents() {
    var ww = editorInstance && editorInstance.getEditorElements && editorInstance.getEditorElements().wwEditor;
    return ww ? ww.querySelector('.toastui-editor-contents') : document.querySelector('.toastui-editor-ww-container .toastui-editor-contents');
}
