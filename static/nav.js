// kjxlkj — Navigation panel
'use strict';

const NavUI = {
  refresh() {
    const noteList = document.getElementById('note-list');
    const wsList = document.getElementById('ws-list');
    if (!noteList || !wsList) return;

    // Render note items
    noteList.innerHTML = App.state.notes.map(n => `
      <div class="item${App.state.currentNote && App.state.currentNote.id === n.id ? ' active' : ''}"
           data-note-id="${n.id}" role="button" tabindex="0"
           aria-label="Open note ${esc(n.title || 'Untitled')}">${esc(n.title || 'Untitled')}</div>
    `).join('');

    // Bind click handlers
    noteList.querySelectorAll('.item').forEach(el => {
      el.onclick = () => EditorUI.openNote(el.dataset.noteId);
      el.onkeydown = (e) => { if (e.key === 'Enter') EditorUI.openNote(el.dataset.noteId); };
    });

    // Render workspace items
    wsList.innerHTML = App.state.workspaces.map(w => `
      <div class="item${App.state.currentWorkspace && App.state.currentWorkspace.id === w.id ? ' active' : ''}"
           data-ws-id="${w.id}" role="button" tabindex="0">${esc(w.name)}</div>
    `).join('');

    wsList.querySelectorAll('.item').forEach(el => {
      el.onclick = async () => {
        const ws = App.state.workspaces.find(w => w.id === el.dataset.wsId);
        if (ws) {
          App.state.currentWorkspace = ws;
          await App.loadNotes();
          NavUI.refresh();
        }
      };
    });
  },
};
