// kjxlkj — Shell layout (header + nav + editor area)
// UX-LAYOUT-01: single responsive component tree
'use strict';

const ShellUI = {
  render(state) {
    const user = state.user || {};
    return `
    <div class="shell">
      <header class="header" role="banner">
        <button class="menu-toggle" id="menu-toggle" aria-label="Toggle navigation">☰</button>
        <span class="logo">kjxlkj</span>
        <span class="spacer"></span>
        <span class="status" id="sync-status"></span>
        <span class="user-badge" role="status">${esc(user.username || '')}</span>
        <button id="logout-btn" style="font-size:.8rem">Logout</button>
      </header>
      <div class="body-wrap">
        <nav class="nav${state.navOpen ? '' : ' collapsed'}" id="nav-panel" role="navigation" aria-label="Notes">
          <div class="section">Notes</div>
          <button class="create-btn" id="create-note-btn">+ New Note</button>
          <div id="note-list"></div>
          <div class="section" style="margin-top:auto">Workspaces</div>
          <div id="ws-list"></div>
        </nav>
        <main class="editor-area" id="editor-area" role="main">
          <div id="editor-content"></div>
        </main>
      </div>
    </div>
    ${CmdPalette.renderContainer()}
    ${ConflictDialog.renderContainer()}`;
  },

  bind() {
    const toggle = document.getElementById('menu-toggle');
    if (toggle) toggle.onclick = () => {
      App.state.navOpen = !App.state.navOpen;
      document.getElementById('nav-panel').classList.toggle('collapsed');
    };
    const logout = document.getElementById('logout-btn');
    if (logout) logout.onclick = async () => {
      await fetch('/api/auth/logout', {method:'POST'});
      App.state.user = null;
      App.render();
    };
    const createBtn = document.getElementById('create-note-btn');
    if (createBtn) createBtn.onclick = () => EditorUI.createNote();
    NavUI.refresh();
    EditorUI.renderEmpty();
  },
};

// Escape HTML helper
function esc(s) {
  const d = document.createElement('div');
  d.textContent = s;
  return d.innerHTML;
}
