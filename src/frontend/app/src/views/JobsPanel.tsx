// Jobs panel per web-app.md: export/backup/automation progress.
// Lists automation runs and their status.

import { useCallback, useEffect, useState } from 'react';
import { api } from '../api';
import type { AutomationRun } from '../types';

interface Props {
  workspaceId: string | null;
}

export default function JobsPanel({ workspaceId }: Props) {
  const [runs, setRuns] = useState<AutomationRun[]>([]);
  const [loading, setLoading] = useState(false);

  const load = useCallback(async () => {
    if (!workspaceId) return;
    setLoading(true);
    try {
      // List recent automation runs
      const data = await api.get<AutomationRun[]>(
        `/api/workspaces/${workspaceId}/automation/runs`,
      );
      setRuns(data);
    } catch {
      // Non-critical; jobs may not exist yet
    } finally {
      setLoading(false);
    }
  }, [workspaceId]);

  useEffect(() => {
    void load();
  }, [load]);

  if (!workspaceId) {
    return (
      <div className="jobs-panel">
        <p className="jobs-empty">Select a workspace to view jobs.</p>
      </div>
    );
  }

  return (
    <div className="jobs-panel">
      <h2>Jobs</h2>
      {loading && <p>Loading…</p>}
      {!loading && runs.length === 0 && (
        <p className="jobs-empty">No automation runs yet.</p>
      )}
      <ul className="jobs-list">
        {runs.map((run) => (
          <li key={run.id} className={`job-item job-${run.status}`}>
            <span className="job-id">{run.id.slice(0, 8)}</span>
            <span className="job-status">{run.status}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}
