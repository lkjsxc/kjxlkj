import { createNote, getSession, listNotes, patchNote, patchTitle } from "./api/http-client";
import { noteEditorDefault, type NoteEditorState } from "./state/notes";
import { editorStatusLabel } from "./ui/editor";
import { renderShell, type ShellElements } from "./ui/shell";

export type AppMode = "setup" | "login" | "workspace";

export const defaultMode: AppMode = "login";

export type AppApi = {
	getSession: typeof getSession;
	listNotes: typeof listNotes;
	createNote: typeof createNote;
	patchNote: typeof patchNote;
	patchTitle: typeof patchTitle;
};

const liveApi: AppApi = {
	getSession,
	listNotes,
	createNote,
	patchNote,
	patchTitle,
};

export type BootResult = {
	shell: ShellElements;
	state: NoteEditorState;
};

export function nextIdempotencyKey(
	maybeCrypto: Pick<Crypto, "randomUUID"> | null | undefined = globalThis.crypto,
	now: () => number = Date.now,
): string {
	if (maybeCrypto && typeof maybeCrypto.randomUUID === "function") {
		return maybeCrypto.randomUUID();
	}
	return `fallback-${now()}`;
}

function setStatus(state: NoteEditorState, element: HTMLElement, status: NoteEditorState["saveStatus"]): void {
	state.saveStatus = status;
	element.textContent = editorStatusLabel(status);
}

export async function bootstrapApp(root: HTMLElement, api: AppApi = liveApi): Promise<BootResult> {
	const shell = renderShell(root);
	const state: NoteEditorState = structuredClone(noteEditorDefault);

	let csrfToken = "";
	try {
		const session = await api.getSession();
		csrfToken = session.csrf_token;
	} catch {
		shell.status.textContent = "login required";
		return { shell, state };
	}

	const notes = await api.listNotes();
	const currentNote = notes[0] ?? (await api.createNote(csrfToken));
	state.synced = {
		noteId: currentNote.id,
		title: currentNote.title,
		markdown: currentNote.markdown,
		version: currentNote.current_version,
	};
	state.draft = {
		title: currentNote.title,
		markdown: currentNote.markdown,
	};

	const listItem = document.createElement("li");
	listItem.textContent = state.draft.title;
	shell.notesList.append(listItem);
	shell.titleInput.value = state.draft.title;
	shell.editor.value = state.draft.markdown;
	setStatus(state, shell.status, "saved");

	let autosaveTimer: number | undefined;
	const scheduleSave = (): void => {
		if (autosaveTimer !== undefined) {
			window.clearTimeout(autosaveTimer);
		}
		autosaveTimer = window.setTimeout(async () => {
			setStatus(state, shell.status, "saving");
			try {
				const patched = await api.patchNote(
					state.synced.noteId,
					state.synced.version,
					state.synced.markdown,
					state.draft.markdown,
					csrfToken,
					nextIdempotencyKey(),
				);
				state.synced.markdown = patched.markdown;
				state.synced.version = patched.current_version;
				setStatus(state, shell.status, "saved");
			} catch (error) {
				const message = error instanceof Error ? error.message : "unknown";
				if (message.includes("409")) {
					setStatus(state, shell.status, "conflict");
				} else {
					setStatus(state, shell.status, "offline");
				}
			}
		}, 400);
	};

	shell.titleInput.addEventListener("input", async () => {
		state.draft.title = shell.titleInput.value;
		listItem.textContent = state.draft.title;
		try {
			const patched = await api.patchTitle(
				state.synced.noteId,
				state.synced.version,
				state.draft.title,
				csrfToken,
			);
			state.synced.title = patched.title;
			state.synced.version = patched.current_version;
			setStatus(state, shell.status, "saved");
		} catch {
			setStatus(state, shell.status, "conflict");
		}
	});

	shell.editor.addEventListener("input", () => {
		state.draft.markdown = shell.editor.value;
		scheduleSave();
	});

	return { shell, state };
}
