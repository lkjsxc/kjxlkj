function escapeHtml(text) {
    return text
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
}

function textHtml(lines) {
    return escapeHtml(lines.join('\n')).replace(/\n/g, '<br>');
}

function heading(line) {
    var match = line.match(/^(#{1,3})\s+(.*)$/);
    return match ? { level: match[1].length, text: match[2] } : null;
}

function quote(line) {
    return line.indexOf('> ') === 0 ? line.slice(2) : (line[0] === '>' ? line.slice(1) : null);
}

function bullet(line) {
    return line.indexOf('- ') === 0 || line.indexOf('* ') === 0 ? line.slice(2) : null;
}

function ordered(line) {
    var match = line.match(/^\d+\.\s+(.*)$/);
    return match ? match[1] : null;
}

function unsupported(line, next) {
    var trimmed = line.replace(/^\s+/, '');
    return line.indexOf('    ') === 0 ||
        line[0] === '\t' ||
        trimmed.indexOf('####') === 0 ||
        trimmed.indexOf('---') === 0 ||
        trimmed.indexOf('***') === 0 ||
        trimmed.indexOf('~~~') === 0 ||
        trimmed.indexOf('<') === 0 ||
        trimmed.indexOf('- [') === 0 ||
        trimmed.indexOf('* [') === 0 ||
        trimmed.indexOf('[^') === 0 ||
        (trimmed.indexOf('|') !== -1 && next && /^[|:\-\s]+$/.test(next));
}

function inlineMarkup(text) {
    return text.indexOf('**') !== -1 ||
        text.indexOf('__') !== -1 ||
        text.indexOf('`') !== -1 ||
        text.indexOf('![') !== -1 ||
        text.indexOf('](') !== -1 ||
        text.indexOf('~~') !== -1;
}

function special(line) {
    return !!heading(line) || line.indexOf('```') === 0 || quote(line) !== null || bullet(line) !== null || ordered(line) !== null;
}

function parseBlocks(body) {
    var lines = body.split(/\r?\n/);
    var blocks = [];
    for (var i = 0; i < lines.length;) {
        var line = lines[i];
        if (!line.trim()) { i += 1; continue; }
        if (unsupported(line, lines[i + 1])) return null;
        var title = heading(line);
        if (title) {
            if (inlineMarkup(title.text)) return null;
            blocks.push({ kind: 'heading', level: title.level, lines: [title.text] });
            i += 1;
            continue;
        }
        if (line.indexOf('```') === 0) {
            var lang = line.slice(3).trim();
            var code = [];
            for (i += 1; i < lines.length && lines[i].indexOf('```') !== 0; i += 1) code.push(lines[i]);
            if (i === lines.length) return null;
            blocks.push({ kind: 'code', lang: lang, lines: code });
            i += 1;
            continue;
        }
        if (quote(line) !== null) {
            var quoted = [];
            while (i < lines.length && quote(lines[i]) !== null) {
                if (inlineMarkup(quote(lines[i]))) return null;
                quoted.push(quote(lines[i]));
                i += 1;
            }
            blocks.push({ kind: 'quote', lines: quoted });
            continue;
        }
        if (bullet(line) !== null || ordered(line) !== null) {
            var listKind = ordered(line) !== null ? 'ordered' : 'bullet';
            var items = [];
            while (i < lines.length) {
                var item = listKind === 'ordered' ? ordered(lines[i]) : bullet(lines[i]);
                if (item === null) break;
                if (inlineMarkup(item)) return null;
                items.push(item);
                i += 1;
            }
            blocks.push({ kind: listKind, lines: items });
            continue;
        }
        var paragraph = [];
        while (i < lines.length && lines[i].trim() && !special(lines[i])) {
            if (unsupported(lines[i], lines[i + 1]) || inlineMarkup(lines[i])) return null;
            paragraph.push(lines[i]);
            i += 1;
        }
        blocks.push({ kind: 'paragraph', lines: paragraph });
    }
    return blocks.length ? blocks : [{ kind: 'paragraph', lines: [''] }];
}

function blockHtml(block) {
    if (block.kind === 'heading') return '<article class="rich-block"><h' + block.level + ' class="block-editable" contenteditable="true" spellcheck="true" data-kind="heading" data-level="' + block.level + '">' + textHtml(block.lines) + '</h' + block.level + '></article>';
    if (block.kind === 'paragraph') return '<article class="rich-block"><p class="block-editable" contenteditable="true" spellcheck="true" data-kind="paragraph">' + textHtml(block.lines) + '</p></article>';
    if (block.kind === 'quote') return '<article class="rich-block"><blockquote class="block-editable" contenteditable="true" spellcheck="true" data-kind="quote">' + textHtml(block.lines) + '</blockquote></article>';
    if (block.kind === 'code') return '<article class="rich-block"><pre class="block-code" data-kind="code" data-lang="' + escapeHtml(block.lang || '') + '"><code class="block-editable block-code-input" contenteditable="true" spellcheck="false">' + textHtml(block.lines) + '</code></pre></article>';
    var tag = block.kind === 'ordered' ? 'ol' : 'ul';
    return '<article class="rich-block"><' + tag + ' class="block-list" data-kind="' + block.kind + '">' + block.lines.map(function (line) { return '<li class="block-item" contenteditable="true" spellcheck="true">' + escapeHtml(line) + '</li>'; }).join('') + '</' + tag + '></article>';
}

function serializeRich(root) {
    return Array.from(root.querySelectorAll('.rich-block')).map(function (block) {
        var editable = block.querySelector('[data-kind="heading"], [data-kind="paragraph"], [data-kind="quote"]');
        if (editable) {
            var kind = editable.dataset.kind;
            var text = editable.innerText.replace(/\u00a0/g, ' ').replace(/\n+$/, '');
            if (kind === 'heading') return '#'.repeat(Number(editable.dataset.level || 1)) + ' ' + text;
            if (kind === 'quote') return text.split('\n').filter(Boolean).map(function (line) { return '> ' + line; }).join('\n');
            return text.trim() ? text : '';
        }
        var code = block.querySelector('.block-code');
        if (code) {
            var lang = code.dataset.lang || '';
            return '```' + lang + '\n' + code.innerText.replace(/\u00a0/g, ' ').replace(/\n+$/, '') + '\n```';
        }
        var list = block.querySelector('.block-list');
        var lines = Array.from(list ? list.querySelectorAll('.block-item') : []).map(function (item, index) {
            var text = item.innerText.replace(/\u00a0/g, ' ').trim();
            if (!text) return '';
            return (list.dataset.kind === 'ordered' ? (index + 1) + '. ' : '- ') + text;
        }).filter(Boolean);
        return lines.join('\n');
    }).filter(Boolean).join('\n\n');
}

function addBlock(root, kind) {
    var block = { kind: kind, lines: [''] };
    root.insertAdjacentHTML('beforeend', blockHtml(block));
    root.querySelector('.rich-block:last-child [contenteditable="true"]')?.focus();
}

window.richMarkdown = { addBlock: addBlock, blockHtml: blockHtml, parseBlocks: parseBlocks, serializeRich: serializeRich };
