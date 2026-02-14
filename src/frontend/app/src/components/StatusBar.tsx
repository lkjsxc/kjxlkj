// UX-FEEDBACK-01: save/sync/conflict/offline status — visible but low-noise.

import type { SaveStatus } from '../hooks/useAutosave';

interface Props {
  status: SaveStatus;
}

const STATUS_TEXT: Record<SaveStatus, string> = {
  idle: '',
  saving: 'Saving…',
  saved: 'Saved',
  conflict: 'Conflict',
  offline: 'Offline',
  error: 'Save error',
};

export default function StatusBar({ status }: Props) {
  if (status === 'idle') return null;

  return (
    <div
      className={`status-bar status-${status}`}
      role="status"
      aria-live="polite"
    >
      {STATUS_TEXT[status]}
    </div>
  );
}
