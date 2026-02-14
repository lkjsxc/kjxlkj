export type EditorStatus = "saving" | "saved" | "conflict" | "offline";

export function editorStatusLabel(status: EditorStatus): string {
  switch (status) {
    case "saving":
      return "saving";
    case "saved":
      return "saved";
    case "conflict":
      return "conflict";
    default:
      return "offline";
  }
}
