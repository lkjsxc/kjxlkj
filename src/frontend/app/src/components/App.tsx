/**
 * App: Root component implementing responsive split layout.
 * Per /docs/spec/ui/layout-and-interaction.md:
 * - One responsive component tree across desktop and mobile.
 * - Desktop (>=1024px): notes list on left, editor on right.
 * - Compact (<1024px): editor is primary; menu toggle in top-right
 *   collapses/restores navigation. Selecting a note closes the menu.
 * - Independent pane scrolling.
 * - 320px minimum width with no horizontal scrolling.
 */
import { useEffect, useState } from "react";
import { useAppState, useAppDispatch } from "../state";
import { getSession, listNotes, ApiError } from "../api";
import { SetupView } from "../views/SetupView";
import { LoginView } from "../views/LoginView";
import { NotesList } from "../views/NotesList";
import { NoteDetail } from "../views/NoteDetail";
import { JobsPanel } from "../views/JobsPanel";

export function App() {
  const state = useAppState();
  const dispatch = useAppDispatch();
  const [setupAvailable, setSetupAvailable] = useState<boolean | null>(null);

  // Session probe per /docs/spec/ui/web-app.md
  useEffect(() => {
    (async () => {
      try {
        const session = await getSession();
        dispatch({ type: "SET_SESSION", session });
      } catch (err) {
        if (err instanceof ApiError && err.status === 401) {
          // Expected unauthenticated state — not fatal (USR-001)
          dispatch({ type: "SET_SESSION", session: undefined });
          // Check if setup is available
          try {
            const res = await fetch("/api/auth/register", { method: "HEAD" });
            setSetupAvailable(res.status !== 409);
          } catch {
            setSetupAvailable(false);
          }
        } else {
          dispatch({ type: "SET_SESSION", session: undefined });
          setSetupAvailable(false);
        }
      }
    })();
  }, [dispatch]);

  // Load notes when session and workspace are available
  useEffect(() => {
    if (!state.session || !state.workspaceId) return;
    (async () => {
      try {
        const notes = await listNotes(
          state.workspaceId!,
          state.session!.csrf_token,
        );
        dispatch({ type: "SET_NOTES", notes });
      } catch {
        // handle error
      }
    })();
  }, [state.session, state.workspaceId, dispatch]);

  // Loading state
  if (state.loading) {
    return <div className="loading">Loading…</div>;
  }

  // Unauthenticated
  if (state.session === undefined) {
    if (setupAvailable) {
      return (
        <SetupView onLocked={() => setSetupAvailable(false)} />
      );
    }
    return <LoginView />;
  }

  // Authenticated — main layout
  return (
    <div className="app-layout">
      {/* Top-right menu button for compact screens */}
      <button
        className="menu-toggle"
        onClick={() => dispatch({ type: "TOGGLE_MENU" })}
        aria-label="Toggle navigation menu"
        aria-expanded={state.menuOpen}
      >
        ☰
      </button>

      <div className={`sidebar ${state.menuOpen ? "sidebar-open" : ""}`}>
        <NotesList />
      </div>

      <div className="main-content">
        <NoteDetail />
      </div>

      <JobsPanel />
    </div>
  );
}
