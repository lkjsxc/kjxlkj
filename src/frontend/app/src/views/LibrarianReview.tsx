// Librarian review UX per UX-LIB-01, UX-LIB-02
// Per-operation accept/reject with deterministic audit trace.
// Applying changes MUST preserve unresolved local drafts.

import { useCallback, useEffect, useState } from 'react';
import { api } from '../api';
import type { RunDetail, LibrarianOperation } from '../types';

interface Props {
  runId: string;
  onClose: () => void;
}

export default function LibrarianReview({ runId, onClose }: Props) {
  const [detail, setDetail] = useState<RunDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [decisions, setDecisions] = useState<Map<string, 'accept' | 'reject'>>(
    () => new Map(),
  );
  const [submitting, setSubmitting] = useState(false);
  const [feedback, setFeedback] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    api
      .get<RunDetail>(`/api/automation/runs/${runId}`)
      .then((d) => {
        setDetail(d);
        setLoading(false);
      })
      .catch(() => setLoading(false));
  }, [runId]);

  const setDecision = useCallback(
    (opId: string, decision: 'accept' | 'reject') => {
      setDecisions((prev) => {
        const next = new Map(prev);
        next.set(opId, decision);
        return next;
      });
    },
    [],
  );

  const handleSubmitReview = useCallback(async () => {
    if (!detail) return;
    setSubmitting(true);
    setFeedback(null);
    try {
      const ops = detail.operations.map((op) => ({
        operation_id: op.id,
        decision: decisions.get(op.id) ?? 'reject',
      }));
      await api.post(`/api/automation/runs/${runId}/review`, {
        decisions: ops,
      });
      setFeedback('Review submitted successfully.');
      setTimeout(() => onClose(), 800);
    } catch {
      setFeedback('Failed to submit review.');
    } finally {
      setSubmitting(false);
    }
  }, [detail, decisions, runId, onClose]);

  // Accept or reject all operations at once
  function handleBulk(decision: 'accept' | 'reject') {
    if (!detail) return;
    const next = new Map<string, 'accept' | 'reject'>();
    for (const op of detail.operations) {
      next.set(op.id, decision);
    }
    setDecisions(next);
  }

  if (loading) {
    return (
      <div className="librarian-review" role="region" aria-label="Librarian review">
        <p>Loading run details…</p>
      </div>
    );
  }

  if (!detail) {
    return (
      <div className="librarian-review" role="region" aria-label="Librarian review">
        <p>Run not found.</p>
        <button type="button" onClick={onClose}>Close</button>
      </div>
    );
  }

  return (
    <div className="librarian-review" role="region" aria-label="Librarian review">
      <div className="librarian-review-header">
        <h2>Librarian Run Review</h2>
        <span className={`run-status run-status-${detail.run.status}`}>
          {detail.run.status}
        </span>
        <button type="button" className="secondary" onClick={onClose}>
          Close
        </button>
      </div>

      <div className="librarian-review-bulk">
        <button type="button" onClick={() => handleBulk('accept')}>
          Accept All
        </button>
        <button
          type="button"
          className="danger"
          onClick={() => handleBulk('reject')}
        >
          Reject All
        </button>
      </div>

      <ul className="librarian-operations" role="list">
        {detail.operations.map((op) => (
          <OperationItem
            key={op.id}
            op={op}
            decision={decisions.get(op.id)}
            onDecide={(d) => setDecision(op.id, d)}
          />
        ))}
      </ul>

      {feedback && (
        <div role="status" aria-live="polite" className="librarian-feedback">
          {feedback}
        </div>
      )}

      <div className="librarian-review-actions">
        <button
          type="button"
          disabled={submitting || decisions.size < detail.operations.length}
          onClick={handleSubmitReview}
        >
          {submitting ? 'Submitting…' : 'Submit Review'}
        </button>
      </div>
    </div>
  );
}

// Individual operation review item
interface OpItemProps {
  op: LibrarianOperation;
  decision?: 'accept' | 'reject';
  onDecide: (d: 'accept' | 'reject') => void;
}

function OperationItem({ op, decision, onDecide }: OpItemProps) {
  return (
    <li className="librarian-op-item" role="listitem">
      <div className="op-header">
        <strong className="op-kind">{op.kind}</strong>
        {op.confidence != null && (
          <span className="op-confidence">
            {(op.confidence * 100).toFixed(0)}% confidence
          </span>
        )}
      </div>
      {op.title && <div className="op-title">Title: {op.title}</div>}
      {op.reason && <div className="op-reason">Reason: {op.reason}</div>}
      {op.body_markdown && (
        <details className="op-body-details">
          <summary>Preview markdown</summary>
          <pre className="op-body-pre">{op.body_markdown}</pre>
        </details>
      )}
      <div className="op-decision-btns" role="group" aria-label="Operation decision">
        <button
          type="button"
          className={decision === 'accept' ? 'op-btn-active' : 'secondary'}
          onClick={() => onDecide('accept')}
          aria-pressed={decision === 'accept'}
        >
          Accept
        </button>
        <button
          type="button"
          className={decision === 'reject' ? 'op-btn-active danger' : 'secondary'}
          onClick={() => onDecide('reject')}
          aria-pressed={decision === 'reject'}
        >
          Reject
        </button>
      </div>
    </li>
  );
}
