export type SyncedSnapshot = {
  noteId: string;
  title: string;
  markdown: string;
  version: number;
};

export type DraftBuffer = {
  title: string;
  markdown: string;
};

export type NoteEditorState = {
  synced: SyncedSnapshot;
  draft: DraftBuffer;
  saveStatus: "saving" | "saved" | "conflict" | "offline";
};

export const noteEditorDefault: NoteEditorState = {
  synced: {
    noteId: "",
    title: "",
    markdown: "",
    version: 0,
  },
  draft: {
    title: "",
    markdown: "",
  },
  saveStatus: "saved",
};
