export type ShellElements = {
  status: HTMLElement;
  notesList: HTMLUListElement;
  titleInput: HTMLInputElement;
  editor: HTMLTextAreaElement;
};

export function renderShell(root: HTMLElement): ShellElements {
  root.innerHTML = "";

  const status = document.createElement("div");
  status.setAttribute("aria-live", "polite");

  const notesList = document.createElement("ul");
  const titleInput = document.createElement("input");
  titleInput.setAttribute("aria-label", "Note title");

  const editor = document.createElement("textarea");
  editor.setAttribute("aria-label", "Note editor");

  root.append(status, notesList, titleInput, editor);
  return { status, notesList, titleInput, editor };
}
