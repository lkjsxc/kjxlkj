import { beforeEach, describe, expect, it, vi } from "vitest";

import { bootstrapApp, type AppApi } from "../src/app";
import type { NotePayload, SessionPayload } from "../src/api/http-client";

const session: SessionPayload = {
  user: { id: "u-1", email: "owner@example.com" },
  csrf_token: "csrf-1",
};

const initialNote: NotePayload = {
  id: "n-1",
  title: "Untitled",
  markdown: "hello",
  current_version: 1,
  workspace_id: "ws-1",
};

function defaultApi(overrides: Partial<AppApi> = {}): AppApi {
  return {
    getSession: async () => session,
    listNotes: async () => [initialNote],
    createNote: async () => initialNote,
    patchNote: async (_noteId, _baseVersion, _baseMarkdown, markdown) => ({
      ...initialNote,
      markdown,
      current_version: 2,
    }),
    patchTitle: async (_noteId, _baseVersion, title) => ({
      ...initialNote,
      title,
      current_version: 2,
    }),
    ...overrides,
  };
}

beforeEach(() => {
  document.body.innerHTML = '<div id="app"></div>';
  vi.useRealTimers();
});

describe("regression pack frontend extended", () => {
  it("REG-IMP-001 sends autosave patch using synced markdown as base", async () => {
    vi.useFakeTimers();
    const root = document.getElementById("app");
    if (!root) {
      throw new Error("missing app root");
    }

    const patchNote = vi.fn(async (_noteId, _baseVersion, _baseMarkdown: string, markdown: string) => ({
      ...initialNote,
      markdown,
      current_version: 2,
    }));

    const { shell } = await bootstrapApp(root, defaultApi({ patchNote }));
    shell.editor.value = "hello world";
    shell.editor.dispatchEvent(new Event("input", { bubbles: true }));
    await vi.advanceTimersByTimeAsync(450);

    expect(patchNote).toHaveBeenCalledTimes(1);
    expect(patchNote.mock.calls[0][2]).toBe("hello");
    expect(patchNote.mock.calls[0][3]).toBe("hello world");
  });

  it("REG-USR-003 saves edits via autosave without manual save control", async () => {
    vi.useFakeTimers();
    const root = document.getElementById("app");
    if (!root) {
      throw new Error("missing app root");
    }

    const patchNote = vi.fn(async (_noteId, _baseVersion, _baseMarkdown: string, markdown: string) => ({
      ...initialNote,
      markdown,
      current_version: 2,
    }));

    const { shell } = await bootstrapApp(root, defaultApi({ patchNote }));
    shell.editor.value = "autosaved";
    shell.editor.dispatchEvent(new Event("input", { bubbles: true }));
    await vi.advanceTimersByTimeAsync(450);

    expect(patchNote).toHaveBeenCalledTimes(1);
    expect(shell.status.textContent).toBe("saved");
    expect(root.querySelectorAll("button").length).toBe(0);
  });

  it("REG-USR-008 keeps default editor chrome minimal", async () => {
    const root = document.getElementById("app");
    if (!root) {
      throw new Error("missing app root");
    }

    await bootstrapApp(root, defaultApi());

    expect(root.querySelectorAll("textarea").length).toBe(1);
    expect(root.querySelectorAll("input").length).toBe(1);
    expect(root.querySelectorAll("button").length).toBe(0);
  });
});
