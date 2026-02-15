/**
 * NoteDetail: markdown-native editor with title, body, status.
 * Spec: UX-EDIT-01 — synced/draft split.
 * Spec: UX-EDIT-05 — default chrome omits save/version/delete.
 * Spec: UX-FEEDBACK-01 — status visible but low-noise.
 */
import { useEffect } from "react";
import { useEditor } from "../hooks/useEditor";
import { StatusBar } from "../components/StatusBar";

interface Props {
  noteId: string;
}

export function NoteDetail({ noteId }: Props) {
  const {
    noteId: loadedId,
    draftTitle,
    draftBody,
    status,
    dirty,
    loadNote,
    setTitle,
    setBody,
    reloadLatest,
  } = useEditor();

  useEffect(() => {
    if (noteId !== loadedId) {
      void loadNote(noteId);
    }
  }, [noteId, loadedId, loadNote]);

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <input
          type="text"
          value={draftTitle}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Untitled"
          style={styles.titleInput}
          aria-label="Note title"
        />
        <StatusBar status={status} dirty={dirty} onReload={reloadLatest} />
      </div>
      <textarea
        value={draftBody}
        onChange={(e) => setBody(e.target.value)}
        placeholder="Start writing…"
        style={styles.editor}
        aria-label="Note body"
      />
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  container: {
    display: "flex",
    flexDirection: "column",
    height: "100%",
    padding: "0.75rem",
  },
  header: {
    display: "flex",
    alignItems: "center",
    gap: "0.5rem",
    marginBottom: "0.5rem",
  },
  titleInput: {
    flex: 1,
    fontSize: "1.25rem",
    fontWeight: 600,
    border: "none",
    outline: "none",
    padding: "0.25rem 0",
    background: "transparent",
  },
  editor: {
    flex: 1,
    resize: "none",
    border: "none",
    outline: "none",
    fontSize: "1rem",
    lineHeight: 1.6,
    fontFamily: "system-ui, -apple-system, sans-serif",
    padding: 0,
  },
};
