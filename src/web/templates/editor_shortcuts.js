var shortcutTimer = null;
var shortcutComposing = false;
var shortcutKey = '';
var shortcutSnapshot = null;
var shortcutSurface = null;
function bindShortcutNormalization() {
    var surface = visibleWysiwygSurface();
    if (!surface) {
        setTimeout(bindShortcutNormalization, 50);
        return;
    }
    if (surface === shortcutSurface) return;
    shortcutSurface = surface;
    surface.addEventListener('compositionstart', onShortcutCompositionStart);
    surface.addEventListener('compositionend', onShortcutCompositionEnd);
    surface.addEventListener('keydown', onShortcutKeydown);
    surface.addEventListener('keyup', onShortcutKeyup);
    surface.addEventListener('paste', queueShortcutNormalization);
}
function clearShortcutNormalization() {
    clearTimeout(shortcutTimer);
    shortcutComposing = false;
    shortcutKey = '';
    shortcutSnapshot = null;
    shortcutSurface = null;
}
function onShortcutCompositionStart() {
    shortcutComposing = true;
}
function onShortcutCompositionEnd() {
    shortcutComposing = false;
    shortcutKey = 'composition';
    queueShortcutNormalization();
}
function onShortcutKeydown(event) {
    if (shortcutComposing || event.key !== 'Enter') return;
    shortcutSnapshot = captureShortcutState();
}
function onShortcutKeyup(event) {
    if (shortcutComposing || !shouldNormalizeShortcut(event)) return;
    shortcutKey = event.key;
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
    var state = shortcutKey === 'Enter' && shortcutSnapshot ? shortcutSnapshot : captureShortcutState();
    shortcutKey = '';
    shortcutSnapshot = null;
    var markdown = editorInstance.getMarkdown();
    var rewritten = rewriteShortcutMarkdown(markdown);
    if (!rewritten || rewritten === markdown) return;
    editorInstance.setMarkdown(rewritten, false);
    if (state) restoreShortcutSelection(state.selection, state.shortcut, rewritten);
}
function captureShortcutState() {
    var shortcut = currentShortcutBlock();
    if (!shortcut) return null;
    return { shortcut: shortcut, selection: captureShortcutSelection() };
}
function captureShortcutSelection() {
    try {
        var ww = editorInstance.getSelection();
        return editorInstance.convertPosToMatchEditorMode(ww[0], ww[1], 'markdown');
    } catch (_) {
        return null;
    }
}
function rewriteShortcutMarkdown(markdown) {
    var changed = false;
    var lines = markdown.split('\n').map(function (line) {
        var rewritten = normalizeShortcutLine(line);
        changed = changed || rewritten !== line;
        return rewritten;
    });
    return changed ? lines.join('\n') : null;
}
function normalizeShortcutLine(line) {
    if (/^(?:\\#){1,6}(?:\s.*)?$/.test(line)) return line.replace(/\\#/g, '#');
    if (/^\\[-+*](?:\s.*)?$/.test(line)) return line.replace(/^\\([-+*])/, '$1');
    if (/^\d+\\\.(?:\s.*)?$/.test(line)) return line.replace(/^(\d+)\\\./, '$1.');
    if (/^\\>(?:\s.*)?$/.test(line)) return line.replace(/^\\>/, '>');
    if (/^(?:\\`){3}[\w-]*$/.test(line)) return line.replace(/\\`/g, '`');
    return line;
}
function restoreShortcutSelection(selection, shortcut, markdown) {
    if (selection) {
        try {
            var ww = editorInstance.convertPosToMatchEditorMode(clampMarkdownPos(selection[0], markdown), clampMarkdownPos(selection[1], markdown), 'wysiwyg');
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
    var text = (block.textContent || '').replace(/\u200b/g, '').replace(/\u00a0/g, ' ');
    if (!text.trim()) return null;
    var heading = text.match(/^(#{1,6})\s+(.*)$/);
    if (heading) return buildShortcut('heading', heading[1], heading[2] || '');
    var list = text.match(/^((?:[-+*]|\d+\.))\s+(.*)$/);
    if (list) return buildShortcut('list', list[1], list[2] || '');
    var quote = text.match(/^(>)\s+(.*)$/);
    if (quote) return buildShortcut('quote', quote[1], quote[2] || '');
    var code = text.match(/^```([\w-]*)$/);
    if (code) return buildShortcut('code', '```', code[1] || '');
    return null;
}
function buildShortcut(kind, marker, text) {
    var content = text.replace(/\s+$/, '');
    return { kind: kind, marker: marker, text: content, markdown: kind === 'code' ? marker + content : marker + ' ' + content, escaped: escapedShortcutLine(kind, marker, content) };
}
function escapedShortcutLine(kind, marker, text) {
    if (kind === 'heading') return marker.replace(/#/g, '\\#') + ' ' + text;
    if (kind === 'quote') return '\\> ' + text;
    if (kind === 'code') return '\\`\\`\\`' + text;
    if (/^\d+\.$/.test(marker)) return marker.replace('.', '\\.') + ' ' + text;
    return '\\' + marker + ' ' + text;
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
    var root = visibleWysiwygContents();
    if (!root) return null;
    var matches = Array.from(root.querySelectorAll(selectors[shortcut.kind] || 'p'));
    if (!matches.length) return null;
    if (!shortcut.text) return matches[matches.length - 1];
    return matches.reverse().find(function (node) {
        return (node.textContent || '').trim() === shortcut.text;
    }) || matches[0];
}
function clampMarkdownPos(pos, markdown) {
    var lines = markdown.split('\n');
    var line = Math.min(Math.max(pos[0], 1), lines.length);
    var column = Math.min(Math.max(pos[1], 1), lines[line - 1].length + 1);
    return [line, column];
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
