// Command palette per UX-NAV-02
// Keyboard-first access to: create/open/move/tag/run-rule actions.
// Opens with Ctrl+K / Cmd+K, deterministic feedback.

import { useCallback, useEffect, useRef, useState } from 'react';

export interface CommandAction {
  id: string;
  label: string;
  shortcut?: string;
  handler: () => void | Promise<void>;
}

interface Props {
  actions: CommandAction[];
}

export default function CommandPalette({ actions }: Props) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [feedback, setFeedback] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Global keyboard shortcut: Ctrl+K / Cmd+K
  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setOpen((prev) => !prev);
      }
      if (e.key === 'Escape' && open) {
        setOpen(false);
      }
    }
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [open]);

  // Focus input when opened
  useEffect(() => {
    if (open) {
      setQuery('');
      setSelectedIndex(0);
      setFeedback(null);
      setTimeout(() => inputRef.current?.focus(), 0);
    }
  }, [open]);

  // Filter actions by query
  const filtered = actions.filter((a) =>
    a.label.toLowerCase().includes(query.toLowerCase()),
  );

  // Execute selected action with deterministic feedback
  const executeAction = useCallback(
    async (action: CommandAction) => {
      try {
        await action.handler();
        setFeedback(`Done: ${action.label}`);
        setTimeout(() => setOpen(false), 400);
      } catch {
        setFeedback(`Failed: ${action.label}`);
      }
    },
    [],
  );

  // Keyboard navigation within palette
  function handlePaletteKeyDown(e: React.KeyboardEvent) {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((i) => Math.min(i + 1, filtered.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((i) => Math.max(i - 1, 0));
    } else if (e.key === 'Enter' && filtered[selectedIndex]) {
      e.preventDefault();
      void executeAction(filtered[selectedIndex]);
    }
  }

  if (!open) return null;

  return (
    <div
      className="command-palette-overlay"
      role="dialog"
      aria-label="Command palette"
      onClick={() => setOpen(false)}
    >
      <div
        className="command-palette"
        onClick={(e) => e.stopPropagation()}
        onKeyDown={handlePaletteKeyDown}
      >
        <input
          ref={inputRef}
          type="text"
          className="command-palette-input"
          placeholder="Type a command…"
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setSelectedIndex(0);
          }}
          aria-label="Command search"
          role="combobox"
          aria-expanded="true"
          aria-controls="command-list"
          aria-activedescendant={
            filtered[selectedIndex]
              ? `cmd-${filtered[selectedIndex].id}`
              : undefined
          }
        />
        <ul
          id="command-list"
          className="command-palette-list"
          role="listbox"
        >
          {filtered.map((action, i) => (
            <li
              key={action.id}
              id={`cmd-${action.id}`}
              className={`command-palette-item${i === selectedIndex ? ' selected' : ''}`}
              role="option"
              aria-selected={i === selectedIndex}
              onClick={() => void executeAction(action)}
            >
              <span>{action.label}</span>
              {action.shortcut && (
                <kbd className="command-shortcut">{action.shortcut}</kbd>
              )}
            </li>
          ))}
          {filtered.length === 0 && (
            <li className="command-palette-empty">No matching commands</li>
          )}
        </ul>
        {feedback && (
          <div className="command-palette-feedback" role="status" aria-live="polite">
            {feedback}
          </div>
        )}
      </div>
    </div>
  );
}
