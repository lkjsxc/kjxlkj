import { beforeEach, describe, expect, it, vi } from "vitest";

import { bootstrapApp, nextIdempotencyKey, type AppApi } from "../src/app";
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

describe("regression pack frontend", () => {
  it("REG-USR-002 uses fallback idempotency key without randomUUID", () => {
    const key = nextIdempotencyKey(null, () => 12345);
    expect(key).toBe("fallback-12345");
  });

  it("REG-USR-001 keeps pre-auth session probe non-fatal", async () => {
    const root = document.getElementById("app");
    if (!root) {
      throw new Error("missing app root");
    }

    await bootstrapApp(
      root,
      defaultApi({
        getSession: async () => {
          throw new Error("HTTP 401");
        },
      }),
    );

    const status = root.querySelector("div");
    expect(status?.textContent).toBe("login required");
  });

  it("REG-USR-007 propagates title rename to list in same cycle", async () => {
    const root = document.getElementById("app");
    if (!root) {
      throw new Error("missing app root");
    }

    const { shell } = await bootstrapApp(root, defaultApi());

    shell.titleInput.value = "Renamed";
    shell.titleInput.dispatchEvent(new Event("input", { bubbles: true }));

    expect(shell.notesList.firstElementChild?.textContent).toBe("Renamed");
  });

  it("REG-UX-005 surfaces conflict status on 409 autosave failure", async () => {
    vi.useFakeTimers();
    const root = document.getElementById("app");
    if (!root) {
      throw new Error("missing app root");
    }

    const { shell } = await bootstrapApp(
      root,
      defaultApi({
        patchNote: async () => {
          throw new Error("HTTP 409");
        },
      }),
    );

    shell.editor.value = "changed";
    shell.editor.dispatchEvent(new Event("input", { bubbles: true }));
    await vi.advanceTimersByTimeAsync(450);

    expect(shell.status.textContent).toBe("conflict");
  });

  it("REG-UX-005 surfaces offline status on non-409 autosave failure", async () => {
    vi.useFakeTimers();
    const root = document.getElementById("app");
    if (!root) {
      throw new Error("missing app root");
    }

    const { shell } = await bootstrapApp(
      root,
      defaultApi({
        patchNote: async () => {
          throw new Error("HTTP 500");
        },
      }),
    );

    shell.editor.value = "changed";
    shell.editor.dispatchEvent(new Event("input", { bubbles: true }));
    await vi.advanceTimersByTimeAsync(450);

    expect(shell.status.textContent).toBe("offline");
  });
});
