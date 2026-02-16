/**
 * Markdown utilities per /docs/spec/ui/editor-flow.md
 *
 * Implements:
 * - Wiki-link detection ([[note]])
 * - Markdown preview rendering (simple HTML conversion)
 * - Keyboard shortcut handling
 */

/** Regular expression for wiki-links: [[target]] or [[target|display]] */
const WIKI_LINK_RE = /\[\[([^\]|]+)(?:\|([^\]]+))?\]\]/g;

/** Extracted wiki-link */
export interface WikiLink {
  readonly target: string;
  readonly display: string;
  readonly start: number;
  readonly end: number;
}

/** Extract all wiki-links from markdown text */
export function extractWikiLinks(markdown: string): ReadonlyArray<WikiLink> {
  const links: WikiLink[] = [];
  let match: RegExpExecArray | null;
  const re = new RegExp(WIKI_LINK_RE.source, 'g');
  while ((match = re.exec(markdown)) !== null) {
    const target = match[1];
    const display = match[2];
    if (target === undefined) continue;
    links.push({
      target: target.trim(),
      display: (display ?? target).trim(),
      start: match.index,
      end: match.index + match[0].length,
    });
  }
  return links;
}

/**
 * Simple markdown to HTML conversion for preview mode.
 * Per /docs/spec/ui/editor-flow.md: raw markdown fidelity preserved.
 * This is a minimal implementation — NOT a full CommonMark parser.
 */
export function markdownToHtml(markdown: string): string {
  return markdown
    .split('\n')
    .map(convertLine)
    .join('\n');
}

function convertLine(line: string): string {
  // Headings
  const headingMatch = /^(#{1,6})\s+(.+)$/.exec(line);
  if (headingMatch) {
    const hashes = headingMatch[1];
    const text = headingMatch[2];
    if (hashes !== undefined && text !== undefined) {
      const level = hashes.length;
      return `<h${level}>${escapeHtml(text)}</h${level}>`;
    }
  }
  // Code fence toggle (simplified — just mark it)
  if (line.startsWith('```')) {
    return `<pre><code>${escapeHtml(line.slice(3))}</code></pre>`;
  }
  // Blockquote
  if (line.startsWith('> ')) {
    return `<blockquote>${escapeHtml(line.slice(2))}</blockquote>`;
  }
  // Unordered list
  if (/^[-*+]\s/.test(line)) {
    return `<li>${escapeHtml(line.slice(2))}</li>`;
  }
  // Wiki-links: [[target]] → linked text
  const withLinks = line.replace(WIKI_LINK_RE, (_m, target: string, display?: string) => {
    const text = (display ?? target).trim();
    return `<a class="wiki-link" data-target="${escapeHtml(target.trim())}">${escapeHtml(text)}</a>`;
  });
  // Empty line
  if (withLinks.trim() === '') return '<br>';
  return `<p>${withLinks}</p>`;
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

/** Keyboard shortcut action */
export type ShortcutAction =
  | 'heading1' | 'heading2' | 'heading3'
  | 'bold' | 'italic' | 'code'
  | 'list_bullet' | 'list_ordered'
  | 'code_fence' | 'blockquote'
  | 'wiki_link' | 'save';

/**
 * Map keyboard event to shortcut action.
 * Per /docs/spec/ui/editor-flow.md: keyboard-centric command patterns.
 */
export function mapKeyboardShortcut(e: KeyboardEvent): ShortcutAction | null {
  const ctrl = e.ctrlKey || e.metaKey;
  if (!ctrl) return null;
  switch (e.key) {
    case '1': return 'heading1';
    case '2': return 'heading2';
    case '3': return 'heading3';
    case 'b': return 'bold';
    case 'i': return 'italic';
    case 'e': return 'code';
    case 'k': return 'wiki_link';
    case 's': return 'save';
    default: return null;
  }
}

/**
 * Apply a markdown formatting action to selected text.
 * Returns the new text and cursor position.
 */
export function applyFormatting(
  text: string,
  selStart: number,
  selEnd: number,
  action: ShortcutAction,
): { text: string; cursorStart: number; cursorEnd: number } {
  const before = text.slice(0, selStart);
  const selected = text.slice(selStart, selEnd);
  const after = text.slice(selEnd);
  let insert: string;
  let cStart: number;
  let cEnd: number;
  switch (action) {
    case 'bold':
      insert = `**${selected || 'text'}**`;
      cStart = selStart + 2;
      cEnd = selStart + insert.length - 2;
      break;
    case 'italic':
      insert = `*${selected || 'text'}*`;
      cStart = selStart + 1;
      cEnd = selStart + insert.length - 1;
      break;
    case 'code':
      insert = `\`${selected || 'code'}\``;
      cStart = selStart + 1;
      cEnd = selStart + insert.length - 1;
      break;
    case 'wiki_link':
      insert = `[[${selected || 'note'}]]`;
      cStart = selStart + 2;
      cEnd = selStart + insert.length - 2;
      break;
    case 'heading1':
      insert = `# ${selected || 'Heading'}`;
      cStart = selStart + 2;
      cEnd = selStart + insert.length;
      break;
    case 'heading2':
      insert = `## ${selected || 'Heading'}`;
      cStart = selStart + 3;
      cEnd = selStart + insert.length;
      break;
    case 'heading3':
      insert = `### ${selected || 'Heading'}`;
      cStart = selStart + 4;
      cEnd = selStart + insert.length;
      break;
    case 'code_fence':
      insert = `\`\`\`\n${selected || ''}\n\`\`\``;
      cStart = selStart + 4;
      cEnd = selStart + 4 + (selected || '').length;
      break;
    case 'blockquote':
      insert = `> ${selected || 'quote'}`;
      cStart = selStart + 2;
      cEnd = selStart + insert.length;
      break;
    default:
      return { text, cursorStart: selStart, cursorEnd: selEnd };
  }
  return {
    text: before + insert + after,
    cursorStart: cStart,
    cursorEnd: cEnd,
  };
}
