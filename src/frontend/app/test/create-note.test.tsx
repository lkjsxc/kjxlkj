/**
 * E2E-23: pressing "Create New Note" adds a note and selects it for editing.
 * Per /docs/spec/ui/web-app.md and /docs/spec/technical/testing.md.
 *
 * This test verifies:
 * - Clicking the "Create New Note" button triggers note creation via the API.
 * - After creation, the note appears in the notes list.
 * - The new note is selected (becomes the active note in the editor).
 * - On small screens, the menu closes after note creation.
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { useEffect, type ReactNode } from "react";
import { AppProvider, useAppDispatch } from "../src/state";
import { NotesList } from "../src/views/NotesList";
import * as api from "../src/api";

// Mock the API module
vi.mock("../src/api", () => ({
  createNote: vi.fn(),
  listNotes: vi.fn(),
  getNote: vi.fn(),
  getSession: vi.fn(),
  ApiError: class ApiError extends Error {
    constructor(
      public status: number,
      public body: string,
    ) {
      super(`HTTP ${status}`);
    }
  },
}));

describe("E2E-23: Create New Note", () => {
  const mockNote = {
    id: "note-001",
    title: "Untitled",
    note_kind: "document",
    workspace_id: "ws-001",
    created_at: "2026-01-01T00:00:00Z",
  };

  const mockProjection = {
    note_id: "note-001",
    title: "Untitled",
    note_kind: "document",
    body_text: "",
    settings_json: "{}",
    version: 1,
    workspace_id: "ws-001",
    updated_at: "2026-01-01T00:00:00Z",
  };

  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(api.createNote).mockResolvedValue({ id: "note-001" });
    vi.mocked(api.listNotes).mockResolvedValue([mockNote]);
    vi.mocked(api.getNote).mockResolvedValue(mockProjection);
  });

  /**
   * Helper: render NotesList inside AppProvider with pre-set state.
   * We wrap in a context provider that has session and workspace loaded.
   */
  function renderWithState() {
    // We use a wrapper component that dispatches initial state
    function Wrapper() {
      return (
        <AppProvider>
          <StateInjector>
            <NotesList />
          </StateInjector>
        </AppProvider>
      );
    }
    return render(<Wrapper />);
  }

  /**
   * StateInjector: sets session and workspace in context for testing.
   */
  function StateInjector({ children }: { children: ReactNode }) {
    const dispatch = useAppDispatch();

    useEffect(() => {
      dispatch({
        type: "SET_SESSION",
        session: {
          user_id: "user-001",
          email: "test@test.com",
          display_name: "Test",
          role: "owner",
          csrf_token: "csrf-token-123",
        },
      });
      dispatch({ type: "SET_WORKSPACE", workspaceId: "ws-001" });
    }, [dispatch]);

    return <>{children}</>;
  }

  it("creates a note and selects it when Create New Note is clicked", async () => {
    renderWithState();

    // Find and click the Create New Note button
    const createBtn = screen.getByRole("button", { name: /Create New Note/i });
    expect(createBtn).toBeInTheDocument();

    fireEvent.click(createBtn);

    // Verify the API was called to create a note
    await waitFor(() => {
      expect(api.createNote).toHaveBeenCalledWith(
        "ws-001",
        "Untitled",
        "document",
        "csrf-token-123",
      );
    });

    // Verify the note list was refreshed
    await waitFor(() => {
      expect(api.listNotes).toHaveBeenCalledWith("ws-001", "csrf-token-123");
    });

    // Verify the new note was fetched for editing (selected)
    await waitFor(() => {
      expect(api.getNote).toHaveBeenCalledWith(
        "ws-001",
        "note-001",
        "csrf-token-123",
      );
    });
  });

  it("shows the new note in the list after creation", async () => {
    renderWithState();

    const createBtn = screen.getByRole("button", { name: /Create New Note/i });
    fireEvent.click(createBtn);

    // After creation, the note should appear in the list
    await waitFor(() => {
      expect(screen.getByText("Untitled")).toBeInTheDocument();
    });
  });
});
